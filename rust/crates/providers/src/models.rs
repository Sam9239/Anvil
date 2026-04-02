use runtime::ModelPricing;

/// Which LLM provider family a model belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderKind {
    Anthropic,
    OpenAi,
    Google,
    XAi,
}

impl std::fmt::Display for ProviderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Anthropic => write!(f, "anthropic"),
            Self::OpenAi => write!(f, "openai"),
            Self::Google => write!(f, "google"),
            Self::XAi => write!(f, "xai"),
        }
    }
}

/// Performance/cost tier for a model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelTier {
    /// Fast and cost-efficient (e.g. Haiku, GPT-5 mini, Gemini Flash).
    Fast,
    /// Versatile and highly intelligent (e.g. Sonnet, GPT-4.1, GPT-5).
    Balanced,
    /// Most powerful at complex tasks (e.g. Opus, GPT-5.2, Gemini Pro).
    Powerful,
}

/// Static metadata about a model.
#[derive(Debug, Clone)]
pub struct ModelInfo {
    /// Display name (e.g. "Claude Sonnet 4").
    pub display_name: &'static str,
    /// API model ID string (e.g. "claude-sonnet-4-20250514").
    pub model_id: &'static str,
    /// Which provider serves this model.
    pub provider: ProviderKind,
    /// Performance tier.
    pub tier: ModelTier,
    /// Max context window in tokens.
    pub max_context_tokens: u32,
    /// Default max output tokens.
    pub default_max_output_tokens: u32,
    /// Whether tool/function calling is supported.
    pub supports_tools: bool,
    /// Whether vision/image input is supported.
    pub supports_vision: bool,
    /// Known pricing (if available).
    pub pricing: Option<ModelPricing>,
}

/// Registry of all known models.
pub struct ModelRegistry;

impl ModelRegistry {
    /// All known models.
    #[must_use]
    pub fn all() -> Vec<ModelInfo> {
        let mut models = Vec::new();
        models.extend(Self::anthropic_models());
        models.extend(Self::openai_models());
        models.extend(Self::google_models());
        models.extend(Self::xai_models());
        models
    }

    /// Look up a model by its API ID.
    #[must_use]
    pub fn find(model_id: &str) -> Option<ModelInfo> {
        Self::all().into_iter().find(|m| m.model_id == model_id)
    }

    /// Look up a model by a short alias (e.g. "sonnet", "gpt-4.1", "opus").
    #[must_use]
    pub fn find_by_alias(alias: &str) -> Option<ModelInfo> {
        let lower = alias.to_ascii_lowercase();
        Self::all().into_iter().find(|m| {
            m.model_id.to_ascii_lowercase().contains(&lower)
                || m.display_name.to_ascii_lowercase().contains(&lower)
        })
    }

    /// Get the default model for a given provider.
    #[must_use]
    pub fn default_for_provider(provider: ProviderKind) -> Option<ModelInfo> {
        match provider {
            ProviderKind::Anthropic => Self::find("claude-sonnet-4-20250514"),
            ProviderKind::OpenAi => Self::find("gpt-4.1"),
            ProviderKind::Google => Self::find("gemini-3-flash"),
            ProviderKind::XAi => Self::find("grok-code-fast-1"),
        }
    }

    /// List models for a specific provider.
    #[must_use]
    pub fn for_provider(provider: ProviderKind) -> Vec<ModelInfo> {
        Self::all()
            .into_iter()
            .filter(|m| m.provider == provider)
            .collect()
    }

    #[must_use]
    fn anthropic_models() -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                display_name: "Claude Haiku 4.5",
                model_id: "claude-haiku-4-5-20251001",
                provider: ProviderKind::Anthropic,
                tier: ModelTier::Fast,
                max_context_tokens: 200_000,
                default_max_output_tokens: 8_192,
                supports_tools: true,
                supports_vision: true,
                pricing: Some(ModelPricing {
                    input_cost_per_million: 1.0,
                    output_cost_per_million: 5.0,
                    cache_creation_cost_per_million: 1.25,
                    cache_read_cost_per_million: 0.1,
                }),
            },
            ModelInfo {
                display_name: "Claude Sonnet 4",
                model_id: "claude-sonnet-4-20250514",
                provider: ProviderKind::Anthropic,
                tier: ModelTier::Balanced,
                max_context_tokens: 200_000,
                default_max_output_tokens: 16_000,
                supports_tools: true,
                supports_vision: true,
                pricing: Some(ModelPricing {
                    input_cost_per_million: 3.0,
                    output_cost_per_million: 15.0,
                    cache_creation_cost_per_million: 3.75,
                    cache_read_cost_per_million: 0.3,
                }),
            },
            ModelInfo {
                display_name: "Claude Sonnet 4.5",
                model_id: "claude-sonnet-4-5-20250514",
                provider: ProviderKind::Anthropic,
                tier: ModelTier::Balanced,
                max_context_tokens: 200_000,
                default_max_output_tokens: 16_000,
                supports_tools: true,
                supports_vision: true,
                pricing: Some(ModelPricing {
                    input_cost_per_million: 3.0,
                    output_cost_per_million: 15.0,
                    cache_creation_cost_per_million: 3.75,
                    cache_read_cost_per_million: 0.3,
                }),
            },
            ModelInfo {
                display_name: "Claude Opus 4.5",
                model_id: "claude-opus-4-5-20250520",
                provider: ProviderKind::Anthropic,
                tier: ModelTier::Powerful,
                max_context_tokens: 200_000,
                default_max_output_tokens: 32_000,
                supports_tools: true,
                supports_vision: true,
                pricing: Some(ModelPricing {
                    input_cost_per_million: 15.0,
                    output_cost_per_million: 75.0,
                    cache_creation_cost_per_million: 18.75,
                    cache_read_cost_per_million: 1.5,
                }),
            },
        ]
    }

    #[must_use]
    fn openai_models() -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                display_name: "GPT-5 mini",
                model_id: "gpt-5-mini",
                provider: ProviderKind::OpenAi,
                tier: ModelTier::Fast,
                max_context_tokens: 128_000,
                default_max_output_tokens: 16_384,
                supports_tools: true,
                supports_vision: true,
                pricing: Some(ModelPricing {
                    input_cost_per_million: 1.50,
                    output_cost_per_million: 6.0,
                    cache_creation_cost_per_million: 0.0,
                    cache_read_cost_per_million: 0.375,
                }),
            },
            ModelInfo {
                display_name: "GPT-4o",
                model_id: "gpt-4o",
                provider: ProviderKind::OpenAi,
                tier: ModelTier::Balanced,
                max_context_tokens: 128_000,
                default_max_output_tokens: 16_384,
                supports_tools: true,
                supports_vision: true,
                pricing: Some(ModelPricing {
                    input_cost_per_million: 2.50,
                    output_cost_per_million: 10.0,
                    cache_creation_cost_per_million: 0.0,
                    cache_read_cost_per_million: 1.25,
                }),
            },
            ModelInfo {
                display_name: "GPT-4.1",
                model_id: "gpt-4.1",
                provider: ProviderKind::OpenAi,
                tier: ModelTier::Balanced,
                max_context_tokens: 1_000_000,
                default_max_output_tokens: 32_768,
                supports_tools: true,
                supports_vision: true,
                pricing: Some(ModelPricing {
                    input_cost_per_million: 2.0,
                    output_cost_per_million: 8.0,
                    cache_creation_cost_per_million: 0.0,
                    cache_read_cost_per_million: 0.5,
                }),
            },
            ModelInfo {
                display_name: "GPT-5",
                model_id: "gpt-5",
                provider: ProviderKind::OpenAi,
                tier: ModelTier::Balanced,
                max_context_tokens: 128_000,
                default_max_output_tokens: 32_768,
                supports_tools: true,
                supports_vision: true,
                pricing: Some(ModelPricing {
                    input_cost_per_million: 5.0,
                    output_cost_per_million: 20.0,
                    cache_creation_cost_per_million: 0.0,
                    cache_read_cost_per_million: 1.25,
                }),
            },
            ModelInfo {
                display_name: "GPT-5.1",
                model_id: "gpt-5.1",
                provider: ProviderKind::OpenAi,
                tier: ModelTier::Powerful,
                max_context_tokens: 256_000,
                default_max_output_tokens: 32_768,
                supports_tools: true,
                supports_vision: true,
                pricing: Some(ModelPricing {
                    input_cost_per_million: 10.0,
                    output_cost_per_million: 40.0,
                    cache_creation_cost_per_million: 0.0,
                    cache_read_cost_per_million: 2.5,
                }),
            },
            ModelInfo {
                display_name: "GPT-5.2",
                model_id: "gpt-5.2",
                provider: ProviderKind::OpenAi,
                tier: ModelTier::Powerful,
                max_context_tokens: 256_000,
                default_max_output_tokens: 32_768,
                supports_tools: true,
                supports_vision: true,
                pricing: Some(ModelPricing {
                    input_cost_per_million: 15.0,
                    output_cost_per_million: 60.0,
                    cache_creation_cost_per_million: 0.0,
                    cache_read_cost_per_million: 3.75,
                }),
            },
        ]
    }

    #[must_use]
    fn google_models() -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                display_name: "Gemini 3 Flash",
                model_id: "gemini-3-flash",
                provider: ProviderKind::Google,
                tier: ModelTier::Fast,
                max_context_tokens: 1_000_000,
                default_max_output_tokens: 8_192,
                supports_tools: true,
                supports_vision: true,
                pricing: Some(ModelPricing {
                    input_cost_per_million: 0.15,
                    output_cost_per_million: 0.60,
                    cache_creation_cost_per_million: 0.0,
                    cache_read_cost_per_million: 0.0375,
                }),
            },
            ModelInfo {
                display_name: "Gemini 3 Pro",
                model_id: "gemini-3-pro",
                provider: ProviderKind::Google,
                tier: ModelTier::Powerful,
                max_context_tokens: 1_000_000,
                default_max_output_tokens: 8_192,
                supports_tools: true,
                supports_vision: true,
                pricing: Some(ModelPricing {
                    input_cost_per_million: 2.50,
                    output_cost_per_million: 10.0,
                    cache_creation_cost_per_million: 0.0,
                    cache_read_cost_per_million: 0.625,
                }),
            },
        ]
    }

    #[must_use]
    fn xai_models() -> Vec<ModelInfo> {
        vec![ModelInfo {
            display_name: "Grok Code Fast 1",
            model_id: "grok-code-fast-1",
            provider: ProviderKind::XAi,
            tier: ModelTier::Fast,
            max_context_tokens: 128_000,
            default_max_output_tokens: 16_384,
            supports_tools: true,
            supports_vision: false,
            pricing: Some(ModelPricing {
                input_cost_per_million: 0.15,
                output_cost_per_million: 0.60,
                cache_creation_cost_per_million: 0.0,
                cache_read_cost_per_million: 0.0,
            }),
        }]
    }
}

/// Get pricing for a model ID across all providers.
#[must_use]
pub fn pricing_for_model(model_id: &str) -> Option<ModelPricing> {
    ModelRegistry::find(model_id)
        .and_then(|m| m.pricing)
        .or_else(|| runtime::pricing_for_model(model_id))
}

/// Get the default max output tokens for a model.
#[must_use]
pub fn max_tokens_for_model(model_id: &str) -> u32 {
    ModelRegistry::find(model_id).map_or(16_384, |m| m.default_max_output_tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_contains_all_providers() {
        let all = ModelRegistry::all();
        assert!(all.iter().any(|m| m.provider == ProviderKind::Anthropic));
        assert!(all.iter().any(|m| m.provider == ProviderKind::OpenAi));
        assert!(all.iter().any(|m| m.provider == ProviderKind::Google));
        assert!(all.iter().any(|m| m.provider == ProviderKind::XAi));
    }

    #[test]
    fn finds_model_by_id() {
        let model = ModelRegistry::find("gpt-4.1").expect("gpt-4.1 should exist");
        assert_eq!(model.provider, ProviderKind::OpenAi);
        assert_eq!(model.display_name, "GPT-4.1");
    }

    #[test]
    fn finds_model_by_alias() {
        let model = ModelRegistry::find_by_alias("haiku").expect("haiku alias should match");
        assert_eq!(model.provider, ProviderKind::Anthropic);
    }

    #[test]
    fn defaults_per_provider() {
        assert!(ModelRegistry::default_for_provider(ProviderKind::Anthropic).is_some());
        assert!(ModelRegistry::default_for_provider(ProviderKind::OpenAi).is_some());
        assert!(ModelRegistry::default_for_provider(ProviderKind::Google).is_some());
        assert!(ModelRegistry::default_for_provider(ProviderKind::XAi).is_some());
    }

    #[test]
    fn pricing_lookup_works() {
        let pricing = pricing_for_model("claude-haiku-4-5-20251001");
        assert!(pricing.is_some());
        let p = pricing.unwrap();
        assert!((p.input_cost_per_million - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn max_tokens_lookup() {
        assert_eq!(max_tokens_for_model("claude-opus-4-5-20250520"), 32_000);
        assert_eq!(max_tokens_for_model("gpt-4.1"), 32_768);
        assert_eq!(max_tokens_for_model("unknown-model"), 16_384);
    }
}
