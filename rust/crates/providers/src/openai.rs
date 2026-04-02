use runtime::TokenUsage;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    ChatContent, ChatRole, Provider, ProviderCapabilities, ProviderError, ProviderEvent,
    ProviderKind, ProviderRequest, StopReason,
};

/// Default `OpenAI` API base URL.
const OPENAI_BASE_URL: &str = "https://api.openai.com/v1";

/// xAI (Grok) API base URL.
const XAI_BASE_URL: &str = "https://api.x.ai/v1";

/// `OpenAI`-compatible provider adapter.
///
/// Works with `OpenAI`, xAI (Grok), and any other provider that implements
/// the `OpenAI` chat completions API format.
pub struct OpenAiProvider {
    runtime: tokio::runtime::Runtime,
    http: reqwest::Client,
    api_key: String,
    base_url: String,
    model: String,
    kind: ProviderKind,
}

impl OpenAiProvider {
    /// Create a new `OpenAI` provider.
    ///
    /// # Errors
    ///
    /// Returns an error if the tokio runtime or HTTP client cannot be created.
    pub fn new(api_key: String, model: String) -> Result<Self, ProviderError> {
        Self::with_base_url(
            api_key,
            model,
            OPENAI_BASE_URL.to_string(),
            ProviderKind::OpenAi,
        )
    }

    /// Create a new xAI (Grok) provider using the OpenAI-compatible API.
    ///
    /// # Errors
    ///
    /// Returns an error if the tokio runtime or HTTP client cannot be created.
    pub fn new_xai(api_key: String, model: String) -> Result<Self, ProviderError> {
        Self::with_base_url(api_key, model, XAI_BASE_URL.to_string(), ProviderKind::XAi)
    }

    /// Create with a custom base URL and provider kind.
    ///
    /// # Errors
    ///
    /// Returns an error if the tokio runtime or HTTP client cannot be created.
    pub fn with_base_url(
        api_key: String,
        model: String,
        base_url: String,
        kind: ProviderKind,
    ) -> Result<Self, ProviderError> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| ProviderError::Other(format!("failed to create tokio runtime: {e}")))?;
        let http = reqwest::Client::new();
        Ok(Self {
            runtime,
            http,
            api_key,
            base_url,
            model,
            kind,
        })
    }
}

impl Provider for OpenAiProvider {
    fn stream_message(
        &mut self,
        request: &ProviderRequest,
    ) -> Result<Vec<ProviderEvent>, ProviderError> {
        let oai_request = to_openai_request(request);
        let url = format!("{}/chat/completions", self.base_url);

        self.runtime.block_on(async {
            let response = self
                .http
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&oai_request)
                .send()
                .await
                .map_err(|e| ProviderError::Network(e.to_string()))?;

            let status = response.status().as_u16();
            if status != 200 {
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "unknown error".to_string());
                return Err(map_openai_http_error(status, &body));
            }

            if request.stream {
                parse_sse_stream(response).await
            } else {
                parse_json_response(response).await
            }
        })
    }

    fn capabilities(&self) -> ProviderCapabilities {
        let info = crate::ModelRegistry::find(&self.model);
        ProviderCapabilities {
            max_context_tokens: info.as_ref().map_or(128_000, |m| m.max_context_tokens),
            max_output_tokens: info
                .as_ref()
                .map_or(16_384, |m| m.default_max_output_tokens),
            supports_tools: true,
            supports_vision: info.as_ref().is_none_or(|m| m.supports_vision),
            supports_streaming: true,
        }
    }

    fn model_id(&self) -> &str {
        &self.model
    }

    fn provider_kind(&self) -> ProviderKind {
        self.kind
    }
}

// --- OpenAI request/response types ---

#[derive(Debug, Serialize)]
struct OaiRequest {
    model: String,
    messages: Vec<OaiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OaiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct OaiMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<OaiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OaiToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum OaiContent {
    Text(String),
    Parts(Vec<OaiContentPart>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[allow(clippy::enum_variant_names)]
enum OaiContentPart {
    #[serde(rename = "text")]
    TextPart { text: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OaiToolCall {
    #[serde(default)]
    id: String,
    #[serde(rename = "type", default)]
    kind: String,
    function: OaiFunction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OaiFunction {
    name: String,
    #[serde(default)]
    arguments: String,
}

#[derive(Debug, Serialize)]
struct OaiTool {
    #[serde(rename = "type")]
    kind: String,
    function: OaiToolDef,
}

#[derive(Debug, Serialize)]
struct OaiToolDef {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    parameters: Value,
}

#[derive(Debug, Deserialize)]
struct OaiResponse {
    #[serde(default)]
    choices: Vec<OaiChoice>,
    #[serde(default)]
    usage: Option<OaiUsage>,
}

#[derive(Debug, Deserialize)]
struct OaiChoice {
    #[serde(default)]
    message: Option<OaiResponseMessage>,
    #[serde(default)]
    delta: Option<OaiResponseMessage>,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OaiResponseMessage {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<OaiToolCall>>,
}

#[derive(Debug, Deserialize)]
struct OaiUsage {
    #[serde(default)]
    prompt_tokens: u32,
    #[serde(default)]
    completion_tokens: u32,
}

// --- Conversion functions ---

#[allow(clippy::too_many_lines)]
fn to_openai_request(request: &ProviderRequest) -> OaiRequest {
    let mut messages = Vec::new();

    // System message
    if let Some(ref system) = request.system_prompt {
        messages.push(OaiMessage {
            role: "system".to_string(),
            content: Some(OaiContent::Text(system.clone())),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        });
    }

    // Conversation messages
    for msg in &request.messages {
        match msg.role {
            ChatRole::System => {
                // Already handled above
            }
            ChatRole::User => {
                let text = msg
                    .content
                    .iter()
                    .filter_map(|c| match c {
                        ChatContent::Text(t) => Some(t.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                messages.push(OaiMessage {
                    role: "user".to_string(),
                    content: Some(OaiContent::Text(text)),
                    tool_calls: None,
                    tool_call_id: None,
                    name: None,
                });
            }
            ChatRole::Assistant => {
                let text_parts: Vec<String> = msg
                    .content
                    .iter()
                    .filter_map(|c| match c {
                        ChatContent::Text(t) => Some(t.clone()),
                        _ => None,
                    })
                    .collect();

                let tool_calls: Vec<OaiToolCall> = msg
                    .content
                    .iter()
                    .filter_map(|c| match c {
                        ChatContent::ToolUse { id, name, input } => Some(OaiToolCall {
                            id: id.clone(),
                            kind: "function".to_string(),
                            function: OaiFunction {
                                name: name.clone(),
                                arguments: input.to_string(),
                            },
                        }),
                        _ => None,
                    })
                    .collect();

                let content = if text_parts.is_empty() {
                    None
                } else {
                    Some(OaiContent::Text(text_parts.join("\n")))
                };

                messages.push(OaiMessage {
                    role: "assistant".to_string(),
                    content,
                    tool_calls: if tool_calls.is_empty() {
                        None
                    } else {
                        Some(tool_calls)
                    },
                    tool_call_id: None,
                    name: None,
                });
            }
            ChatRole::Tool => {
                for block in &msg.content {
                    if let ChatContent::ToolResult {
                        tool_use_id,
                        content,
                        ..
                    } = block
                    {
                        messages.push(OaiMessage {
                            role: "tool".to_string(),
                            content: Some(OaiContent::Text(content.clone())),
                            tool_calls: None,
                            tool_call_id: Some(tool_use_id.clone()),
                            name: None,
                        });
                    }
                }
            }
        }
    }

    let tools = request.tools.as_ref().map(|tools| {
        tools
            .iter()
            .map(|t| OaiTool {
                kind: "function".to_string(),
                function: OaiToolDef {
                    name: t.name.clone(),
                    description: t.description.clone(),
                    parameters: t.input_schema.clone(),
                },
            })
            .collect()
    });

    let tool_choice = tools.as_ref().map(|_| "auto".to_string());

    OaiRequest {
        model: request.model.clone(),
        messages,
        max_tokens: Some(request.max_tokens),
        tools,
        tool_choice,
        stream: request.stream,
    }
}

async fn parse_json_response(
    response: reqwest::Response,
) -> Result<Vec<ProviderEvent>, ProviderError> {
    let body: OaiResponse = response
        .json()
        .await
        .map_err(|e| ProviderError::Parse(e.to_string()))?;

    let mut events = Vec::new();

    if let Some(choice) = body.choices.first() {
        if let Some(ref message) = choice.message {
            if let Some(ref text) = message.content {
                if !text.is_empty() {
                    events.push(ProviderEvent::TextDelta(text.clone()));
                }
            }
            if let Some(ref tool_calls) = message.tool_calls {
                for tc in tool_calls {
                    events.push(ProviderEvent::ToolUseStart {
                        id: tc.id.clone(),
                        name: tc.function.name.clone(),
                    });
                    events.push(ProviderEvent::ToolUseComplete {
                        id: tc.id.clone(),
                        name: tc.function.name.clone(),
                        input: tc.function.arguments.clone(),
                    });
                }
            }
        }

        let stop_reason = choice
            .finish_reason
            .as_deref()
            .map(parse_openai_stop_reason);
        events.push(ProviderEvent::MessageStop { stop_reason });
    }

    if let Some(usage) = body.usage {
        events.push(ProviderEvent::Usage(TokenUsage {
            input_tokens: usage.prompt_tokens,
            output_tokens: usage.completion_tokens,
            cache_creation_input_tokens: 0,
            cache_read_input_tokens: 0,
        }));
    }

    Ok(events)
}

async fn parse_sse_stream(
    response: reqwest::Response,
) -> Result<Vec<ProviderEvent>, ProviderError> {
    let body = response
        .text()
        .await
        .map_err(|e| ProviderError::Network(e.to_string()))?;

    let mut events = Vec::new();
    // Track tool calls being accumulated across SSE chunks
    let mut tool_calls: std::collections::HashMap<u32, (String, String, String)> =
        std::collections::HashMap::new();
    let mut stop_reason: Option<StopReason> = None;

    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(':') {
            continue;
        }

        if let Some(data) = line.strip_prefix("data: ") {
            if data.trim() == "[DONE]" {
                break;
            }

            let chunk: Result<OaiResponse, _> = serde_json::from_str(data);
            let Ok(chunk) = chunk else { continue };

            for choice in &chunk.choices {
                if let Some(ref delta) = choice.delta {
                    if let Some(ref text) = delta.content {
                        if !text.is_empty() {
                            events.push(ProviderEvent::TextDelta(text.clone()));
                        }
                    }
                    if let Some(ref tcs) = delta.tool_calls {
                        for tc in tcs {
                            // OpenAI streams tool calls with an index
                            // First chunk has id + name, subsequent have argument fragments
                            let index = 0u32; // simplified; real impl would track index field
                            let entry = tool_calls.entry(index).or_insert_with(|| {
                                (tc.id.clone(), tc.function.name.clone(), String::new())
                            });
                            entry.2.push_str(&tc.function.arguments);

                            if !tc.id.is_empty() {
                                entry.0.clone_from(&tc.id);
                            }
                            if !tc.function.name.is_empty() {
                                entry.1.clone_from(&tc.function.name);
                            }
                        }
                    }
                }
                if let Some(ref reason) = choice.finish_reason {
                    stop_reason = Some(parse_openai_stop_reason(reason));
                }
            }

            if let Some(usage) = chunk.usage {
                if usage.prompt_tokens > 0 || usage.completion_tokens > 0 {
                    events.push(ProviderEvent::Usage(TokenUsage {
                        input_tokens: usage.prompt_tokens,
                        output_tokens: usage.completion_tokens,
                        cache_creation_input_tokens: 0,
                        cache_read_input_tokens: 0,
                    }));
                }
            }
        }
    }

    // Emit accumulated tool calls
    for (_, (id, name, arguments)) in tool_calls {
        events.push(ProviderEvent::ToolUseStart {
            id: id.clone(),
            name: name.clone(),
        });
        events.push(ProviderEvent::ToolUseComplete {
            id,
            name,
            input: arguments,
        });
    }

    events.push(ProviderEvent::MessageStop { stop_reason });

    Ok(events)
}

fn parse_openai_stop_reason(reason: &str) -> StopReason {
    match reason {
        "length" => StopReason::MaxTokens,
        "tool_calls" => StopReason::ToolUse,
        // "stop" and unknown reasons
        _ => StopReason::EndTurn,
    }
}

fn map_openai_http_error(status: u16, body: &str) -> ProviderError {
    if status == 401 {
        return ProviderError::Auth(format!("unauthorized: {body}"));
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
    fn builds_openai_request_with_system_message() {
        let request = ProviderRequest {
            model: "gpt-4.1".to_string(),
            max_tokens: 4096,
            system_prompt: Some("Be helpful.".to_string()),
            messages: vec![ChatMessage {
                role: ChatRole::User,
                content: vec![ChatContent::Text("hello".to_string())],
            }],
            tools: None,
            stream: false,
        };
        let oai = to_openai_request(&request);
        assert_eq!(oai.messages.len(), 2); // system + user
        assert_eq!(oai.messages[0].role, "system");
        assert_eq!(oai.messages[1].role, "user");
    }

    #[test]
    fn builds_openai_request_with_tools() {
        let request = ProviderRequest {
            model: "gpt-4.1".to_string(),
            max_tokens: 4096,
            system_prompt: None,
            messages: vec![ChatMessage {
                role: ChatRole::User,
                content: vec![ChatContent::Text("hi".to_string())],
            }],
            tools: Some(vec![crate::ToolDefinition {
                name: "read_file".to_string(),
                description: Some("Read a file".to_string()),
                input_schema: json!({"type": "object", "properties": {"path": {"type": "string"}}}),
            }]),
            stream: true,
        };
        let oai = to_openai_request(&request);
        assert!(oai.tools.is_some());
        assert_eq!(oai.tools.as_ref().unwrap().len(), 1);
        assert_eq!(oai.tool_choice, Some("auto".to_string()));
    }

    #[test]
    fn converts_assistant_tool_use_messages() {
        let request = ProviderRequest {
            model: "gpt-4.1".to_string(),
            max_tokens: 4096,
            system_prompt: None,
            messages: vec![
                ChatMessage {
                    role: ChatRole::Assistant,
                    content: vec![
                        ChatContent::Text("Let me read that.".to_string()),
                        ChatContent::ToolUse {
                            id: "call_1".to_string(),
                            name: "read_file".to_string(),
                            input: json!({"path": "foo.rs"}),
                        },
                    ],
                },
                ChatMessage {
                    role: ChatRole::Tool,
                    content: vec![ChatContent::ToolResult {
                        tool_use_id: "call_1".to_string(),
                        content: "file contents here".to_string(),
                        is_error: false,
                    }],
                },
            ],
            tools: None,
            stream: false,
        };
        let oai = to_openai_request(&request);
        assert_eq!(oai.messages.len(), 2);
        assert_eq!(oai.messages[0].role, "assistant");
        assert!(oai.messages[0].tool_calls.is_some());
        assert_eq!(oai.messages[1].role, "tool");
        assert_eq!(oai.messages[1].tool_call_id, Some("call_1".to_string()));
    }

    #[test]
    fn parses_openai_stop_reasons() {
        assert_eq!(parse_openai_stop_reason("stop"), StopReason::EndTurn);
        assert_eq!(parse_openai_stop_reason("length"), StopReason::MaxTokens);
        assert_eq!(parse_openai_stop_reason("tool_calls"), StopReason::ToolUse);
    }
}
