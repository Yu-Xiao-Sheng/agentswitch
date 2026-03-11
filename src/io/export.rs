//! 导出功能

use crate::config::ModelConfig;
use crate::io::sanitizer::sanitize_api_key;
use crate::presets::preset::Preset;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// 导出包
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPackage {
    /// 版本号
    pub version: String,
    /// 导出时间
    pub exported_at: DateTime<Utc>,
    /// 预设集合
    pub presets: Vec<Preset>,
    /// 模型配置集合（API Key 脱敏）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_configs: Option<HashMap<String, SanitizedModelConfig>>,
    /// 当前活跃配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_config: Option<HashMap<String, String>>,
}

/// 脱敏的模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizedModelConfig {
    /// API Key（已脱敏）
    pub api_key: String,
    /// Base URL
    pub base_url: String,
    /// 模型名称
    pub model: String,
}

/// 导出预设到 JSON 文件
pub fn export_presets(presets: &[Preset], output_path: &Path) -> anyhow::Result<()> {
    let package = ExportPackage {
        version: "1.0.0".to_string(),
        exported_at: Utc::now(),
        presets: presets.to_vec(),
        model_configs: None,
        active_config: None,
    };

    let json = serde_json::to_string_pretty(&package)?;
    fs::write(output_path, json)?;

    // 设置文件权限为 600
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(output_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(output_path, perms)?;
    }

    Ok(())
}

/// 导出预设（包含模型配置）
pub fn export_with_model_configs(
    presets: &[Preset],
    model_configs: &HashMap<String, ModelConfig>,
    output_path: &Path,
) -> anyhow::Result<()> {
    // 脱敏模型配置
    let sanitized_configs: HashMap<String, SanitizedModelConfig> = model_configs
        .iter()
        .map(|(name, config)| {
            let sanitized = SanitizedModelConfig {
                api_key: sanitize_api_key(&config.api_key),
                base_url: config.base_url.clone(),
                model: config.model_id.clone(),
            };
            (name.clone(), sanitized)
        })
        .collect();

    let package = ExportPackage {
        version: "1.0.0".to_string(),
        exported_at: Utc::now(),
        presets: presets.to_vec(),
        model_configs: Some(sanitized_configs),
        active_config: None,
    };

    let json = serde_json::to_string_pretty(&package)?;
    fs::write(output_path, json)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(output_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(output_path, perms)?;
    }

    Ok(())
}

/// 导出单个预设
pub fn export_single_preset(preset: &Preset, output_path: &Path) -> anyhow::Result<()> {
    export_presets(&[preset.clone()], output_path)
}
