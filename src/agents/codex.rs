//! Codex 适配器
//!
//! **注意**: Codex 使用 OpenAI Response API（非标准 chat/completions 协议），
//! 兼容性较差，暂不支持自定义供应商切换。
//!
//! 如需使用自定义供应商，请考虑以下替代方案：
//! - claude-code: 支持 Anthropic `/v1/messages` 协议
//! - gemini-cli: 支持 OpenAI 兼容协议
//! - opencode: 支持 OpenAI 兼容协议和 Anthropic 协议
//! - qwen: 支持 OpenAI 兼容协议
//! - grok: 支持 OpenAI 兼容协议

use crate::agents::adapter::{AgentAdapter, Backup};
use crate::config::ModelConfig;
use anyhow::{bail, Context, Result};
use std::path::PathBuf;

/// Adapter for Codex (OpenAI's code assistant CLI)
///
/// **状态**: 暂不支持
/// **原因**: Codex 使用 OpenAI Response API，与标准 `/v1/chat/completions` 协议不兼容
pub struct CodexAdapter;

impl CodexAdapter {
    pub fn new() -> Self {
        Self
    }

    fn config_dir(&self) -> Result<PathBuf> {
        Ok(dirs::home_dir()
            .context("Could not find home directory")?
            .join(".codex"))
    }
}

impl AgentAdapter for CodexAdapter {
    fn name(&self) -> &str {
        "codex"
    }

    fn detect(&self) -> Result<bool> {
        // Codex 使用 Response API，暂不支持自定义供应商
        // 返回 false 以避免在可用工具列表中显示
        Ok(false)
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
            .join("codex");

        std::fs::create_dir_all(&backup_dir).context("Failed to create backup directory")?;

        let timestamp = chrono::Utc::now();
        let backup_filename = format!("backup-{}.toml", timestamp.format("%Y%m%d-%H%M%S"));
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
        bail!(
            "Codex 使用 OpenAI Response API，与标准 /v1/chat/completions 协议不兼容，暂不支持自定义供应商。\\\n\
             \\n\
             推荐替代方案：\\n\
             - claude-code: 支持 Anthropic /v1/messages 协议\\n\
             - gemini-cli: 支持 OpenAI 兼容协议\\n\
             - opencode: 支持多种协议\\n\
             - qwen: 支持 OpenAI 兼容协议\\n\
             - grok: 支持 OpenAI 兼容协议"
        )
    }

    fn restore(&self, backup: &Backup) -> Result<()> {
        if backup.backup_path.exists()
            && backup
                .original_config_path
                .parent()
                .map(|p| p.exists())
                .unwrap_or(false)
        {
            std::fs::copy(&backup.backup_path, &backup.original_config_path)
                .context("Failed to restore backup")?;
        }
        Ok(())
    }

    fn current_model(&self) -> Result<Option<String>> {
        // Codex 暂不支持，直接返回 None
        Ok(None)
    }
}
