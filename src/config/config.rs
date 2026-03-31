//! 新版应用配置模型
//!
//! 替代旧的 AppConfig，采用 Provider-Model 分层架构

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::provider::{ActiveModel, Provider};

/// 新版应用配置（替代旧版 AppConfig）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Config {
    /// 供应商列表
    #[serde(default)]
    pub providers: Vec<Provider>,
    /// 每个 tool 当前激活的模型 { tool_name -> ActiveModel }
    #[serde(default)]
    pub active: HashMap<String, ActiveModel>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加供应商
    pub fn add_provider(&mut self, provider: Provider) -> anyhow::Result<()> {
        if self.providers.iter().any(|p| p.name == provider.name) {
            anyhow::bail!("供应商 '{}' 已存在", provider.name);
        }
        self.providers.push(provider);
        Ok(())
    }

    /// 获取供应商
    pub fn get_provider(&self, name: &str) -> Option<&Provider> {
        self.providers.iter().find(|p| p.name == name)
    }

    /// 获取可变引用的供应商
    pub fn get_provider_mut(&mut self, name: &str) -> Option<&mut Provider> {
        self.providers.iter_mut().find(|p| p.name == name)
    }

    /// 删除供应商
    pub fn remove_provider(&mut self, name: &str) -> anyhow::Result<()> {
        let index = self
            .providers
            .iter()
            .position(|p| p.name == name)
            .ok_or_else(|| anyhow::anyhow!("供应商 '{}' 不存在", name))?;

        self.providers.remove(index);
        // 清理相关的 active 映射
        self.active.retain(|_, am| am.provider != name);
        Ok(())
    }

    /// 更新活跃模型
    pub fn set_active(&mut self, tool: &str, provider: String, model: String) {
        self.active
            .insert(tool.to_string(), ActiveModel::new(provider, model));
    }

    /// 从旧版 AppConfig 格式迁移
    pub fn migrate_from_legacy(
        old_models: &[super::models::ModelConfig],
        old_active: &HashMap<String, String>,
    ) -> Self {
        let mut providers = Vec::new();

        for mc in old_models {
            let protocol = if mc.base_url.contains("anthropic")
                || mc.name.contains("anthropic")
                || mc.name.contains("claude")
            {
                super::provider::Protocol::Anthropic
            } else {
                super::provider::Protocol::OpenAI
            };

            let provider = Provider::new(
                mc.name.clone(),
                mc.base_url.clone(),
                mc.api_key.clone(),
                protocol,
                mc.models.clone(),
            );
            providers.push(provider);
        }

        // 旧格式 active_models: { tool -> model_config_name }
        // 新格式: { tool -> ActiveModel { provider, model } }
        let mut active = HashMap::new();
        for (tool, model_config_name) in old_active {
            // 在旧的 model config 中找到对应条目
            if let Some(mc) = old_models.iter().find(|m| m.name == *model_config_name) {
                let model = mc.get_default_model().unwrap_or("unknown").to_string();
                active.insert(
                    tool.clone(),
                    ActiveModel::new(model_config_name.clone(), model),
                );
            }
        }

        Self { providers, active }
    }
}
