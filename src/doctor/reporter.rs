/// 完整诊断报告
#[derive(Debug, Clone)]
pub struct DoctorReport {
    /// 所有工具检测结果
    pub detections: Vec<super::ToolDetection>,

    /// 健康检查结果
    pub health_results: Vec<super::HealthCheckResult>,

    /// 系统信息
    pub system_info: SystemInfo,

    /// 检测时间
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// 系统信息
#[derive(Debug, Clone)]
pub struct SystemInfo {
    /// 操作系统
    pub os: String,

    /// 架构
    pub arch: String,

    /// Shell 类型（如果可检测）
    pub shell: Option<String>,

    /// Git 版本（如果已安装）
    pub git_version: Option<String>,
}

impl SystemInfo {
    pub fn new() -> Self {
        let os = std::env::consts::OS.to_string();
        let arch = std::env::consts::ARCH.to_string();

        let shell = std::env::var("SHELL").ok();

        let git_version = std::process::Command::new("git")
            .arg("--version")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok());

        Self {
            os,
            arch,
            shell,
            git_version,
        }
    }
}
