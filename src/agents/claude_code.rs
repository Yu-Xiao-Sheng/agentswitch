use crate::agents::adapter::{AgentAdapter, Backup};
use crate::config::ModelConfig;
use anyhow::{Context, Result};
use std::path::PathBuf;

/// Adapter for Claude Code CLI
pub struct ClaudeCodeAdapter;

impl ClaudeCodeAdapter {
    pub fn new() -> Self {
        Self
    }

    fn config_dir(&self) -> Result<PathBuf> {
        Ok(dirs::home_dir()
            .context("Could not find home directory")?
            .join(".claude"))
    }
}

impl AgentAdapter for ClaudeCodeAdapter {
    fn name(&self) -> &str {
        "claude-code"
    }

    fn detect(&self) -> Result<bool> {
        // 检查可执行文件是否存在（在 PATH 中）
        let in_path = which::which("claude").is_ok();

        // 检查配置目录是否存在
        let config_dir = self.config_dir()?;
        let has_config = config_dir.exists();

        Ok(in_path || has_config)
    }

    fn config_path(&self) -> Result<PathBuf> {
        Ok(self.config_dir()?.join("config.json"))
    }

    fn backup(&self) -> Result<Backup> {
        let config_path = self.config_path()?;
        let backup_dir = dirs::home_dir()
            .context("Could not find home directory")?
            .join(".agentswitch")
            .join("backups")
            .join("claude-code");

        std::fs::create_dir_all(&backup_dir).context("Failed to create backup directory")?;

        let timestamp = chrono::Utc::now();
        let backup_filename = format!("backup-{}.json", timestamp.format("%Y%m%d-%H%M%S"));
        let backup_path = backup_dir.join(&backup_filename);

        if config_path.exists() {
            std::fs::copy(&config_path, &backup_path).context("Failed to backup configuration")?;
        }

        Ok(Backup {
            agent_name: self.name().to_string(),
            original_config_path: config_path,
            backup_path,
            timestamp,
        })
    }

    fn apply(&self, _model_config: &ModelConfig) -> Result<()> {
        // TODO: Implement Claude Code configuration
        anyhow::bail!("Claude Code adapter is not yet implemented")
    }

    fn restore(&self, backup: &Backup) -> Result<()> {
        if backup.backup_path.exists() {
            std::fs::copy(&backup.backup_path, &backup.original_config_path)
                .context("Failed to restore backup")?;
        }
        Ok(())
    }

    fn current_model(&self) -> Result<Option<String>> {
        let config_path = self.config_path()?;

        if !config_path.exists() {
            return Ok(None);
        }

        // TODO: Implement actual config parsing when adapter is implemented
        Ok(None)
    }
}
