use crate::config::{AppConfig, ModelConfig};
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// Configuration file manager
pub struct ConfigStore {
    config_dir: PathBuf,
    config_path: PathBuf,
}

impl ConfigStore {
    /// Create a new ConfigStore with default paths
    pub fn new() -> Result<Self> {
        let config_dir = dirs::home_dir()
            .context("Could not find home directory")?
            .join(".agentswitch");

        let config_path = config_dir.join("config.toml");

        Ok(Self {
            config_dir,
            config_path,
        })
    }

    /// Ensure the configuration directory exists
    pub fn ensure_config_dir(&self) -> Result<()> {
        if !self.config_dir.exists() {
            fs::create_dir_all(&self.config_dir)
                .context("Failed to create config directory")?;
        }
        Ok(())
    }

    /// Load the application configuration
    pub fn load(&self) -> Result<AppConfig> {
        if !self.config_path.exists() {
            return Ok(AppConfig::default());
        }

        let content = fs::read_to_string(&self.config_path)
            .context("Failed to read config file")?;

        let config: AppConfig = toml::from_str(&content)
            .context("Failed to parse config file")?;

        Ok(config)
    }

    /// Save the application configuration
    pub fn save(&self, config: &AppConfig) -> Result<()> {
        self.ensure_config_dir()?;

        let content = toml::to_string_pretty(config)
            .context("Failed to serialize config")?;

        fs::write(&self.config_path, content)
            .context("Failed to write config file")?;

        Ok(())
    }

    /// Add a new model configuration
    pub fn add_model(&self, model: ModelConfig) -> Result<()> {
        let mut config = self.load()?;

        if config.models.iter().any(|m| m.name == model.name) {
            anyhow::bail!("Model configuration '{}' already exists", model.name);
        }

        config.models.push(model);
        self.save(&config)?;
        Ok(())
    }

    /// Remove a model configuration
    pub fn remove_model(&self, name: &str) -> Result<()> {
        let mut config = self.load()?;

        let pos = config.models
            .iter()
            .position(|m| m.name == name)
            .context("Model configuration not found")?;

        config.models.remove(pos);
        self.save(&config)?;
        Ok(())
    }

    /// Get a model configuration by name
    pub fn get_model(&self, name: &str) -> Result<ModelConfig> {
        let config = self.load()?;
        config.models
            .into_iter()
            .find(|m| m.name == name)
            .context("Model configuration not found")
    }

    /// List all model configurations
    pub fn list_models(&self) -> Result<Vec<ModelConfig>> {
        let config = self.load()?;
        Ok(config.models)
    }
}

impl Default for ConfigStore {
    fn default() -> Self {
        Self::new().expect("Failed to create ConfigStore")
    }
}
