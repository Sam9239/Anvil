use api::{
    AnthropicClient, AuthSource, ContentBlockDelta, OutputContentBlock,
    StreamEvent as ApiStreamEvent,
};
use runtime::TokenUsage;

use crate::{
    ChatContent, ChatMessage, ChatRole, Provider, ProviderCapabilities, ProviderError,
    ProviderEvent, ProviderKind, ProviderRequest, StopReason,
};

/// Anthropic provider adapter wrapping the existing `api` crate.
pub struct AnthropicProvider {
    runtime: tokio::runtime::Runtime,
    client: AnthropicClient,
    model: String,
}

impl AnthropicProvider {
    /// Create a new Anthropic provider with the given auth and model.
    ///
    /// # Errors
    ///
    /// Returns an error if the tokio runtime cannot be created.
    pub fn new(auth: AuthSource, model: String) -> Result<Self, ProviderError> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| ProviderError::Other(format!("failed to create tokio runtime: {e}")))?;
        let client = AnthropicClient::from_auth(auth).with_base_url(api::read_base_url());
        Ok(Self {
            runtime,
            client,
            model,
        })
    }

    /// Create with a custom base URL.
    #[must_use]
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.client = AnthropicClient::from_auth(AuthSource::None).with_base_url(base_url);
        self
    }
}

impl Provider for AnthropicProvider {
    fn stream_message(
        &mut self,
        request: &ProviderRequest,
    ) -> Result<Vec<ProviderEvent>, ProviderError> {
        let api_request = to_anthropic_request(request);
        self.runtime.block_on(async {
            let mut stream = self
                .client
                .stream_message(&api_request)
                .await
                .map_err(map_api_error)?;

            let mut events = Vec::new();
            let mut pending_tool: Option<(String, String, String)> = None;

            while let Some(event) = stream.next_event().await.map_err(map_api_error)? {
                match event {
                    ApiStreamEvent::MessageStart(start) => {
                        // Process any content blocks in the initial message
                        for block in start.message.content {
                            process_output_block(block, &mut events, &mut pending_tool);
                        }
                        // Emit initial usage if available
                        let u = &start.message.usage;
                        if u.input_tokens > 0 || u.output_tokens > 0 {
                            events.push(ProviderEvent::Usage(TokenUsage {
                                input_tokens: u.input_tokens,
                                output_tokens: u.output_tokens,
                                cache_creation_input_tokens: u.cache_creation_input_tokens,
                                cache_read_input_tokens: u.cache_read_input_tokens,
                            }));
                        }
                    }
                    ApiStreamEvent::ContentBlockStart(start) => {
                        process_output_block(start.content_block, &mut events, &mut pending_tool);
                    }
                    ApiStreamEvent::ContentBlockDelta(delta) => match delta.delta {
                        ContentBlockDelta::TextDelta { text } => {
                            if !text.is_empty() {
                                events.push(ProviderEvent::TextDelta(text));
                            }
                        }
                        ContentBlockDelta::InputJsonDelta { partial_json } => {
                            if let Some((_, _, ref mut input)) = pending_tool {
                                input.push_str(&partial_json);
                            }
                            events.push(ProviderEvent::ToolUseInputDelta(partial_json));
                        }
                    },
                    ApiStreamEvent::ContentBlockStop(_) => {
                        if let Some((id, name, input)) = pending_tool.take() {
                            events.push(ProviderEvent::ToolUseComplete { id, name, input });
                        }
                    }
                    ApiStreamEvent::MessageDelta(delta) => {
                        let stop_reason = delta
                            .delta
                            .stop_reason
                            .as_deref()
                            .map(parse_anthropic_stop_reason);
                        events.push(ProviderEvent::Usage(TokenUsage {
                            input_tokens: delta.usage.input_tokens,
                            output_tokens: delta.usage.output_tokens,
                            cache_creation_input_tokens: 0,
                            cache_read_input_tokens: 0,
                        }));
                        if stop_reason.is_some() {
                            events.push(ProviderEvent::MessageStop { stop_reason });
                        }
                    }
                    ApiStreamEvent::MessageStop(_) => {
                        if !events
                            .iter()
                            .any(|e| matches!(e, ProviderEvent::MessageStop { .. }))
                        {
                            events.push(ProviderEvent::MessageStop { stop_reason: None });
                        }
                    }
                }
            }

            // Ensure we always have a MessageStop
            if !events
                .iter()
                .any(|e| matches!(e, ProviderEvent::MessageStop { .. }))
            {
                let has_content = events.iter().any(|e| {
                    matches!(
                        e,
                        ProviderEvent::TextDelta(_) | ProviderEvent::ToolUseComplete { .. }
                    )
                });
                if has_content {
                    events.push(ProviderEvent::MessageStop { stop_reason: None });
                }
            }

            Ok(events)
        })
    }

    fn capabilities(&self) -> ProviderCapabilities {
        let info = crate::ModelRegistry::find(&self.model);
        ProviderCapabilities {
            max_context_tokens: info.as_ref().map_or(200_000, |m| m.max_context_tokens),
            max_output_tokens: info
                .as_ref()
                .map_or(16_000, |m| m.default_max_output_tokens),
            supports_tools: true,
            supports_vision: true,
            supports_streaming: true,
        }
    }

    fn model_id(&self) -> &str {
        &self.model
    }

    fn provider_kind(&self) -> ProviderKind {
        ProviderKind::Anthropic
    }
}

fn process_output_block(
    block: OutputContentBlock,
    events: &mut Vec<ProviderEvent>,
    pending_tool: &mut Option<(String, String, String)>,
) {
    match block {
        OutputContentBlock::Text { text } => {
            if !text.is_empty() {
                events.push(ProviderEvent::TextDelta(text));
            }
        }
        OutputContentBlock::ToolUse { id, name, input } => {
            let input_str = input.to_string();
            events.push(ProviderEvent::ToolUseStart {
                id: id.clone(),
                name: name.clone(),
            });
            *pending_tool = Some((id, name, input_str));
        }
    }
}

fn to_anthropic_request(request: &ProviderRequest) -> api::MessageRequest {
    let messages = request
        .messages
        .iter()
        .filter(|m| m.role != ChatRole::System)
        .map(to_anthropic_message)
        .collect();

    let tools = request.tools.as_ref().map(|tools| {
        tools
            .iter()
            .map(|t| api::ToolDefinition {
                name: t.name.clone(),
                description: t.description.clone(),
                input_schema: t.input_schema.clone(),
            })
            .collect()
    });

    let tool_choice = tools.as_ref().map(|_| api::ToolChoice::Auto);

    api::MessageRequest {
        model: request.model.clone(),
        max_tokens: request.max_tokens,
        messages,
        system: request.system_prompt.clone(),
        tools,
        tool_choice,
        stream: request.stream,
    }
}

fn to_anthropic_message(msg: &ChatMessage) -> api::InputMessage {
    let role = match msg.role {
        ChatRole::User | ChatRole::System | ChatRole::Tool => "user",
        ChatRole::Assistant => "assistant",
    };

    let content = msg
        .content
        .iter()
        .map(|c| match c {
            ChatContent::Text(text) => api::InputContentBlock::Text { text: text.clone() },
            ChatContent::ToolUse { id, name, input } => api::InputContentBlock::ToolUse {
                id: id.clone(),
                name: name.clone(),
                input: input.clone(),
            },
            ChatContent::ToolResult {
                tool_use_id,
                content,
                is_error,
            } => api::InputContentBlock::ToolResult {
                tool_use_id: tool_use_id.clone(),
                content: vec![api::ToolResultContentBlock::Text {
                    text: content.clone(),
                }],
                is_error: *is_error,
            },
        })
        .collect();

    api::InputMessage {
        role: role.to_string(),
        content,
    }
}

fn parse_anthropic_stop_reason(reason: &str) -> StopReason {
    match reason {
        "max_tokens" => StopReason::MaxTokens,
        "tool_use" => StopReason::ToolUse,
        "stop_sequence" => StopReason::StopSequence,
        // "end_turn" and unknown reasons
        _ => StopReason::EndTurn,
    }
}

fn map_api_error(error: api::ApiError) -> ProviderError {
    match error {
        api::ApiError::MissingApiKey => {
            ProviderError::Auth("missing Anthropic API key".to_string())
        }
        api::ApiError::ExpiredOAuthToken => {
            ProviderError::Auth("expired Anthropic OAuth token".to_string())
        }
        api::ApiError::Auth(msg) => ProviderError::Auth(msg),
        api::ApiError::Http(e) => ProviderError::Network(e.to_string()),
        api::ApiError::Api {
            status,
            message,
            body,
            retryable,
            ..
        } => {
            let msg = message.unwrap_or(body);
            let code = status.as_u16();
            if code == 429 {
                ProviderError::RateLimited {
                    retry_after_ms: None,
                    message: msg,
                }
            } else {
                ProviderError::Api {
                    status: code,
                    message: msg,
                    retryable,
                }
            }
        }
        api::ApiError::RetriesExhausted { last_error, .. } => {
            ProviderError::Network(last_error.to_string())
        }
        other => ProviderError::Other(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn converts_chat_messages_to_anthropic_format() {
        let msg = ChatMessage {
            role: ChatRole::User,
            content: vec![ChatContent::Text("hello".to_string())],
        };
        let api_msg = to_anthropic_message(&msg);
        assert_eq!(api_msg.role, "user");
        assert_eq!(api_msg.content.len(), 1);
    }

    #[test]
    fn converts_tool_result_messages() {
        let msg = ChatMessage {
            role: ChatRole::Tool,
            content: vec![ChatContent::ToolResult {
                tool_use_id: "t1".to_string(),
                content: "result text".to_string(),
                is_error: false,
            }],
        };
        let api_msg = to_anthropic_message(&msg);
        assert_eq!(api_msg.role, "user");
    }

    #[test]
    fn builds_full_anthropic_request() {
        let request = ProviderRequest {
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 4096,
            system_prompt: Some("You are helpful.".to_string()),
            messages: vec![ChatMessage {
                role: ChatRole::User,
                content: vec![ChatContent::Text("hi".to_string())],
            }],
            tools: Some(vec![crate::ToolDefinition {
                name: "test".to_string(),
                description: Some("A test tool".to_string()),
                input_schema: json!({"type": "object"}),
            }]),
            stream: true,
        };
        let api_req = to_anthropic_request(&request);
        assert_eq!(api_req.model, "claude-sonnet-4-20250514");
        assert!(api_req.system.is_some());
        assert!(api_req.tools.is_some());
        assert!(api_req.stream);
    }

    #[test]
    fn parses_stop_reasons() {
        assert_eq!(parse_anthropic_stop_reason("end_turn"), StopReason::EndTurn);
        assert_eq!(
            parse_anthropic_stop_reason("max_tokens"),
            StopReason::MaxTokens
        );
        assert_eq!(parse_anthropic_stop_reason("tool_use"), StopReason::ToolUse);
    }
}
