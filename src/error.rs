//! 智能错误提示系统
//!
//! 提供友好的错误消息、原因分析和解决建议

use std::path::PathBuf;

/// AgentSwitch 统一错误类型
#[derive(thiserror::Error, Debug)]
pub enum AgentSwitchError {
    /// Agent 工具未找到
    #[error("未检测到 Agent 工具: {0}")]
    AgentNotFound(String),

    /// 配置文件只读
    #[error("配置文件只读，无法修改: {0}")]
    ConfigFileReadOnly(PathBuf),

    /// 备份文件损坏
    #[error("备份文件损坏: {0}")]
    BackupCorrupted(String),

    /// 磁盘空间不足
    #[error("磁盘空间不足，无法创建备份")]
    DiskSpaceInsufficient,

    /// 工具配置错误
    #[error("工具配置错误: {0}")]
    ToolConfigError(String),

    /// 未找到模型配置
    #[error("模型配置 '{0}' 不存在")]
    ModelConfigNotFound(String),

    /// 配置文件不存在
    #[error("配置文件不存在: {0}")]
    ConfigNotFound(PathBuf),

    /// IO 错误
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    /// 序列化错误
    #[error("序列化错误: {0}")]
    Serialization(String),

    /// 其他错误
    #[error("错误: {0}")]
    Other(String),
}

impl From<serde_json::Error> for AgentSwitchError {
    fn from(err: serde_json::Error) -> Self {
        AgentSwitchError::Serialization(err.to_string())
    }
}

impl From<toml::de::Error> for AgentSwitchError {
    fn from(err: toml::de::Error) -> Self {
        AgentSwitchError::Serialization(err.to_string())
    }
}

impl From<toml::ser::Error> for AgentSwitchError {
    fn from(err: toml::ser::Error) -> Self {
        AgentSwitchError::Serialization(err.to_string())
    }
}

/// AgentSwitch Result 类型
pub type Result<T> = std::result::Result<T, AgentSwitchError>;

// ============================================================================
// 智能错误提示系统
// ============================================================================

use colored::Colorize;

/// 智能错误类型
#[derive(Debug, Clone)]
pub enum AswError {
    /// 配置错误
    Config {
        message: String,
        field: Option<String>,
        value: Option<String>,
        suggestion: String,
    },
    /// 网络错误
    Network {
        message: String,
        url: Option<String>,
        causes: Vec<String>,
        suggestions: Vec<String>,
    },
    /// 权限错误
    Permission {
        message: String,
        path: Option<String>,
        suggestion: String,
    },
    /// 提供商错误
    Provider {
        message: String,
        provider: String,
        suggestion: String,
    },
    /// 工具错误
    Tool {
        message: String,
        tool: String,
        suggestion: String,
    },
    /// 加密错误
    Crypto {
        message: String,
        suggestion: String,
    },
    /// Git 错误
    Git {
        message: String,
        suggestion: String,
    },
    /// 其他错误
    Other { message: String },
}

impl AswError {
    /// 创建配置错误
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
            field: None,
            value: None,
            suggestion: String::new(),
        }
    }

    /// 设置字段名
    pub fn field(mut self, field: impl Into<String>) -> Self {
        if let Self::Config { field: f, .. } = &mut self {
            *f = Some(field.into());
        }
        self
    }

    /// 设置字段值
    pub fn value(mut self, value: impl Into<String>) -> Self {
        if let Self::Config { value: v, .. } = &mut self {
            *v = Some(value.into());
        }
        self
    }

    /// 设置建议
    pub fn suggest(mut self, suggestion: impl Into<String>) -> Self {
        match &mut self {
            Self::Config { suggestion: s, .. } => *s = suggestion.into(),
            Self::Network { suggestions: ss, .. } => ss.push(suggestion.into()),
            Self::Permission { suggestion: s, .. } => *s = suggestion.into(),
            Self::Provider { suggestion: s, .. } => *s = suggestion.into(),
            Self::Tool { suggestion: s, .. } => *s = suggestion.into(),
            Self::Crypto { suggestion: s, .. } => *s = suggestion.into(),
            Self::Git { suggestion: s, .. } => *s = suggestion.into(),
            _ => {}
        }
        self
    }

    /// 创建网络错误
    pub fn network(message: impl Into<String>) -> Self {
        Self::Network {
            message: message.into(),
            url: None,
            causes: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// 设置 URL
    pub fn url(mut self, url: impl Into<String>) -> Self {
        if let Self::Network { url: u, .. } = &mut self {
            *u = Some(url.into());
        }
        self
    }

    /// 添加原因
    pub fn cause(mut self, cause: impl Into<String>) -> Self {
        if let Self::Network { causes: c, .. } = &mut self {
            c.push(cause.into());
        }
        self
    }

    /// 创建权限错误
    pub fn permission(message: impl Into<String>) -> Self {
        Self::Permission {
            message: message.into(),
            path: None,
            suggestion: String::new(),
        }
    }

    /// 设置路径
    pub fn path(mut self, path: impl Into<String>) -> Self {
        if let Self::Permission { path: p, .. } = &mut self {
            *p = Some(path.into());
        }
        self
    }

    /// 创建提供商错误
    pub fn provider(message: impl Into<String>, provider: impl Into<String>) -> Self {
        Self::Provider {
            message: message.into(),
            provider: provider.into(),
            suggestion: String::new(),
        }
    }

    /// 创建工具错误
    pub fn tool(message: impl Into<String>, tool: impl Into<String>) -> Self {
        Self::Tool {
            message: message.into(),
            tool: tool.into(),
            suggestion: String::new(),
        }
    }

    /// 创建加密错误
    pub fn crypto(message: impl Into<String>) -> Self {
        Self::Crypto {
            message: message.into(),
            suggestion: String::new(),
        }
    }

    /// 创建 Git 错误
    pub fn git(message: impl Into<String>) -> Self {
        Self::Git {
            message: message.into(),
            suggestion: String::new(),
        }
    }

    /// 创建其他错误
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for AswError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", "✗".red(), "错误".red().bold())?;
        writeln!(f)?;

        match self {
            Self::Config {
                message,
                field,
                value,
                suggestion,
            } => {
                writeln!(f, "  {}: {}", "类型".bold(), "配置错误".yellow())?;
                if let Some(fld) = field {
                    writeln!(f, "  {}: {}", "字段".bold(), fld.yellow())?;
                }
                if let Some(v) = value {
                    writeln!(f, "  {}: {}", "值".bold(), v.yellow())?;
                }
                writeln!(f, "  {}: {}", "描述".bold(), message)?;
                if !suggestion.is_empty() {
                    writeln!(f)?;
                    writeln!(f, "{}", "建议操作:".green().bold())?;
                    writeln!(f, "  • {}", suggestion)?;
                }
            }
            Self::Network {
                message,
                url,
                causes,
                suggestions,
            } => {
                writeln!(f, "  {}: {}", "类型".bold(), "网络错误".yellow())?;
                if let Some(u) = url {
                    writeln!(f, "  {}: {}", "URL".bold(), u.yellow())?;
                }
                writeln!(f, "  {}: {}", "描述".bold(), message)?;
                if !causes.is_empty() {
                    writeln!(f)?;
                    writeln!(f, "{}", "可能的原因:".yellow().bold())?;
                    for (i, cause) in causes.iter().enumerate() {
                        writeln!(f, "  {}. {}", i + 1, cause)?;
                    }
                }
                if !suggestions.is_empty() {
                    writeln!(f)?;
                    writeln!(f, "{}", "建议操作:".green().bold())?;
                    for s in suggestions {
                        writeln!(f, "  • {}", s)?;
                    }
                }
            }
            Self::Permission {
                message,
                path,
                suggestion,
            } => {
                writeln!(f, "  {}: {}", "类型".bold(), "权限错误".yellow())?;
                if let Some(p) = path {
                    writeln!(f, "  {}: {}", "路径".bold(), p.yellow())?;
                }
                writeln!(f, "  {}: {}", "描述".bold(), message)?;
                if !suggestion.is_empty() {
                    writeln!(f)?;
                    writeln!(f, "{}", "建议操作:".green().bold())?;
                    writeln!(f, "  • {}", suggestion)?;
                }
            }
            Self::Provider {
                message,
                provider,
                suggestion,
            } => {
                writeln!(f, "  {}: {}", "类型".bold(), "提供商错误".yellow())?;
                writeln!(f, "  {}: {}", "提供商".bold(), provider.yellow())?;
                writeln!(f, "  {}: {}", "描述".bold(), message)?;
                if !suggestion.is_empty() {
                    writeln!(f)?;
                    writeln!(f, "{}", "建议操作:".green().bold())?;
                    writeln!(f, "  • {}", suggestion)?;
                }
            }
            Self::Tool {
                message,
                tool,
                suggestion,
            } => {
                writeln!(f, "  {}: {}", "类型".bold(), "工具错误".yellow())?;
                writeln!(f, "  {}: {}", "工具".bold(), tool.yellow())?;
                writeln!(f, "  {}: {}", "描述".bold(), message)?;
                if !suggestion.is_empty() {
                    writeln!(f)?;
                    writeln!(f, "{}", "建议操作:".green().bold())?;
                    writeln!(f, "  • {}", suggestion)?;
                }
            }
            Self::Crypto { message, suggestion } => {
                writeln!(f, "  {}: {}", "类型".bold(), "加密错误".yellow())?;
                writeln!(f, "  {}: {}", "描述".bold(), message)?;
                if !suggestion.is_empty() {
                    writeln!(f)?;
                    writeln!(f, "{}", "建议操作:".green().bold())?;
                    writeln!(f, "  • {}", suggestion)?;
                }
            }
            Self::Git { message, suggestion } => {
                writeln!(f, "  {}: {}", "类型".bold(), "Git 错误".yellow())?;
                writeln!(f, "  {}: {}", "描述".bold(), message)?;
                if !suggestion.is_empty() {
                    writeln!(f)?;
                    writeln!(f, "{}", "建议操作:".green().bold())?;
                    writeln!(f, "  • {}", suggestion)?;
                }
            }
            Self::Other { message } => {
                writeln!(f, "  {}: {}", "描述".bold(), message)?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for AswError {}

/// 便捷宏：创建配置错误
#[macro_export]
macro_rules! config_error {
    ($msg:expr) => {
        AswError::config($msg)
    };
    ($msg:expr, $field:expr) => {
        AswError::config($msg).field($field)
    };
}

/// 便捷宏：创建网络错误
#[macro_export]
macro_rules! network_error {
    ($msg:expr) => {
        AswError::network($msg)
    };
}

/// 便捷宏：创建提供商错误
#[macro_export]
macro_rules! provider_error {
    ($msg:expr, $provider:expr) => {
        AswError::provider($msg, $provider)
    };
}

/// 便捷宏：创建工具错误
#[macro_export]
macro_rules! tool_error {
    ($msg:expr, $tool:expr) => {
        AswError::tool($msg, $tool)
    };
}

// ============================================================================
// 错误创建辅助函数
// ============================================================================

/// 创建 URL 格式错误
pub fn invalid_url_error(url: &str, reason: &str) -> AswError {
    AswError::config(format!("URL 格式无效: {}", reason))
        .field("base_url")
        .value(url)
        .suggest("URL 格式应为: https://api.example.com/v1")
}

/// 创建 API Key 格式错误
pub fn invalid_api_key_error(key_preview: &str) -> AswError {
    AswError::config("API Key 格式可能无效")
        .field("api_key")
        .value(format!("{}...", key_preview))
        .suggest("请检查 API Key 是否正确复制，确保没有多余的空格或换行")
}

/// 创建模型名称格式错误
pub fn invalid_model_name_error(name: &str, reason: &str) -> AswError {
    AswError::config(format!("模型名称无效: {}", reason))
        .field("model")
        .value(name)
        .suggest("模型名称只能包含字母、数字、连字符和下划线，且不能以连字符或下划线开头/结尾")
}

/// 创建提供商不存在错误
pub fn provider_not_found_error(provider: &str) -> AswError {
    AswError::provider(format!("供应商 '{}' 不存在", provider), provider)
        .suggest(format!("运行 'asw provider list' 查看所有供应商，或使用 'asw provider add {}' 添加", provider))
}

/// 创建工具未安装错误
pub fn tool_not_installed_error(tool: &str) -> AswError {
    AswError::tool(format!("未检测到 {} 安装", tool), tool)
        .suggest(get_install_hint(tool))
}

/// 创建配置文件不存在错误
pub fn config_not_found_error(path: &str, tool: &str) -> AswError {
    AswError::config("配置文件不存在")
        .field("config_path")
        .value(path)
        .suggest(format!(
            "请先运行 '{}' 进行初始配置，或手动创建配置文件",
            tool
        ))
}

/// 创建网络连接失败错误
pub fn network_connection_error(url: &str, error_msg: &str) -> AswError {
    AswError::network(format!("无法连接到 API 端点: {}", error_msg))
        .url(url)
        .cause("API 地址错误")
        .cause("网络连接问题")
        .cause("需要配置代理")
        .cause("API 服务不可用")
        .suggest("检查 base_url 配置是否正确".to_string())
        .suggest("测试网络连接: ping <api-host>")
        .suggest("如需代理，设置环境变量: export HTTP_PROXY=http://proxy:port")
        .suggest("运行 'asw provider test <name> --verbose' 获取详细诊断".to_string())
}

/// 创建权限不足错误
pub fn permission_denied_error(path: &str, operation: &str) -> AswError {
    AswError::permission(format!("权限不足，无法{}", operation))
        .path(path)
        .suggest("检查文件权限，或尝试使用管理员权限运行")
}

/// 创建模型不在提供商列表中错误
pub fn model_not_in_provider_error(model: &str, provider: &str, available_models: &[String]) -> AswError {
    let available = if available_models.is_empty() {
        "无可用模型".to_string()
    } else {
        available_models.join(", ")
    };

    AswError::provider(
        format!("模型 '{}' 不在供应商的模型列表中", model),
        provider,
    )
    .suggest(format!("可用模型: {}", available))
}

/// 获取工具安装提示
fn get_install_hint(tool: &str) -> String {
    match tool {
        "claude-code" => "安装: npm install -g @anthropic-ai/claude-code".to_string(),
        "codex" => "安装: npm install -g @openai/codex@0.80.0".to_string(),
        "gemini-cli" => "安装: npm install -g @google/gemini-cli".to_string(),
        "qwen-cli" => "安装: npm install -g @alibaba/qwen-cli".to_string(),
        "grok-cli" => "安装: npm install -g xai/grok-cli".to_string(),
        "opencode" => "安装: npm install -g opencode".to_string(),
        _ => "请查看官方文档了解安装方法".to_string(),
    }
}

/// 打印错误（友好的错误格式）
pub fn print_error(error: &AswError) {
    eprintln!("{}", error);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_error() {
        let err = AswError::config("供应商名称不能为空")
            .field("name")
            .suggest("请提供有效的供应商名称");

        let output = format!("{}", err);
        assert!(output.contains("配置错误"));
        assert!(output.contains("字段"));
        assert!(output.contains("建议操作"));
    }

    #[test]
    fn test_network_error() {
        let err = AswError::network("连接超时")
            .url("https://api.example.com/v1")
            .cause("网络连接问题")
            .suggest("检查网络连接");

        let output = format!("{}", err);
        assert!(output.contains("网络错误"));
        assert!(output.contains("URL"));
        assert!(output.contains("可能的原因"));
        assert!(output.contains("建议操作"));
    }

    #[test]
    fn test_provider_not_found_error() {
        let err = provider_not_found_error("zhipu");
        let output = format!("{}", err);
        assert!(output.contains("提供商错误"));
        assert!(output.contains("zhipu"));
        assert!(output.contains("asw provider list"));
    }
}
