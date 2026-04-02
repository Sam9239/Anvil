use runtime::TokenUsage;
use serde::Deserialize;
use serde_json::Value;

use crate::{
    ChatContent, ChatRole, Provider, ProviderCapabilities, ProviderError, ProviderEvent,
    ProviderKind, ProviderRequest, StopReason,
};

/// Google AI Studio / Gemini API base URL.
const GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";

/// Gemini provider adapter using the Google Generative Language API.
pub struct GeminiProvider {
    runtime: tokio::runtime::Runtime,
    http: reqwest::Client,
    api_key: String,
    model: String,
}

impl GeminiProvider {
    /// Create a new Gemini provider.
    ///
    /// # Errors
    ///
    /// Returns an error if the tokio runtime cannot be created.
    pub fn new(api_key: String, model: String) -> Result<Self, ProviderError> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| ProviderError::Other(format!("failed to create tokio runtime: {e}")))?;
        let http = reqwest::Client::new();
        Ok(Self {
            runtime,
            http,
            api_key,
            model,
        })
    }
}

impl Provider for GeminiProvider {
    fn stream_message(
        &mut self,
        request: &ProviderRequest,
    ) -> Result<Vec<ProviderEvent>, ProviderError> {
        let gemini_request = to_gemini_request(request);

        // Gemini uses generateContent (non-streaming) or streamGenerateContent
        let action = if request.stream {
            "streamGenerateContent?alt=sse"
        } else {
            "generateContent"
        };
        let url = format!("{}/models/{}:{}", GEMINI_BASE_URL, self.model, action);

        self.runtime.block_on(async {
            let response = self
                .http
                .post(&url)
                .header("Content-Type", "application/json")
                .query(&[("key", &self.api_key)])
                .json(&gemini_request)
                .send()
                .await
                .map_err(|e| ProviderError::Network(e.to_string()))?;

            let status = response.status().as_u16();
            if status != 200 {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "unknown error".to_string());
                return Err(map_gemini_http_error(status, &body));
            }

            if request.stream {
                parse_gemini_sse(response).await
            } else {
                parse_gemini_response(response).await
            }
        })
    }

    fn capabilities(&self) -> ProviderCapabilities {
        let info = crate::ModelRegistry::find(&self.model);
        ProviderCapabilities {
            max_context_tokens: info.as_ref().map_or(1_000_000, |m| m.max_context_tokens),
            max_output_tokens: info.as_ref().map_or(8_192, |m| m.default_max_output_tokens),
            supports_tools: true,
            supports_vision: true,
            supports_streaming: true,
        }
    }

    fn model_id(&self) -> &str {
        &self.model
    }

    fn provider_kind(&self) -> ProviderKind {
        ProviderKind::Google
    }
}

// --- Gemini API response types ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiResponse {
    #[serde(default)]
    candidates: Vec<GeminiCandidate>,
    #[serde(default)]
    usage_metadata: Option<GeminiUsageMetadata>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiCandidate {
    content: Option<GeminiResponseContent>,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GeminiResponseContent {
    #[serde(default)]
    parts: Vec<GeminiResponsePart>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiResponsePart {
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    function_call: Option<GeminiFunctionCall>,
}

#[derive(Debug, Deserialize)]
struct GeminiFunctionCall {
    name: String,
    #[serde(default)]
    args: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiUsageMetadata {
    #[serde(default)]
    prompt_token_count: u32,
    #[serde(default)]
    candidates_token_count: u32,
}

// --- Conversion ---

fn to_gemini_request(request: &ProviderRequest) -> Value {
    let system_instruction = request.system_prompt.as_ref().map(|prompt| {
        serde_json::json!({
            "parts": [{"text": prompt}]
        })
    });

    let contents: Vec<Value> = request
        .messages
        .iter()
        .filter(|m| m.role != ChatRole::System)
        .map(|msg| {
            let role = match msg.role {
                ChatRole::Assistant => "model",
                // User, Tool, and System all map to "user" in Gemini
                ChatRole::User | ChatRole::Tool | ChatRole::System => "user",
            };

            let parts: Vec<Value> = msg
                .content
                .iter()
                .map(|c| match c {
                    ChatContent::Text(text) => serde_json::json!({"text": text}),
                    ChatContent::ToolUse { name, input, .. } => {
                        serde_json::json!({"functionCall": {"name": name, "args": input}})
                    }
                    ChatContent::ToolResult {
                        content,
                        tool_use_id,
                        ..
                    } => {
                        // Gemini uses the tool name, not the ID, but we may not have it
                        // Use tool_use_id as a fallback name
                        serde_json::json!({
                            "functionResponse": {
                                "name": tool_use_id,
                                "response": {"content": content}
                            }
                        })
                    }
                })
                .collect();

            serde_json::json!({"role": role, "parts": parts})
        })
        .collect();

    let tools = request.tools.as_ref().map(|tools| {
        let declarations: Vec<Value> = tools
            .iter()
            .map(|t| {
                let mut decl = serde_json::json!({
                    "name": t.name,
                });
                if let Some(ref desc) = t.description {
                    decl["description"] = serde_json::json!(desc);
                }
                // Gemini uses "parameters" for input schema
                decl["parameters"] = t.input_schema.clone();
                decl
            })
            .collect();
        vec![serde_json::json!({"functionDeclarations": declarations})]
    });

    let mut req = serde_json::json!({
        "contents": contents,
        "generationConfig": {
            "maxOutputTokens": request.max_tokens
        }
    });

    if let Some(si) = system_instruction {
        req["systemInstruction"] = si;
    }
    if let Some(tools) = tools {
        req["tools"] = serde_json::json!(tools);
    }

    req
}

async fn parse_gemini_response(
    response: reqwest::Response,
) -> Result<Vec<ProviderEvent>, ProviderError> {
    let body: GeminiResponse = response
        .json()
        .await
        .map_err(|e| ProviderError::Parse(e.to_string()))?;

    let mut events = Vec::new();
    extract_gemini_events(&body, &mut events);

    if !events
        .iter()
        .any(|e| matches!(e, ProviderEvent::MessageStop { .. }))
    {
        let stop_reason = body
            .candidates
            .first()
            .and_then(|c| c.finish_reason.as_deref())
            .map(parse_gemini_stop_reason);
        events.push(ProviderEvent::MessageStop { stop_reason });
    }

    Ok(events)
}

async fn parse_gemini_sse(
    response: reqwest::Response,
) -> Result<Vec<ProviderEvent>, ProviderError> {
    let body = response
        .text()
        .await
        .map_err(|e| ProviderError::Network(e.to_string()))?;

    let mut events = Vec::new();
    let mut last_stop_reason: Option<StopReason> = None;

    for line in body.lines() {
        let line = line.trim();
        if let Some(data) = line.strip_prefix("data: ") {
            let chunk: Result<GeminiResponse, _> = serde_json::from_str(data);
            if let Ok(chunk) = chunk {
                extract_gemini_events(&chunk, &mut events);
                if let Some(candidate) = chunk.candidates.first() {
                    if let Some(ref reason) = candidate.finish_reason {
                        last_stop_reason = Some(parse_gemini_stop_reason(reason));
                    }
                }
            }
        }
    }

    events.push(ProviderEvent::MessageStop {
        stop_reason: last_stop_reason,
    });

    Ok(events)
}

fn extract_gemini_events(response: &GeminiResponse, events: &mut Vec<ProviderEvent>) {
    for candidate in &response.candidates {
        if let Some(ref content) = candidate.content {
            let mut tool_index = 0u32;
            for part in &content.parts {
                if let Some(ref text) = part.text {
                    if !text.is_empty() {
                        events.push(ProviderEvent::TextDelta(text.clone()));
                    }
                }
                if let Some(ref fc) = part.function_call {
                    let id = format!("gemini_call_{tool_index}");
                    tool_index += 1;
                    events.push(ProviderEvent::ToolUseStart {
                        id: id.clone(),
                        name: fc.name.clone(),
                    });
                    events.push(ProviderEvent::ToolUseComplete {
                        id,
                        name: fc.name.clone(),
                        input: fc.args.to_string(),
                    });
                }
            }
        }
    }

    if let Some(ref usage) = response.usage_metadata {
        if usage.prompt_token_count > 0 || usage.candidates_token_count > 0 {
            events.push(ProviderEvent::Usage(TokenUsage {
                input_tokens: usage.prompt_token_count,
                output_tokens: usage.candidates_token_count,
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
            }));
        }
    }
}

fn parse_gemini_stop_reason(reason: &str) -> StopReason {
    match reason {
        "MAX_TOKENS" => StopReason::MaxTokens,
        "TOOL_CALL" | "FUNCTION_CALL" => StopReason::ToolUse,
        // "STOP" and unknown reasons
        _ => StopReason::EndTurn,
    }
}

fn map_gemini_http_error(status: u16, body: &str) -> ProviderError {
    if status == 401 || status == 403 {
        return ProviderError::Auth(format!("Gemini auth error: {body}"));
    }
    if status == 429 {
        return ProviderError::RateLimited {
            retry_after_ms: None,
            message: body.to_string(),
        };
    }
    ProviderError::Api {
        status,
        message: body.to_string(),
        retryable: status >= 500,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ChatMessage;
    use serde_json::json;

    #[test]
    fn builds_gemini_request_with_system() {
        let request = ProviderRequest {
            model: "gemini-3-flash".to_string(),
            max_tokens: 4096,
            system_prompt: Some("Be helpful.".to_string()),
            messages: vec![ChatMessage {
                role: ChatRole::User,
                content: vec![ChatContent::Text("hello".to_string())],
            }],
            tools: None,
            stream: false,
        };
        let gemini = to_gemini_request(&request);
        assert!(gemini["systemInstruction"].is_object());
        assert_eq!(gemini["contents"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn builds_gemini_request_with_tools() {
        let request = ProviderRequest {
            model: "gemini-3-pro".to_string(),
            max_tokens: 8192,
            system_prompt: None,
            messages: vec![ChatMessage {
                role: ChatRole::User,
                content: vec![ChatContent::Text("read foo.rs".to_string())],
            }],
            tools: Some(vec![crate::ToolDefinition {
                name: "read_file".to_string(),
                description: Some("Read a file".to_string()),
                input_schema: json!({"type": "object", "properties": {"path": {"type": "string"}}}),
            }]),
            stream: false,
        };
        let gemini = to_gemini_request(&request);
        assert!(gemini["tools"].is_array());
    }

    #[test]
    fn parses_gemini_stop_reasons() {
        assert_eq!(parse_gemini_stop_reason("STOP"), StopReason::EndTurn);
        assert_eq!(
            parse_gemini_stop_reason("MAX_TOKENS"),
            StopReason::MaxTokens
        );
        assert_eq!(
            parse_gemini_stop_reason("FUNCTION_CALL"),
            StopReason::ToolUse
        );
    }

    #[test]
    fn extracts_events_from_gemini_response() {
        let response = GeminiResponse {
            candidates: vec![GeminiCandidate {
                content: Some(GeminiResponseContent {
                    parts: vec![GeminiResponsePart {
                        text: Some("Hello!".to_string()),
                        function_call: None,
                    }],
                }),
                finish_reason: Some("STOP".to_string()),
            }],
            usage_metadata: Some(GeminiUsageMetadata {
                prompt_token_count: 10,
                candidates_token_count: 5,
            }),
        };

        let mut events = Vec::new();
        extract_gemini_events(&response, &mut events);
        assert!(events
            .iter()
            .any(|e| matches!(e, ProviderEvent::TextDelta(t) if t == "Hello!")));
        assert!(events.iter().any(|e| matches!(e, ProviderEvent::Usage(_))));
    }
}
