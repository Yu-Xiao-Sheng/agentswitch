use std::path::PathBuf;

/// 工具检测结果
#[derive(Debug, Clone)]
pub struct ToolDetection {
    /// 工具名称（如 claude-code, codex）
    pub name: String,

    /// 显示名称（如 Claude Code, Microsoft Codex）
    pub display_name: String,

    /// 检测状态
    pub status: ToolStatus,

    /// 工具版本（如果已安装）
    pub version: Option<String>,

    /// 可执行文件路径（如果已安装）
    pub executable_path: Option<PathBuf>,

    /// 配置文件路径（如果找到）
    pub config_path: Option<PathBuf>,

    /// 配置文件格式
    pub config_format: Option<ConfigFormat>,
}

/// 工具状态
#[derive(Debug, Clone)]
pub enum ToolStatus {
    /// 已安装且配置正常
    Installed { healthy: bool },

    /// 未安装
    NotInstalled,

    /// 检测失败
    DetectionFailed(String),
}

/// 配置文件格式
#[derive(Debug, Clone, Copy)]
pub enum ConfigFormat {
    /// JSON 格式
    Json,

    /// TOML 格式
    Toml,

    /// YAML 格式
    Yaml,

    /// 环境变量 (.env)
    Env,
}
