use crate::agents::adapter::{AgentAdapter, Backup};
use crate::config::ModelConfig;
use anyhow::{Context, Result};
use serde_json::json;
use std::fs;
use std::path::PathBuf;

/// Claude Code 配置文件结构
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct ClaudeCodeConfig {
    #[serde(default)]
    env: serde_json::Map<String, serde_json::Value>,
}

impl Default for ClaudeCodeConfig {
    fn default() -> Self {
        Self {
            env: serde_json::Map::new(),
        }
    }
}

/// Adapter for Claude Code (Anthropic's official CLI tool)
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

        // 检查配置文件是否存在
        let config_path = self.config_path();
        let has_config = config_path.is_ok() && config_path.unwrap().exists();

        Ok(in_path || has_config)
    }

    fn config_path(&self) -> Result<PathBuf> {
        // Claude Code 使用 ~/.claude/settings.json
        Ok(self.config_dir()?.join("settings.json"))
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

        std::fs::copy(&config_path, &backup_path).context("Failed to backup configuration")?;

        Ok(Backup {
            agent_name: self.name().to_string(),
            original_config_path: config_path,
            backup_path,
            timestamp,
        })
    }

    fn apply(&self, model_config: &ModelConfig) -> Result<()> {
        let config_path = self.config_path()?;

        // 读取或创建配置文件
        let mut config = if config_path.exists() {
            let content = fs::read_to_string(&config_path).context("读取配置文件失败")?;
            serde_json::from_str::<ClaudeCodeConfig>(&content).context("解析配置文件失败")?
        } else {
            // 配置文件不存在，创建默认配置
            ClaudeCodeConfig::default()
        };

        // 应用模型配置到 env 字段
        config.env.insert(
            "ANTHROPIC_AUTH_TOKEN".to_string(),
            json!(model_config.api_key.clone()),
        );
        config.env.insert(
            "ANTHROPIC_BASE_URL".to_string(),
            json!(model_config.base_url.clone()),
        );
        config.env.insert(
            "ANTHROPIC_MODEL".to_string(),
            json!(model_config.model_id.clone()),
        );

        // 写回配置文件
        let content = serde_json::to_string_pretty(&config).context("序列化配置失败")?;

        fs::write(&config_path, content).context("写入配置文件失败")?;

        Ok(())
    }

    fn restore(&self, backup: &Backup) -> Result<()> {
        std::fs::copy(&backup.backup_path, &backup.original_config_path)
            .context("Failed to restore backup")?;
        Ok(())
    }

    fn current_model(&self) -> Result<Option<String>> {
        let config_path = self.config_path()?;

        if !config_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&config_path).context("读取配置文件失败")?;

        let config: ClaudeCodeConfig =
            serde_json::from_str(&content).context("解析配置文件失败")?;

        // 从 env 字段读取模型 ID
        Ok(config
            .env
            .get("ANTHROPIC_MODEL")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()))
    }
}
