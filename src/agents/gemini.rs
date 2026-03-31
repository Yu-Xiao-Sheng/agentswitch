use crate::agents::adapter::{AgentAdapter, Backup};
use crate::config::Provider;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// Gemini CLI settings.json 结构
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
#[allow(non_snake_case)]
struct GeminiSettings {
    #[serde(default)]
    defaultModel: Option<GeminiModel>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct GeminiModel {
    #[serde(rename = "apiBaseUrl")]
    api_base_url: Option<String>,
    #[serde(rename = "modelId")]
    model_id: Option<String>,
}

/// Adapter for Gemini CLI (Google's code assistant)
pub struct GeminiAdapter;

impl GeminiAdapter {
    pub fn new() -> Self {
        Self
    }

    fn config_dir(&self) -> Result<PathBuf> {
        // Gemini CLI 使用 ~/.gemini/
        Ok(dirs::home_dir()
            .context("Could not find home directory")?
            .join(".gemini"))
    }
}

impl AgentAdapter for GeminiAdapter {
    fn name(&self) -> &str {
        "gemini-cli"
    }

    fn detect(&self) -> Result<bool> {
        // 检查可执行文件是否存在（在 PATH 中）
        let in_path = which::which("gemini").is_ok();

        // 检查配置文件是否存在
        let config_path = self.config_path();
        let has_config = config_path.is_ok() && config_path.unwrap().exists();

        Ok(in_path || has_config)
    }

    fn config_path(&self) -> Result<PathBuf> {
        // Gemini CLI 使用 ~/.gemini/settings.json
        Ok(self.config_dir()?.join("settings.json"))
    }

    fn backup(&self) -> Result<Backup> {
        let config_path = self.config_path()?;
        let backup_dir = dirs::home_dir()
            .context("Could not find home directory")?
            .join(".agentswitch")
            .join("backups")
            .join("gemini-cli");

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

    fn apply(&self, provider: &Provider, model: &str) -> Result<()> {
        let config_dir = self.config_dir()?;

        // 创建配置目录（如果不存在）
        fs::create_dir_all(&config_dir).context("创建配置目录失败")?;

        // 更新 settings.json
        let settings_path = config_dir.join("settings.json");
        let mut settings = if settings_path.exists() {
            let content = fs::read_to_string(&settings_path).context("读取 settings.json 失败")?;
            serde_json::from_str::<GeminiSettings>(&content).context("解析 settings.json 失败")?
        } else {
            GeminiSettings::default()
        };

        settings.defaultModel = Some(GeminiModel {
            api_base_url: Some(provider.base_url.clone()),
            model_id: Some(model.to_string()),
        });

        let content =
            serde_json::to_string_pretty(&settings).context("序列化 settings.json 失败")?;
        fs::write(&settings_path, content).context("写入 settings.json 失败")?;

        // 更新 .env 文件
        let env_path = config_dir.join(".env");
        let env_content = format!(
            "GOOGLE_GEMINI_BASE_URL={}\nGEMINI_API_KEY={}\nGEMINI_MODEL={}\n",
            provider.base_url, provider.api_key, model
        );
        fs::write(&env_path, env_content).context("写入 .env 失败")?;

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

        let settings: GeminiSettings =
            serde_json::from_str(&content).context("解析配置文件失败")?;

        // 从 defaultModel 读取模型 ID
        Ok(settings
            .defaultModel
            .and_then(|m| m.model_id)
            .map(|s| s.to_string()))
    }
}
