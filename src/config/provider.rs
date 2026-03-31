//! Provider 数据模型定义
//!
//! Provider 代表一个 API 供应商（如 OpenAI、Anthropic 等），
//! 包含 base_url、api_key、协议类型和可用模型列表。

use serde::{Deserialize, Serialize};

use super::models::ModelConfig;

/// API 协议类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Protocol {
    /// OpenAI 兼容协议 (/v1/chat/completions)
    #[default]
    OpenAI,
    /// Anthropic 协议 (/v1/messages)
    Anthropic,
}

impl Protocol {
    pub fn as_str(&self) -> &str {
        match self {
            Protocol::OpenAI => "openai",
            Protocol::Anthropic => "anthropic",
        }
    }
}

/// Provider 结构体：代表一个 API 供应商
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Provider {
    /// 供应商名称（唯一标识符）
    pub name: String,
    /// API 基础 URL
    pub base_url: String,
    /// API Key
    pub api_key: String,
    /// 协议类型
    #[serde(default)]
    pub protocol: Protocol,
    /// 可用模型列表
    #[serde(default)]
    pub models: Vec<String>,
}

impl Provider {
    pub fn new(
        name: String,
        base_url: String,
        api_key: String,
        protocol: Protocol,
        models: Vec<String>,
    ) -> Self {
        Self {
            name,
            base_url,
            api_key,
            protocol,
            models,
        }
    }

    /// 获取默认模型
    pub fn get_default_model(&self) -> Option<&str> {
        self.models.first().map(|s| s.as_str())
    }

    /// 检查是否包含指定模型
    pub fn has_model(&self, model_name: &str) -> bool {
        self.models.iter().any(|m| m == model_name)
    }

    /// 将 Provider + model_name 解析为 ModelConfig（供 AgentAdapter 使用）
    pub fn resolve_model_config(&self, model_name: &str) -> anyhow::Result<ModelConfig> {
        if !self.has_model(model_name) {
            anyhow::bail!(
                "模型 '{}' 不在供应商 '{}' 的模型列表中",
                model_name,
                self.name
            );
        }

        Ok(ModelConfig::new(
            format!("{}:{}", self.name, model_name),
            self.base_url.clone(),
            self.api_key.clone(),
            vec![model_name.to_string()],
        ))
    }

    /// 验证配置
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.is_empty() {
            anyhow::bail!("供应商名称不能为空");
        }
        if self.base_url.is_empty() {
            anyhow::bail!("Base URL 不能为空");
        }
        if self.api_key.is_empty() {
            anyhow::bail!("API Key 不能为空");
        }
        if self.models.is_empty() {
            anyhow::bail!("至少需要配置一个模型");
        }
        Ok(())
    }
}

/// 活跃模型配置：记录每个 tool 当前使用的 provider + model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActiveModel {
    /// 供应商名称
    pub provider: String,
    /// 模型名称
    pub model: String,
}

impl ActiveModel {
    pub fn new(provider: String, model: String) -> Self {
        Self { provider, model }
    }
}
