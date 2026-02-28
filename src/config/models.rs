//! 数据模型定义

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelConfig {
    pub name: String,
    pub base_url: String,
    pub api_key: String,
    pub model_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<HashMap<String, Value>>,
}

impl ModelConfig {
    pub fn new(name: String, base_url: String, api_key: String, model_id: String) -> Self {
        Self {
            name,
            base_url,
            api_key,
            model_id,
            extra_params: None,
        }
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
