use crate::agents::adapter::{AgentAdapter, Backup};
use crate::config::ModelConfig;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Codex config.toml 结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CodexConfig {
    #[serde(default)]
    custom_provider: Option<CodexCustomProvider>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CodexCustomProvider {
    #[serde(default)]
    base_url: Option<String>,
    #[serde(default)]
    model: Option<String>,
}

/// Codex auth.json 结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CodexAuth {
    #[serde(rename = "OPENAI_API_KEY")]
    openai_api_key: Option<String>,
}

/// Adapter for Codex (OpenAI's code assistant CLI)
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
        // 检查可执行文件是否存在（在 PATH 中）
        let in_path = which::which("codex").is_ok();

        // 检查配置文件是否存在
        let config_dir = self.config_dir()?;
        let has_config = config_dir.exists() && config_dir.join("config.toml").exists();

        Ok(in_path || has_config)
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

        std::fs::copy(&config_path, &backup_path).context("Failed to backup configuration")?;

        Ok(Backup {
            agent_name: self.name().to_string(),
            original_config_path: config_path,
            backup_path,
            timestamp,
        })
    }

    fn apply(&self, model_config: &ModelConfig) -> Result<()> {
        let config_dir = self.config_dir()?;

        // 创建配置目录（如果不存在）
        fs::create_dir_all(&config_dir).context("创建配置目录失败")?;

        // 处理 config.toml
        let config_path = config_dir.join("config.toml");
        let mut config = if config_path.exists() {
            let content = fs::read_to_string(&config_path).context("读取 config.toml 失败")?;
            toml::from_str::<CodexConfig>(&content).context("解析 config.toml 失败")?
        } else {
            CodexConfig::default()
        };

        // 更新 custom_provider 配置
        config.custom_provider = Some(CodexCustomProvider {
            base_url: Some(model_config.base_url.clone()),
            model: Some(model_config.model_id.clone()),
        });

        // 写回 config.toml
        let content = toml::to_string_pretty(&config).context("序列化 config.toml 失败")?;
        fs::write(&config_path, content).context("写入 config.toml 失败")?;

        // 处理 auth.json
        let auth_path = config_dir.join("auth.json");
        let mut auth = if auth_path.exists() {
            let content = fs::read_to_string(&auth_path).context("读取 auth.json 失败")?;
            serde_json::from_str::<CodexAuth>(&content).context("解析 auth.json 失败")?
        } else {
            CodexAuth::default()
        };

        auth.openai_api_key = Some(model_config.api_key.clone());

        // 写回 auth.json
        let content = serde_json::to_string_pretty(&auth).context("序列化 auth.json 失败")?;
        fs::write(&auth_path, content).context("写入 auth.json 失败")?;

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

        let config: CodexConfig = toml::from_str(&content).context("解析配置文件失败")?;

        // 从 custom_provider 读取模型 ID
        Ok(config
            .custom_provider
            .and_then(|p| p.model)
            .map(|s| s.to_string()))
    }
}
