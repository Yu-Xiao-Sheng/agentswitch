//! 预设应用功能

use crate::agents::AgentAdapter;
use crate::config::ModelConfig;
use crate::presets::preset::Preset;
use anyhow::Result;

/// 预设应用器
pub struct PresetAppplier {
    adapters: Vec<Box<dyn AgentAdapter>>,
}

impl PresetAppplier {
    /// 创建新的预设应用器
    pub fn new(adapters: Vec<Box<dyn AgentAdapter>>) -> Self {
        Self { adapters }
    }

    /// 应用预设到所有工具
    pub fn apply(
        &self,
        preset: &Preset,
        model_configs: &std::collections::HashMap<String, ModelConfig>,
    ) -> Result<()> {
        // 验证所有模型配置存在
        self.validate_before_apply(preset, model_configs)?;

        // 备份当前配置
        self.backup_before_apply()?;

        // 应用预设
        for (agent_name, model_name) in &preset.mappings {
            let model_config = model_configs
                .get(model_name)
                .ok_or_else(|| anyhow::anyhow!("模型配置不存在: {}", model_name))?;

            // 找到对应的适配器
            let adapter = self
                .adapters
                .iter()
                .find(|a| a.name() == agent_name.as_str())
                .ok_or_else(|| anyhow::anyhow!("工具未注册: {}", agent_name))?;

            // 应用配置
            adapter
                .apply(model_config)
                .map_err(|e| anyhow::anyhow!("应用失败: 工具={}, 原因={}", agent_name, e))?;
        }

        Ok(())
    }

    /// 应用预设到指定工具列表
    pub fn apply_to_agents(
        &self,
        preset: &Preset,
        model_configs: &std::collections::HashMap<String, ModelConfig>,
        agent_names: &[String],
    ) -> Result<()> {
        // 验证所有模型配置存在
        self.validate_before_apply(preset, model_configs)?;

        // 备份当前配置
        self.backup_before_apply()?;

        // 应用预设到指定工具
        for agent_name in agent_names {
            let model_name = preset
                .mappings
                .get(agent_name)
                .ok_or_else(|| anyhow::anyhow!("预设中未找到工具: {}", agent_name))?;

            let model_config = model_configs
                .get(model_name)
                .ok_or_else(|| anyhow::anyhow!("模型配置不存在: {}", model_name))?;

            let adapter = self
                .adapters
                .iter()
                .find(|a| a.name() == agent_name.as_str())
                .ok_or_else(|| anyhow::anyhow!("工具未注册: {}", agent_name))?;

            adapter
                .apply(model_config)
                .map_err(|e| anyhow::anyhow!("应用失败: 工具={}, 原因={}", agent_name, e))?;
        }

        Ok(())
    }

    /// 备份当前配置
    fn backup_before_apply(&self) -> Result<()> {
        for adapter in &self.adapters {
            adapter
                .backup()
                .map_err(|e| anyhow::anyhow!("备份失败: {}", e))?;
        }
        Ok(())
    }

    /// 验证预设
    fn validate_before_apply(
        &self,
        preset: &Preset,
        model_configs: &std::collections::HashMap<String, ModelConfig>,
    ) -> Result<()> {
        // 验证所有模型配置存在
        for model_name in preset.mappings.values() {
            if !model_configs.contains_key(model_name) {
                anyhow::bail!("模型配置不存在: {}", model_name);
            }
        }

        // 验证工具已注册
        for agent_name in preset.mappings.keys() {
            if !self
                .adapters
                .iter()
                .any(|a| a.name() == agent_name.as_str())
            {
                anyhow::bail!("工具未注册: {}", agent_name);
            }
        }

        Ok(())
    }
}
