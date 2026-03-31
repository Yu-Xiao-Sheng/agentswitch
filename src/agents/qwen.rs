use crate::agents::adapter::{AgentAdapter, Backup};
use crate::config::ModelConfig;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Qwen config.json 结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct QwenConfig {
    #[serde(default)]
    api_base_url: Option<String>,
    #[serde(default)]
    model_id: Option<String>,
}

/// Qwen auth.json 结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct QwenAuth {
    #[serde(rename = "DASHSCOPE_API_KEY")]
    dashscope_api_key: Option<String>,
}

/// Adapter for Qwen CLI (Alibaba's Tongyi Qianwen)
pub struct QwenAdapter;

impl QwenAdapter {
    pub fn new() -> Self {
        Self
    }

    fn config_dir(&self) -> Result<PathBuf> {
        Ok(dirs::home_dir()
            .context("Could not find home directory")?
            .join(".qwen"))
    }
}

impl AgentAdapter for QwenAdapter {
    fn name(&self) -> &str {
        "qwen"
    }

    fn detect(&self) -> Result<bool> {
        // 检查可执行文件是否存在（在 PATH 中）
        let in_path = which::which("qwen").is_ok();

        // 检查配置文件是否存在
        let config_dir = self.config_dir()?;
        let has_config = config_dir.exists() && config_dir.join("config.json").exists();

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
            .join("qwen");

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
        let config_dir = self.config_dir()?;

        // 创建配置目录（如果不存在）
        fs::create_dir_all(&config_dir).context("创建配置目录失败")?;

        // 处理 config.json
        let config_path = config_dir.join("config.json");
        let mut config = if config_path.exists() {
            let content = fs::read_to_string(&config_path).context("读取 config.json 失败")?;
            serde_json::from_str::<QwenConfig>(&content).context("解析 config.json 失败")?
        } else {
            QwenConfig::default()
        };

        // 更新配置
        config.api_base_url = Some(model_config.base_url.clone());
        config.model_id = Some(model_config.get_default_model().unwrap_or("").to_string());

        // 写回 config.json
        let content = serde_json::to_string_pretty(&config).context("序列化 config.json 失败")?;
        fs::write(&config_path, content).context("写入 config.json 失败")?;

        // 处理 auth.json
        let auth_path = config_dir.join("auth.json");
        let mut auth = if auth_path.exists() {
            let content = fs::read_to_string(&auth_path).context("读取 auth.json 失败")?;
            serde_json::from_str::<QwenAuth>(&content).context("解析 auth.json 失败")?
        } else {
            QwenAuth::default()
        };

        auth.dashscope_api_key = Some(model_config.api_key.clone());

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

        let config: QwenConfig = serde_json::from_str(&content).context("解析配置文件失败")?;

        Ok(config.model_id.map(|s| s.to_string()))
    }
}
