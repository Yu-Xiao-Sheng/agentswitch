//! 数据模型定义

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelConfig {
    pub name: String,
    pub base_url: String,
    pub api_key: String,
    #[serde(default)]
    pub models: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<HashMap<String, Value>>,
    // 向后兼容：支持旧的 model_id 字段
    #[serde(skip_serializing_if = "Option::is_none", rename = "model_id")]
    pub legacy_model_id: Option<String>,
}

impl ModelConfig {
    pub fn new(name: String, base_url: String, api_key: String, models: Vec<String>) -> Self {
        let default_model = models.first().cloned();
        Self {
            name,
            base_url,
            api_key,
            models,
            default_model,
            extra_params: None,
            legacy_model_id: None,
        }
    }

    /// 从旧的 model_id 格式迁移
    pub fn migrate_from_legacy(&mut self) {
        if let Some(model_id) = self.legacy_model_id.take() {
            if self.models.is_empty() {
                self.models.push(model_id);
            }
            if self.default_model.is_none() && !self.models.is_empty() {
                self.default_model = Some(self.models[0].clone());
            }
        }
    }

    /// 获取默认模型
    pub fn get_default_model(&self) -> Option<&str> {
        self.default_model
            .as_deref()
            .or_else(|| self.models.first().map(|s| s.as_str()))
    }

    /// 验证配置
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.models.is_empty() {
            anyhow::bail!("至少需要配置一个模型");
        }

        if let Some(ref default) = self.default_model {
            if !self.models.contains(default) {
                anyhow::bail!(
                    "默认模型 '{}' 不在模型列表中",
                    default
                );
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub models: Vec<ModelConfig>,
    #[serde(default)]
    pub active_models: HashMap<String, String>,
}

impl AppConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_model(&mut self, model: ModelConfig) -> anyhow::Result<()> {
        if self.models.iter().any(|m| m.name == model.name) {
            anyhow::bail!("模型名称 '{}' 已存在", model.name);
        }
        self.models.push(model);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_model(&self, name: &str) -> Option<&ModelConfig> {
        self.models.iter().find(|m| m.name == name)
    }

    #[allow(dead_code)]
    pub fn get_model_mut(&mut self, name: &str) -> Option<&mut ModelConfig> {
        self.models.iter_mut().find(|m| m.name == name)
    }

    pub fn remove_model(&mut self, name: &str) -> anyhow::Result<()> {
        let index = self
            .models
            .iter()
            .position(|m| m.name == name)
            .ok_or_else(|| anyhow::anyhow!("模型 '{}' 不存在", name))?;

        self.models.remove(index);
        self.active_models
            .retain(|_, model_name| model_name != name);
        Ok(())
    }

    pub fn edit_model<F>(&mut self, name: &str, updater: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut ModelConfig) -> anyhow::Result<()>,
    {
        let model = self
            .models
            .iter_mut()
            .find(|m| m.name == name)
            .ok_or_else(|| anyhow::anyhow!("模型 '{}' 不存在", name))?;

        updater(model)?;
        Ok(())
    }
}
