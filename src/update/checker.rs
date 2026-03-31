//! 版本检查器

use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use semver::Version;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

/// 当前版本（从 Cargo.toml 编译时注入）
pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// 项目名称
pub const PACKAGE_NAME: &str = "asw";

/// crates.io API 端点
const CRATES_IO_API: &str = "https://crates.io/api/v1/crates";

/// GitHub API 端点
const GITHUB_API: &str = "https://api.github.com/repos/Yu-Xiao-Sheng/agentswitch/releases/latest";

/// 缓存文件名
const CACHE_FILE: &str = "update_check.json";

/// 缓存有效期（小时）
const CACHE_DURATION_HOURS: i64 = 24;

/// 更新信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateInfo {
    /// 最新版本
    pub latest_version: String,
    /// 当前版本
    pub current_version: String,
    /// 是否有更新
    pub has_update: bool,
    /// 检查时间
    pub checked_at: DateTime<Utc>,
    /// 发布说明 URL
    pub release_url: Option<String>,
}

/// crates.io 响应结构
#[derive(Debug, Deserialize)]
struct CratesIoResponse {
    #[serde(rename = "crate")]
    crate_data: CratesIoCrate,
}

#[derive(Debug, Deserialize)]
struct CratesIoCrate {
    newest_version: String,
}

/// GitHub Release 响应结构
#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
}

/// 版本检查缓存
#[derive(Debug, Deserialize, Serialize)]
struct CheckCache {
    last_check: DateTime<Utc>,
    result: UpdateInfo,
}

/// 获取缓存文件路径
fn get_cache_path() -> Result<PathBuf> {
    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("agentswitch");
    
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir)?;
    }
    
    Ok(cache_dir.join(CACHE_FILE))
}

/// 读取缓存
fn read_cache() -> Option<CheckCache> {
    let cache_path = get_cache_path().ok()?;
    if !cache_path.exists() {
        return None;
    }
    
    let content = fs::read_to_string(cache_path).ok()?;
    serde_json::from_str(&content).ok()
}

/// 写入缓存
fn write_cache(cache: &CheckCache) -> Result<()> {
    let cache_path = get_cache_path()?;
    let content = serde_json::to_string_pretty(cache)?;
    fs::write(cache_path, content)?;
    Ok(())
}

/// 检查缓存是否有效
fn is_cache_valid(cache: &CheckCache) -> bool {
    let now = Utc::now();
    let duration = now.signed_duration_since(cache.last_check);
    duration.num_hours() < CACHE_DURATION_HOURS
}

/// 从 crates.io 获取最新版本
fn fetch_latest_version_crates_io(client: &Client) -> Result<String> {
    let url = format!("{}/{}", CRATES_IO_API, PACKAGE_NAME);
    
    let response = client
        .get(&url)
        .header("User-Agent", format!("{}-update-checker/{}", PACKAGE_NAME, CURRENT_VERSION))
        .timeout(Duration::from_secs(10))
        .send()?
        .json::<CratesIoResponse>()?;
    
    Ok(response.crate_data.newest_version)
}

/// 从 GitHub 获取最新版本
fn fetch_latest_version_github(client: &Client) -> Result<(String, String)> {
    let response = client
        .get(GITHUB_API)
        .header("User-Agent", format!("{}-update-checker/{}", PACKAGE_NAME, CURRENT_VERSION))
        .header("Accept", "application/vnd.github.v3+json")
        .timeout(Duration::from_secs(10))
        .send()?
        .json::<GitHubRelease>()?;
    
    // 移除 'v' 前缀（如果有）
    let version = response.tag_name.strip_prefix('v').unwrap_or(&response.tag_name).to_string();
    
    Ok((version, response.html_url))
}

/// 比较版本号
fn compare_versions(current: &str, latest: &str) -> Result<bool> {
    let current_ver = Version::parse(current)?;
    let latest_ver = Version::parse(latest)?;
    
    Ok(latest_ver > current_ver)
}

/// 检查更新（带缓存）
pub fn check_for_update(force: bool) -> Result<UpdateInfo> {
    // 如果不强制检查，先尝试读取缓存
    if !force {
        if let Some(cache) = read_cache() {
            if is_cache_valid(&cache) {
                return Ok(cache.result);
            }
        }
    }
    
    // 执行检查
    let client = Client::new();
    
    // 优先从 crates.io 获取，失败则从 GitHub 获取
    let (latest_version, release_url) = match fetch_latest_version_crates_io(&client) {
        Ok(version) => {
            // 从 crates.io 获取成功，但不包含 release URL
            (version, None)
        }
        Err(e) => {
            eprintln!("⚠️  从 crates.io 获取版本失败: {}, 尝试 GitHub...", e);
            match fetch_latest_version_github(&client) {
                Ok((version, url)) => (version, Some(url)),
                Err(e) => {
                    anyhow::bail!("无法获取最新版本: {}", e);
                }
            }
        }
    };
    
    let has_update = compare_versions(CURRENT_VERSION, &latest_version)?;
    
    let update_info = UpdateInfo {
        latest_version: latest_version.clone(),
        current_version: CURRENT_VERSION.to_string(),
        has_update,
        checked_at: Utc::now(),
        release_url,
    };
    
    // 写入缓存
    let cache = CheckCache {
        last_check: Utc::now(),
        result: update_info.clone(),
    };
    write_cache(&cache)?;
    
    Ok(update_info)
}

/// 显示更新提示
pub fn display_update_notification(info: &UpdateInfo) {
    if !info.has_update {
        println!("✓ 已是最新版本: v{}", info.current_version);
        return;
    }
    
    println!();
    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│  📦 有新版本可用!                                            │");
    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│  当前版本: v{:<47}│", info.current_version);
    println!("│  最新版本: v{:<47}│", info.latest_version);
    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│  更新命令:                                                   │");
    println!("│    cargo install asw --force                                │");
    println!("│  或访问:                                                     │");
    println!("│    https://github.com/Yu-Xiao-Sheng/agentswitch/releases    │");
    println!("└─────────────────────────────────────────────────────────────┘");
    println!();
}

/// 启动时检查更新（静默模式，只在有更新时显示）
pub fn check_on_startup() {
    // 静默检查，不显示错误
    if let Ok(info) = check_for_update(false) {
        if info.has_update {
            display_update_notification(&info);
        }
    }
}
