mod anthropic;
mod gemini;
mod models;
mod openai;

pub use anthropic::AnthropicProvider;
pub use gemini::GeminiProvider;
pub use models::{
    max_tokens_for_model, pricing_for_model, ModelInfo, ModelRegistry, ModelTier, ProviderKind,
};
pub use openai::OpenAiProvider;

use runtime::TokenUsage;
use serde_json::Value;

/// Events emitted by a provider during streaming.
#[derive(Debug, Clone, PartialEq)]
pub enum ProviderEvent {
    /// Incremental text output from the model.
    TextDelta(String),

    /// A tool-use block is starting (id and name known, input streaming).
    ToolUseStart { id: String, name: String },

    /// Incremental JSON fragment for the current tool-use input.
    ToolUseInputDelta(String),

    /// The current tool-use block is complete; `input` is the full JSON string.
    ToolUseComplete {
        id: String,
        name: String,
        input: String,
    },

    /// Token usage for the current response.
    Usage(TokenUsage),

    /// The model finished producing output.
    MessageStop { stop_reason: Option<StopReason> },

    /// Provider-level error surfaced during streaming.
    Error(String),
}

/// Why the model stopped generating.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    /// Natural end of response.
    EndTurn,
    /// Model hit max output tokens.
    MaxTokens,
    /// Model invoked one or more tools.
    ToolUse,
    /// Model hit a stop sequence.
    StopSequence,
}

/// Capabilities a provider adapter declares.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderCapabilities {
    /// Maximum context window in tokens.
    pub max_context_tokens: u32,
    /// Maximum output tokens the model can produce.
    pub max_output_tokens: u32,
    /// Whether the model supports tool/function calling.
    pub supports_tools: bool,
    /// Whether the model supports image/vision input.
    pub supports_vision: bool,
    /// Whether the model supports streaming responses.
    pub supports_streaming: bool,
}

/// A tool definition sent to a provider.
#[derive(Debug, Clone)]
pub struct ToolDefinition {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: Value,
}

/// A message in the conversation history.
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: Vec<ChatContent>,
}

/// Role of a message in the conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatRole {
    System,
    User,
    Assistant,
    Tool,
}

/// Content blocks within a message.
#[derive(Debug, Clone)]
pub enum ChatContent {
    Text(String),
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    ToolResult {
        tool_use_id: String,
        content: String,
        is_error: bool,
    },
}

/// Request parameters for a provider call.
#[derive(Debug, Clone)]
pub struct ProviderRequest {
    pub model: String,
    pub max_tokens: u32,
    pub system_prompt: Option<String>,
    pub messages: Vec<ChatMessage>,
    pub tools: Option<Vec<ToolDefinition>>,
    pub stream: bool,
}

/// Errors that any provider can return.
#[derive(Debug)]
pub enum ProviderError {
    /// Authentication failed (missing or invalid key).
    Auth(String),
    /// HTTP or network-level error.
    Network(String),
    /// Rate limited — caller should retry after the given duration.
    RateLimited {
        retry_after_ms: Option<u64>,
        message: String,
    },
    /// The context was too long for the model.
    ContextTooLong(String),
    /// Provider returned an error response.
    Api {
        status: u16,
        message: String,
        retryable: bool,
    },
    /// Failed to parse provider response.
    Parse(String),
    /// Generic error.
    Other(String),
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auth(msg) => write!(f, "auth error: {msg}"),
            Self::Network(msg) => write!(f, "network error: {msg}"),
            Self::RateLimited { message, .. } => write!(f, "rate limited: {message}"),
            Self::ContextTooLong(msg) => write!(f, "context too long: {msg}"),
            Self::Api {
                status, message, ..
            } => write!(f, "API error ({status}): {message}"),
            Self::Parse(msg) => write!(f, "parse error: {msg}"),
            Self::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for ProviderError {}

/// The core provider trait. Each LLM backend implements this.
///
/// Providers convert a [`ProviderRequest`] into a stream of [`ProviderEvent`]s.
/// The runtime calls `stream_message` and drives the event loop.
pub trait Provider {
    /// Send a request and collect the response as a vector of events.
    ///
    /// Implementations should handle SSE parsing, retry logic, and
    /// normalize the provider-specific wire format into [`ProviderEvent`]s.
    fn stream_message(
        &mut self,
        request: &ProviderRequest,
    ) -> Result<Vec<ProviderEvent>, ProviderError>;

    /// Return the capabilities of the current model.
    fn capabilities(&self) -> ProviderCapabilities;

    /// Return the model identifier string (e.g. "claude-sonnet-4-20250514").
    fn model_id(&self) -> &str;

    /// Return the provider kind.
    fn provider_kind(&self) -> ProviderKind;
}

/// Resolve a `provider:model` string into a provider kind and model ID.
///
/// Accepted formats:
/// - `"anthropic:claude-sonnet-4-20250514"` — explicit provider prefix
/// - `"claude-sonnet-4"` — auto-detect from model name
/// - `"gpt-4.1"` — auto-detect from model name
/// - `"gemini-3-flash"` — auto-detect from model name
#[must_use]
pub fn resolve_provider_model(spec: &str) -> (ProviderKind, String) {
    if let Some((prefix, model)) = spec.split_once(':') {
        let kind = match prefix.to_ascii_lowercase().as_str() {
            "openai" | "gpt" => ProviderKind::OpenAi,
            "google" | "gemini" => ProviderKind::Google,
            "xai" | "grok" => ProviderKind::XAi,
            // "anthropic", "claude", or any unknown prefix defaults to Anthropic
            _ => ProviderKind::Anthropic,
        };
        return (kind, model.to_string());
    }

    let lower = spec.to_ascii_lowercase();
    if lower.starts_with("claude") {
        (ProviderKind::Anthropic, spec.to_string())
    } else if lower.starts_with("gpt") {
        (ProviderKind::OpenAi, spec.to_string())
    } else if lower.starts_with("gemini") {
        (ProviderKind::Google, spec.to_string())
    } else if lower.starts_with("grok") {
        (ProviderKind::XAi, spec.to_string())
    } else {
        // Default to Anthropic for unknown model names
        (ProviderKind::Anthropic, spec.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_explicit_provider_prefix() {
        let (kind, model) = resolve_provider_model("openai:gpt-4.1");
        assert_eq!(kind, ProviderKind::OpenAi);
        assert_eq!(model, "gpt-4.1");
    }

    #[test]
    fn resolves_claude_prefix() {
        let (kind, model) = resolve_provider_model("claude:claude-sonnet-4-20250514");
        assert_eq!(kind, ProviderKind::Anthropic);
        assert_eq!(model, "claude-sonnet-4-20250514");
    }

    #[test]
    fn auto_detects_anthropic_from_model_name() {
        let (kind, _) = resolve_provider_model("claude-opus-4-6");
        assert_eq!(kind, ProviderKind::Anthropic);
    }

    #[test]
    fn auto_detects_openai_from_model_name() {
        let (kind, _) = resolve_provider_model("gpt-5.1");
        assert_eq!(kind, ProviderKind::OpenAi);
    }

    #[test]
    fn auto_detects_google_from_model_name() {
        let (kind, _) = resolve_provider_model("gemini-3-flash");
        assert_eq!(kind, ProviderKind::Google);
    }

    #[test]
    fn auto_detects_xai_from_model_name() {
        let (kind, _) = resolve_provider_model("grok-code-fast-1");
        assert_eq!(kind, ProviderKind::XAi);
    }

    #[test]
    fn defaults_unknown_to_anthropic() {
        let (kind, model) = resolve_provider_model("some-custom-model");
        assert_eq!(kind, ProviderKind::Anthropic);
        assert_eq!(model, "some-custom-model");
    }
}
