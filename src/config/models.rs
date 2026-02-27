use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Model configuration for an OpenAI-compatible API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Unique identifier for this configuration
    pub name: String,

    /// Base URL for the API
    pub base_url: String,

    /// API key for authentication
    pub api_key: String,

    /// Model ID to use
    pub model_id: String,

    /// Additional parameters (optional)
    #[serde(default)]
    pub extra_params: HashMap<String, serde_json::Value>,
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// All configured models
    pub models: Vec<ModelConfig>,

    /// Current active model for each agent
    pub active_models: HashMap<String, String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            models: Vec::new(),
            active_models: HashMap::new(),
        }
    }
}
