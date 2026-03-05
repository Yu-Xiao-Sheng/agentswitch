//! 预设数据结构
//!
//! 本模块定义预设的数据结构。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 配置预设
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    /// 预设名称（唯一标识符）
    pub name: String,
    /// 预设描述
    pub description: String,
    /// 预设标签
    #[serde(default)]
    pub tags: Vec<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 工具到模型的映射关系
    pub mappings: HashMap<String, String>,
}

impl Preset {
    /// 创建新的预设
    pub fn new(name: String, description: String, mappings: HashMap<String, String>) -> Self {
        let now = Utc::now();
        Self {
            name,
            description,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            mappings,
        }
    }

    /// 验证预设的有效性
    pub fn validate(
        &self,
        available_models: &std::collections::HashSet<String>,
    ) -> anyhow::Result<()> {
        // 1. 验证名称格式
        if !is_valid_preset_name(&self.name) {
            anyhow::bail!("预设名称格式无效: {}", self.name);
        }

        // 2. 验证描述长度
        if self.description.len() > 512 {
            anyhow::bail!("预设描述过长（最多 512 字符）");
        }

        // 3. 验证标签
        if self.tags.len() > 10 {
            anyhow::bail!("标签数量过多（最多 10 个）");
        }
        for tag in &self.tags {
            if tag.len() > 32 {
                anyhow::bail!("标签过长: {}（最多 32 字符）", tag);
            }
        }

        // 4. 验证映射关系
        if self.mappings.is_empty() {
            anyhow::bail!("预设必须包含至少一个工具映射");
        }

        // 5. 验证引用的模型存在
        for model_name in self.mappings.values() {
            if !available_models.contains(model_name) {
                anyhow::bail!("模型配置不存在: {}", model_name);
            }
        }

        // 6. 验证时间戳
        if self.updated_at < self.created_at {
            anyhow::bail!("更新时间不能早于创建时间");
        }

        Ok(())
    }
}

/// 验证预设名称格式
pub fn is_valid_preset_name(name: &str) -> bool {
    if name.len() < 1 || name.len() > 64 {
        return false;
    }

    name.chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

/// 验证版本号格式
pub fn is_valid_version(version: &str) -> bool {
    // 简单的 SemVer 验证
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return false;
    }

    parts[0].parse::<u32>().is_ok()
        && parts[1].parse::<u32>().is_ok()
        && (parts[2].parse::<u32>().is_ok() || parts[2].starts_with('-'))
}
