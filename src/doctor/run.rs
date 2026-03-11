use crate::agents::AgentAdapter;
use crate::agents::global_registry;
use crate::doctor::health::HealthCheckResult;
use crate::doctor::{ConfigFormat, SystemInfo, ToolDetection, ToolStatus};
use colored::Colorize;
use std::path::PathBuf;

/// 运行完整诊断
pub fn run_doctor(_verbose: bool, _json: bool, _fix: bool) -> anyhow::Result<()> {
    let system_info = SystemInfo::new();
    let registry = global_registry();

    println!("{}", "AgentSwitch Tool Diagnostic Report".green().bold());
    println!("{}", "==================================".green());
    println!();

    // 系统信息
    println!("{}", "System Information:".cyan());
    println!("  OS: {}", system_info.os);
    println!("  Arch: {}", system_info.arch);
    if let Some(shell) = &system_info.shell {
        println!("  Shell: {}", shell);
    }
    if let Some(git) = &system_info.git_version {
        println!("  Git: {}", git);
    }
    println!();

    // 工具检测
    println!("{}", "Tool Status:".cyan());
    println!("{}", "-------------".cyan());

    let mut detections = Vec::new();
    let _health_results: Vec<HealthCheckResult> = Vec::new();

    registry.for_each_adapter(|adapter| {
        let detection = detect_tool(adapter);
        println!("{}", format_tool_detection(&detection));
        detections.push(detection);
    });

    println!();

    // 总结
    let installed = detections
        .iter()
        .filter(|d| matches!(d.status, ToolStatus::Installed { .. }))
        .count();

    let healthy = detections
        .iter()
        .filter(|d| matches!(d.status, ToolStatus::Installed { healthy: true }))
        .count();

    println!("{}", "Summary:".cyan());
    println!("  Installed: {}", installed);
    println!("  Healthy: {}", format!("{}", healthy).green());

    Ok(())
}

/// 运行简化版工具检测
pub fn run_detect() -> anyhow::Result<()> {
    let registry = global_registry();

    println!("{}", "Installed tools:".cyan());

    registry.for_each_adapter(|adapter| {
        let detection = detect_tool(adapter);
        if let ToolStatus::Installed { .. } = detection.status {
            if let Some(version) = &detection.version {
                println!("  - {} ({})", detection.display_name, version);
            } else {
                println!("  - {}", detection.display_name);
            }
        }
    });

    Ok(())
}

/// 检测单个工具
fn detect_tool(adapter: &dyn AgentAdapter) -> ToolDetection {
    let name = adapter.name().to_string();
    let display_name = adapter.name().to_string(); // 可以使用更好的显示名称

    // 检测可执行文件
    let (status, executable_path, version) = which::which(adapter.name())
        .ok()
        .map(|path| {
            // 获取版本
            let version = std::process::Command::new(&path)
                .arg("--version")
                .output()
                .ok()
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .map(|s| s.trim().to_string());

            (ToolStatus::Installed { healthy: true }, Some(path), version)
        })
        .unwrap_or((ToolStatus::NotInstalled, None, None));

    // 查找配置文件
    let config_path = adapter.config_path().ok();
    let config_format = detect_config_format(&config_path);

    ToolDetection {
        name,
        display_name,
        status,
        version,
        executable_path,
        config_path,
        config_format,
    }
}

/// 检测配置文件格式
fn detect_config_format(path: &Option<PathBuf>) -> Option<ConfigFormat> {
    let path = path.as_ref()?;
    let extension = path.extension()?.to_str()?;

    match extension {
        "json" => Some(ConfigFormat::Json),
        "toml" => Some(ConfigFormat::Toml),
        "yaml" | "yml" => Some(ConfigFormat::Yaml),
        "env" => Some(ConfigFormat::Env),
        _ => None,
    }
}

/// 格式化工具检测结果
fn format_tool_detection(detection: &ToolDetection) -> String {
    match &detection.status {
        ToolStatus::Installed { healthy } => {
            let status = if *healthy {
                "✓".green()
            } else {
                "⚠".yellow()
            };

            format!(
                "{} {:<15} {:<10} {}",
                status,
                detection.display_name,
                detection.version.as_ref().unwrap_or(&"-".to_string()),
                if *healthy {
                    "Installed (Healthy)"
                } else {
                    "Installed (Warning)"
                }
            )
        }
        ToolStatus::NotInstalled => {
            format!(
                "{} {:<15} {}",
                "✗".red(),
                detection.display_name,
                "Not Installed"
            )
        }
        ToolStatus::DetectionFailed(err) => {
            format!(
                "{} {:<15} Detection Failed: {}",
                "?".red(),
                detection.display_name,
                err
            )
        }
    }
}
