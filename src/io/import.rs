//! 导入功能

use crate::config::ModelConfig;
use crate::io::export::ExportPackage;
use crate::presets::preset;
use crate::presets::preset::Preset;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// 导入策略
#[derive(Debug, Clone, Copy)]
pub enum ImportStrategy {
    /// 合并模式：保留现有预设，仅添加不冲突的预设
    Merge,
    /// 覆盖模式：替换现有预设
    Overwrite,
}

/// 从 JSON 文件导入预设
pub fn import_presets(input_path: &Path, strategy: ImportStrategy) -> anyhow::Result<Vec<Preset>> {
    // 读取并验证文件
    let content = fs::read_to_string(input_path)?;

    // 解析 JSON
    let package: ExportPackage = serde_json::from_str(&content)?;

    // 验证版本
    if !preset::is_valid_version(&package.version) {
        anyhow::bail!("无效的版本号: {}", package.version);
    }

    // 验证预设
    if package.presets.is_empty() {
        anyhow::bail!("导出包必须包含至少一个预设");
    }

    // 根据策略处理预设
    match strategy {
        ImportStrategy::Merge => {
            // 合并模式：直接返回所有预设
            Ok(package.presets)
        }
        ImportStrategy::Overwrite => {
            // 覆盖模式：返回所有预设（覆盖逻辑在调用方处理）
            Ok(package.presets)
        }
    }
}

/// 验证导入文件
pub fn validate_import_file(input_path: &Path) -> anyhow::Result<()> {
    // 检查文件是否存在
    if !input_path.exists() {
        anyhow::bail!("文件不存在: {:?}", input_path);
    }

    // 检查文件扩展名
    match input_path.extension().and_then(|e| e.to_str()) {
        Some("json") => {}
        _ => anyhow::bail!("不支持的文件格式，必须是 JSON 文件"),
    }

    // 读取并验证文件内容
    let content = fs::read_to_string(input_path)?;
    let package: ExportPackage = serde_json::from_str(&content)?;

    // 验证版本
    if !preset::is_valid_version(&package.version) {
        anyhow::bail!("无效的版本号: {}", package.version);
    }

    // 验证预设
    if package.presets.is_empty() {
        anyhow::bail!("导出包必须包含至少一个预设");
    }

    // 检查文件大小（不超过 10MB）
    let metadata = fs::metadata(input_path)?;
    if metadata.len() > 10 * 1024 * 1024 {
        anyhow::bail!("文件过大（最多 10MB）");
    }

    Ok(())
}

/// 检查导入依赖
pub fn check_import_dependencies(
    package: &ExportPackage,
    available_models: &HashMap<String, ModelConfig>,
) -> Vec<String> {
    let mut missing_models = Vec::new();

    // 检查预设中引用的模型
    for preset in &package.presets {
        for model_name in preset.mappings.values() {
            if !available_models.contains_key(model_name) {
                if !missing_models.contains(model_name) {
                    missing_models.push(model_name.clone());
                }
            }
        }
    }

    missing_models
}

/// 预览导入变更
pub fn preview_import_changes(
    package: &ExportPackage,
    existing_presets: &HashMap<String, crate::presets::preset::Preset>,
) -> ImportPreview {
    let mut new_presets = Vec::new();
    let mut conflict_presets = Vec::new();
    let mut skipped_presets = Vec::new();

    for preset in &package.presets {
        if existing_presets.contains_key(&preset.name) {
            conflict_presets.push(preset.name.clone());
        } else {
            new_presets.push(preset.name.clone());
        }
    }

    ImportPreview {
        new_presets,
        conflict_presets,
        skipped_presets,
    }
}

/// 导入预览
#[derive(Debug, Clone)]
pub struct ImportPreview {
    /// 新增预设
    pub new_presets: Vec<String>,
    /// 冲突预设
    pub conflict_presets: Vec<String>,
    /// 跳过的预设
    pub skipped_presets: Vec<String>,
}
