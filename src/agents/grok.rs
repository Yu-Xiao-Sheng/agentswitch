use crate::agents::adapter::{AgentAdapter, Backup};
use crate::config::ModelConfig;
use anyhow::{Context, Result};
use std::path::PathBuf;

/// Adapter for Grok CLI (xAI's code assistant)
pub struct GrokAdapter;

impl GrokAdapter {
    pub fn new() -> Self {
        Self
    }

    fn config_dir(&self) -> Result<PathBuf> {
        Ok(dirs::home_dir()
            .context("Could not find home directory")?
            .join(".grok"))
    }
}

impl AgentAdapter for GrokAdapter {
    fn name(&self) -> &str {
        "grok"
    }


    fn detect(&self) -> Result<bool> {
        let config_dir = self.config_dir()?;
        Ok(config_dir.exists() && config_dir.join("config.toml").exists())
    }

    fn config_path(&self) -> Result<PathBuf> {
        Ok(self.config_dir()?.join("config.toml"))
    }

    fn backup(&self) -> Result<Backup> {
        let config_path = self.config_path()?;
        let backup_dir = dirs::home_dir()
            .context("Could not find home directory")?
            .join(".agentswitch")
            .join("backups")
            .join("grok");

        std::fs::create_dir_all(&backup_dir)
            .context("Failed to create backup directory")?;

        let timestamp = chrono::Utc::now();
        let backup_filename = format!("backup-{}.toml", timestamp.format("%Y%m%d-%H%M%S"));
        let backup_path = backup_dir.join(&backup_filename);

        std::fs::copy(&config_path, &backup_path)
            .context("Failed to backup configuration")?;

        Ok(Backup {
            agent_name: self.name().to_string(),
            original_config_path: config_path,
            backup_path,
            timestamp,
        })
    }

    fn apply(&self, _model_config: &ModelConfig) -> Result<()> {
        anyhow::bail!("Not implemented yet")
    }

    fn restore(&self, backup: &Backup) -> Result<()> {
        std::fs::copy(&backup.backup_path, &backup.original_config_path)
            .context("Failed to restore backup")?;
        Ok(())
    }

    fn current_model(&self) -> Result<Option<String>> {
        Ok(None)
    }
}
