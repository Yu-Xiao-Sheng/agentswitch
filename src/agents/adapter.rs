use crate::config::Provider;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

/// 配置文件格式枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub enum ConfigFormat {
    Json,
    Toml,
    Yaml,
}

/// Represents a backup of an agent's configuration
///
/// 注意：这是兼容 v0.1.0 的旧版本结构
/// TODO: 在 v0.3.0 迁移到新结构（file_path, original_path, size_bytes, format）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backup {
    pub agent_name: String,
    pub original_config_path: PathBuf,
    pub backup_path: PathBuf,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Agent 配置状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AgentConfigState {
    /// 工具名称
    pub agent_name: String,

    /// 当前使用的模型配置名称（如果已配置）
    pub model_name: Option<String>,

    /// 配置文件路径
    pub config_path: PathBuf,

    /// 最后切换时间（如果已切换）
    pub last_switched: Option<String>,

    /// 工具是否已安装
    pub is_installed: bool,

    /// 配置文件是否存在
    pub config_exists: bool,
}

/// Trait for adapting different code agent tools
pub trait AgentAdapter: Send + Sync {
    /// Get the name of this agent tool
    fn name(&self) -> &str;

    /// Detect if this agent tool is installed
    fn detect(&self) -> Result<bool>;

    /// Get the path to the agent's configuration file
    fn config_path(&self) -> Result<PathBuf>;

    /// Create a backup of the current configuration
    fn backup(&self) -> Result<Backup>;

    /// Apply a provider configuration with a specific model to this agent
    fn apply(&self, provider: &Provider, model: &str) -> Result<()>;

    /// Restore a configuration from a backup
    fn restore(&self, backup: &Backup) -> Result<()>;

    /// Get the current model being used by this agent
    fn current_model(&self) -> Result<Option<String>>;

    /// Validate if the given fields are compatible with this agent
    /// Returns a list of incompatible field names with warnings
    fn validate_compatibility(&self, _fields: &HashSet<String>) -> Vec<FieldWarning> {
        vec![] // Default implementation: no warnings
    }
}

/// 字段警告信息
#[derive(Debug, Clone)]
pub struct FieldWarning {
    /// 字段名称
    pub field_name: String,

    /// 警告级别
    pub level: WarningLevel,

    /// 警告消息
    pub message: String,
}

/// 警告级别
#[derive(Debug, Clone, PartialEq)]
pub enum WarningLevel {
    /// 信息: 字段将被保留
    Info,
    /// 警告: 字段可能不兼容
    Warning,
    /// 错误: 字段完全不兼容
    Error,
}

/// 不兼容字段检测器
pub struct IncompatibleFieldDetector {
    /// 常见的不兼容字段列表
    incompatible_fields: HashSet<String>,
}

impl IncompatibleFieldDetector {
    /// 创建新的检测器
    pub fn new() -> Self {
        let mut incompatible_fields = HashSet::new();

        // 添加常见的不兼容字段
        incompatible_fields.insert("custom_headers".to_string());
        incompatible_fields.insert("custom_proxy".to_string());
        incompatible_fields.insert("proxy_url".to_string());
        incompatible_fields.insert("timeout_ms".to_string());
        incompatible_fields.insert("max_retries".to_string());
        incompatible_fields.insert("retry_delay".to_string());
        incompatible_fields.insert("debug_mode".to_string());
        incompatible_fields.insert("verbose_logging".to_string());

        Self {
            incompatible_fields,
        }
    }

    /// 检测字段兼容性
    ///
    /// # 参数
    /// - `fields`: 要检测的字段集合
    /// - `agent_name`: 工具名称（用于生成警告消息）
    ///
    /// # 返回
    /// 返回不兼容字段的警告列表
    pub fn detect(&self, fields: &HashSet<String>, agent_name: &str) -> Vec<FieldWarning> {
        let mut warnings = Vec::new();

        for field in fields {
            if self.incompatible_fields.contains(field) {
                warnings.push(FieldWarning {
                    field_name: field.clone(),
                    level: WarningLevel::Warning,
                    message: format!(
                        "字段 '{}' 可能不被 {} 支持。切换后该字段可能被忽略或导致意外行为。",
                        field, agent_name
                    ),
                });
            }
        }

        warnings
    }

    /// 检测特定工具的不兼容字段
    ///
    /// # 参数
    /// - `fields`: 要检测的字段集合
    /// - `agent_name`: 工具名称
    ///
    /// # 返回
    /// 返回工具特定的不兼容字段警告
    pub fn detect_for_agent(
        &self,
        fields: &HashSet<String>,
        agent_name: &str,
    ) -> Vec<FieldWarning> {
        let mut warnings = self.detect(fields, agent_name);

        // 添加工具特定的不兼容字段
        let agent_specific = self.get_agent_specific_incompatible_fields(agent_name);

        for field in fields {
            if agent_specific.contains(field) {
                warnings.push(FieldWarning {
                    field_name: field.clone(),
                    level: WarningLevel::Warning,
                    message: format!(
                        "字段 '{}' 是 {} 特有的配置，切换到其他工具时可能丢失。",
                        field, agent_name
                    ),
                });
            }
        }

        warnings
    }

    /// 获取工具特定的不兼容字段
    fn get_agent_specific_incompatible_fields(&self, agent_name: &str) -> HashSet<String> {
        match agent_name {
            "claude-code" => {
                let mut fields = HashSet::new();
                fields.insert("includeCoAuthoredBy".to_string());
                fields.insert("anthropicVersion".to_string());
                fields
            }
            "codex" => {
                let mut fields = HashSet::new();
                fields.insert("model_provider".to_string());
                fields.insert("wire_api".to_string());
                fields
            }
            "gemini-cli" => {
                let mut fields = HashSet::new();
                fields.insert("defaultModel".to_string());
                fields.insert("project_id".to_string());
                fields
            }
            _ => HashSet::new(),
        }
    }
}

impl Default for IncompatibleFieldDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// 格式化字段警告为可读字符串
pub fn format_field_warnings(warnings: &[FieldWarning]) -> String {
    if warnings.is_empty() {
        return String::new();
    }

    let mut output = String::from("⚠️  配置兼容性警告:\n");

    for warning in warnings {
        let icon = match warning.level {
            WarningLevel::Info => "ℹ️",
            WarningLevel::Warning => "⚠️",
            WarningLevel::Error => "❌",
        };

        output.push_str(&format!(
            "  {} {}: {}\n",
            icon, warning.field_name, warning.message
        ));
    }

    output.pop(); // 移除最后的换行符
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_detection() {
        let detector = IncompatibleFieldDetector::new();
        let mut fields = HashSet::new();
        fields.insert("custom_headers".to_string());
        fields.insert("base_url".to_string());

        let warnings = detector.detect(&fields, "claude-code");

        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].field_name, "custom_headers");
        assert_eq!(warnings[0].level, WarningLevel::Warning);
    }

    #[test]
    fn test_agent_specific_fields() {
        let detector = IncompatibleFieldDetector::new();
        let mut fields = HashSet::new();
        fields.insert("includeCoAuthoredBy".to_string());
        fields.insert("base_url".to_string());

        let warnings = detector.detect_for_agent(&fields, "claude-code");

        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].field_name, "includeCoAuthoredBy");
    }

    #[test]
    fn test_format_warnings() {
        let warnings = vec![FieldWarning {
            field_name: "custom_headers".to_string(),
            level: WarningLevel::Warning,
            message: "可能不被支持".to_string(),
        }];

        let formatted = format_field_warnings(&warnings);

        assert!(formatted.contains("配置兼容性警告"));
        assert!(formatted.contains("custom_headers"));
    }
}
