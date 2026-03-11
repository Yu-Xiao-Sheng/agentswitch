//! 预设验证模块
//!
//! 本模块提供预设验证功能。

use std::collections::HashSet;

/// 验证预设
pub fn validate_preset(
    preset: &crate::presets::preset::Preset,
    available_models: &HashSet<String>,
) -> anyhow::Result<()> {
    preset.validate(available_models)
}

/// 验证预设中的工具安装状态
pub fn validate_preset_agents(
    preset: &crate::presets::preset::Preset,
) -> anyhow::Result<Vec<String>> {
    let mut missing_agents = Vec::new();

    for agent_name in preset.mappings.keys() {
        // 检查工具是否已安装
        if which::which(agent_name).is_ok() {
            // 工具已安装
        } else {
            missing_agents.push(agent_name.clone());
        }
    }

    Ok(missing_agents)
}
