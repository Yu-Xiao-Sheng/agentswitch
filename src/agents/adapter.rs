use anyhow::Result;
use std::path::PathBuf;
use crate::config::ModelConfig;

/// Represents a backup of an agent's configuration
#[derive(Debug, Clone)]
pub struct Backup {
    pub agent_name: String,
    pub original_config_path: PathBuf,
    pub backup_path: PathBuf,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Trait for adapting different code agent tools
pub trait AgentAdapter {
    /// Get the name of this agent tool
    fn name(&self) -> &str;

    /// Get the display name of this agent tool
    fn display_name(&self) -> &str;

    /// Detect if this agent tool is installed
    fn detect(&self) -> Result<bool>;

    /// Get the path to the agent's configuration file
    fn config_path(&self) -> Result<PathBuf>;

    /// Create a backup of the current configuration
    fn backup(&self) -> Result<Backup>;

    /// Apply a model configuration to this agent
    fn apply(&self, model_config: &ModelConfig) -> Result<()>;

    /// Restore a configuration from a backup
    fn restore(&self, backup: &Backup) -> Result<()>;

    /// Get the current model being used by this agent
    fn current_model(&self) -> Result<Option<String>>;
}
