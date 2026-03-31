//! Claude Code 适配器
//!
//! Claude Code 是 Anthropic 官方的 CLI 编码工具。
//! 配置文件位置：
//! - 设置文件: ~/.claude/settings.json
//! - 配置文件: ~/.claude/config.json
//!
//! 支持的配置方式：
//! 1. settings.json 中的 env 字段设置环境变量
//! 2. 直接设置环境变量
//!
//! 配置示例：
//! ```json
//! {
//!   "env": {
//!     "ANTHROPIC_BASE_URL": "https://api.example.com",
//!     "ANTHROPIC_API_KEY": "your-api-key",
//!     "ANTHROPIC_MODEL": "claude-sonnet-4"
//!   },
//!   "model": "claude-sonnet-4"
//! }
//! ```

use crate::agents::adapter::{AgentAdapter, Backup};
use crate::config::Provider;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Claude Code 设置文件结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct ClaudeCodeSettings {
    /// 环境变量配置
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<HashMap<String, String>>,

    /// 默认模型
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,

    /// 其他配置字段（保留）
    #[serde(flatten)]
    other: HashMap<String, serde_json::Value>,
}

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

    /// 获取 settings.json 路径
    fn settings_path(&self) -> Result<PathBuf> {
        Ok(self.config_dir()?.join("settings.json"))
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
        // 返回 settings.json 作为主配置文件
        self.settings_path()
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
        } else {
            // 如果配置文件不存在，创建一个空的备份标记
            std::fs::write(&backup_path, "{}").context("Failed to create empty backup")?;
        }

        Ok(Backup {
            agent_name: self.name().to_string(),
            original_config_path: config_path,
            backup_path,
            timestamp,
        })
    }

    fn apply(&self, provider: &Provider, model: &str) -> Result<()> {
        let config_dir = self.config_dir()?;

        // 创建配置目录（如果不存在）
        fs::create_dir_all(&config_dir).context("Failed to create config directory")?;

        // 读取或创建 settings.json
        let settings_path = self.settings_path()?;
        let mut settings = if settings_path.exists() {
            let content =
                fs::read_to_string(&settings_path).context("Failed to read settings.json")?;
            serde_json::from_str::<ClaudeCodeSettings>(&content)
                .context("Failed to parse settings.json")?
        } else {
            ClaudeCodeSettings::default()
        };

        // 构建环境变量配置
        let env = settings.env.get_or_insert_with(HashMap::new);

        // 设置 API 基础 URL
        // Claude Code 使用 ANTHROPIC_BASE_URL 环境变量
        env.insert(
            "ANTHROPIC_BASE_URL".to_string(),
            provider.base_url.clone(),
        );

        // 设置 API Key
        // Claude Code 支持 ANTHROPIC_API_KEY 或 ANTHROPIC_AUTH_TOKEN
        env.insert("ANTHROPIC_API_KEY".to_string(), provider.api_key.clone());

        // 设置模型
        // Claude Code 使用 ANTHROPIC_MODEL 环境变量
        env.insert("ANTHROPIC_MODEL".to_string(), model.to_string());

        // 更新模型设置
        settings.model = Some(model.to_string());

        // 写回配置文件
        let content =
            serde_json::to_string_pretty(&settings).context("Failed to serialize settings.json")?;
        fs::write(&settings_path, content).context("Failed to write settings.json")?;

        Ok(())
    }

    fn restore(&self, backup: &Backup) -> Result<()> {
        if backup.backup_path.exists() {
            // 读取备份内容
            let backup_content =
                fs::read_to_string(&backup.backup_path).context("Failed to read backup file")?;

            // 如果备份是空的，删除原配置文件
            if backup_content.trim() == "{}" {
                if backup.original_config_path.exists() {
                    std::fs::remove_file(&backup.original_config_path)
                        .context("Failed to remove config file")?;
                }
            } else {
                std::fs::copy(&backup.backup_path, &backup.original_config_path)
                    .context("Failed to restore backup")?;
            }
        }
        Ok(())
    }

    fn current_model(&self) -> Result<Option<String>> {
        let settings_path = self.settings_path()?;

        if !settings_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&settings_path).context("Failed to read settings.json")?;
        let settings: ClaudeCodeSettings =
            serde_json::from_str(&content).context("Failed to parse settings.json")?;

        // 优先从 env.ANTHROPIC_MODEL 获取
        if let Some(env) = &settings.env {
            if let Some(model) = env.get("ANTHROPIC_MODEL") {
                return Ok(Some(model.clone()));
            }
        }

        // 否则从 model 字段获取
        Ok(settings.model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_code_adapter_creation() {
        let adapter = ClaudeCodeAdapter::new();
        assert_eq!(adapter.name(), "claude-code");
    }

    #[test]
    fn test_settings_structure() {
        let settings = ClaudeCodeSettings {
            env: Some({
                let mut env = HashMap::new();
                env.insert("ANTHROPIC_BASE_URL".to_string(), "https://api.example.com".to_string());
                env.insert("ANTHROPIC_API_KEY".to_string(), "sk-test".to_string());
                env.insert("ANTHROPIC_MODEL".to_string(), "claude-sonnet-4".to_string());
                env
            }),
            model: Some("claude-sonnet-4".to_string()),
            other: HashMap::new(),
        };

        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("ANTHROPIC_BASE_URL"));
        assert!(json.contains("ANTHROPIC_MODEL"));
    }
}
