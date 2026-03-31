//! OpenCode 适配器
//!
//! OpenCode 是一个开源的编码 CLI 工具，支持多种 LLM 供应商。
//! 配置文件位置：
//! - 全局配置: ~/.config/opencode/opencode.json
//! - 项目配置: ./opencode.json
//! - 凭证: ~/.local/share/opencode/auth.json
//!
//! 支持的协议：
//! - OpenAI 兼容协议（通过 @ai-sdk/openai-compatible）
//! - Anthropic 协议
//!
//! 配置示例：
//! ```json
//! {
//!   "$schema": "https://opencode.ai/config.json",
//!   "model": "custom-provider/model-id",
//!   "provider": {
//!     "custom-provider": {
//!       "npm": "@ai-sdk/openai-compatible",
//!       "name": "Custom Provider",
//!       "options": {
//!         "baseURL": "https://api.example.com/v1"
//!       },
//!       "models": {
//!         "model-id": {
//!           "name": "Model Display Name"
//!         }
//!       }
//!     }
//!   }
//! }
//! ```

use crate::agents::adapter::{AgentAdapter, Backup};
use crate::config::ModelConfig;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// OpenCode 配置文件结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct OpenCodeConfig {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    schema: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<HashMap<String, OpenCodeProvider>>,
}

/// OpenCode 供应商配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenCodeProvider {
    /// NPM 包名（用于自定义供应商）
    #[serde(skip_serializing_if = "Option::is_none")]
    npm: Option<String>,

    /// 显示名称
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// 供应商选项
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OpenCodeProviderOptions>,

    /// 模型配置
    #[serde(skip_serializing_if = "Option::is_none")]
    models: Option<HashMap<String, OpenCodeModel>>,
}

/// OpenCode 供应商选项
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenCodeProviderOptions {
    /// API 基础 URL
    #[serde(rename = "baseURL", skip_serializing_if = "Option::is_none")]
    base_url: Option<String>,

    /// API Key（可以使用环境变量引用）
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<String>,

    /// 自定义请求头
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<HashMap<String, String>>,
}

/// OpenCode 模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenCodeModel {
    /// 模型显示名称
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// 模型限制
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<OpenCodeModelLimit>,
}

/// OpenCode 模型限制
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenCodeModelLimit {
    /// 上下文长度
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<u32>,

    /// 输出长度
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<u32>,
}

/// OpenCode 认证文件结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct OpenCodeAuth {
    /// 供应商凭证映射
    #[serde(flatten)]
    providers: HashMap<String, OpenCodeAuthProvider>,
}

/// OpenCode 供应商认证信息
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenCodeAuthProvider {
    /// API Key
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<String>,
}

/// Adapter for OpenCode (开源编码 CLI 工具)
pub struct OpenCodeAdapter {
    /// 自定义供应商名称
    provider_name: String,
}

impl OpenCodeAdapter {
    /// 创建新的 OpenCode 适配器
    pub fn new() -> Self {
        Self {
            provider_name: "custom".to_string(),
        }
    }

    /// 创建带有自定义供应商名称的适配器
    pub fn with_provider_name(provider_name: &str) -> Self {
        Self {
            provider_name: provider_name.to_string(),
        }
    }

    /// 获取配置目录
    fn config_dir(&self) -> Result<PathBuf> {
        Ok(dirs::home_dir()
            .context("无法找到用户主目录")?
            .join(".config")
            .join("opencode"))
    }

    /// 获取认证目录
    fn auth_dir(&self) -> Result<PathBuf> {
        Ok(dirs::home_dir()
            .context("无法找到用户主目录")?
            .join(".local")
            .join("share")
            .join("opencode"))
    }

    /// 获取认证文件路径
    fn auth_path(&self) -> Result<PathBuf> {
        Ok(self.auth_dir()?.join("auth.json"))
    }
}

impl AgentAdapter for OpenCodeAdapter {
    fn name(&self) -> &str {
        "opencode"
    }

    fn detect(&self) -> Result<bool> {
        // 检查可执行文件是否存在（在 PATH 中）
        let in_path = which::which("opencode").is_ok();

        // 检查配置文件是否存在
        let config_path = self.config_path();
        let has_global_config = config_path.is_ok() && config_path.unwrap().exists();

        // 检查项目级配置文件
        let has_project_config = std::env::current_dir()
            .map(|cwd| cwd.join("opencode.json").exists())
            .unwrap_or(false);

        Ok(in_path || has_global_config || has_project_config)
    }

    fn config_path(&self) -> Result<PathBuf> {
        Ok(self.config_dir()?.join("opencode.json"))
    }

    fn backup(&self) -> Result<Backup> {
        let config_path = self.config_path()?;
        let backup_dir = dirs::home_dir()
            .context("无法找到用户主目录")?
            .join(".agentswitch")
            .join("backups")
            .join("opencode");

        std::fs::create_dir_all(&backup_dir).context("创建备份目录失败")?;

        let timestamp = chrono::Utc::now();
        let backup_filename = format!("backup-{}.json", timestamp.format("%Y%m%d-%H%M%S"));
        let backup_path = backup_dir.join(&backup_filename);

        if config_path.exists() {
            std::fs::copy(&config_path, &backup_path).context("备份配置文件失败")?;
        } else {
            // 如果配置文件不存在，创建一个空的备份标记
            std::fs::write(&backup_path, "{}").context("创建空备份失败")?;
        }

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

        // 读取或创建配置文件
        let config_path = config_dir.join("opencode.json");
        let mut config = if config_path.exists() {
            let content = fs::read_to_string(&config_path).context("读取 opencode.json 失败")?;
            serde_json::from_str::<OpenCodeConfig>(&content).context("解析 opencode.json 失败")?
        } else {
            OpenCodeConfig {
                schema: Some("https://opencode.ai/config.json".to_string()),
                ..Default::default()
            }
        };

        // 构建自定义供应商配置
        let model_name = OpenCodeModel {
            name: Some(model_config.name.clone()),
            limit: Some(OpenCodeModelLimit {
                context: Some(128000),
                output: Some(16384),
            }),
        };

        let provider_options = OpenCodeProviderOptions {
            base_url: Some(model_config.base_url.clone()),
            api_key: Some(format!(
                "{{env:OPENCODE_{}_API_KEY}}",
                self.provider_name.to_uppercase()
            )),
            headers: None,
        };

        let mut models = HashMap::new();
        let default_model = model_config.get_default_model().unwrap_or("");
        models.insert(default_model.to_string(), model_name);

        let provider = OpenCodeProvider {
            npm: Some("@ai-sdk/openai-compatible".to_string()),
            name: Some(format!("{} (Custom)", model_config.name)),
            options: Some(provider_options),
            models: Some(models),
        };

        // 更新配置
        let providers = config.provider.get_or_insert_with(HashMap::new);
        providers.insert(self.provider_name.clone(), provider);

        // 设置默认模型
        config.model = Some(format!("{}/{}", self.provider_name, default_model));

        // 写回配置文件
        let content = serde_json::to_string_pretty(&config).context("序列化 opencode.json 失败")?;
        fs::write(&config_path, content).context("写入 opencode.json 失败")?;

        // 更新认证文件
        let auth_dir = self.auth_dir()?;
        fs::create_dir_all(&auth_dir).context("创建认证目录失败")?;

        let auth_path = self.auth_path()?;
        let mut auth = if auth_path.exists() {
            let content = fs::read_to_string(&auth_path).context("读取 auth.json 失败")?;
            serde_json::from_str::<OpenCodeAuth>(&content).context("解析 auth.json 失败")?
        } else {
            OpenCodeAuth::default()
        };

        // 添加供应商认证信息
        auth.providers.insert(
            self.provider_name.clone(),
            OpenCodeAuthProvider {
                api_key: Some(model_config.api_key.clone()),
            },
        );

        // 写回认证文件
        let content = serde_json::to_string_pretty(&auth).context("序列化 auth.json 失败")?;
        fs::write(&auth_path, content).context("写入 auth.json 失败")?;

        Ok(())
    }

    fn restore(&self, backup: &Backup) -> Result<()> {
        if backup.backup_path.exists() {
            // 读取备份内容
            let backup_content =
                fs::read_to_string(&backup.backup_path).context("读取备份文件失败")?;

            // 如果备份是空的，删除原配置文件
            if backup_content.trim() == "{}" {
                if backup.original_config_path.exists() {
                    std::fs::remove_file(&backup.original_config_path)
                        .context("删除配置文件失败")?;
                }
            } else {
                std::fs::copy(&backup.backup_path, &backup.original_config_path)
                    .context("恢复备份失败")?;
            }
        }
        Ok(())
    }

    fn current_model(&self) -> Result<Option<String>> {
        let config_path = self.config_path()?;

        if !config_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&config_path).context("读取配置文件失败")?;
        let config: OpenCodeConfig = serde_json::from_str(&content).context("解析配置文件失败")?;

        // 返回当前模型
        Ok(config.model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opencode_adapter_creation() {
        let adapter = OpenCodeAdapter::new();
        assert_eq!(adapter.name(), "opencode");
    }

    #[test]
    fn test_opencode_adapter_with_provider_name() {
        let adapter = OpenCodeAdapter::with_provider_name("zhipu");
        assert_eq!(adapter.name(), "opencode");
        assert_eq!(adapter.provider_name, "zhipu");
    }

    #[test]
    fn test_config_structure() {
        let config = OpenCodeConfig {
            schema: Some("https://opencode.ai/config.json".to_string()),
            model: Some("custom/gpt-4".to_string()),
            provider: None,
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"$schema\""));
        assert!(json.contains("\"model\""));
    }
}
