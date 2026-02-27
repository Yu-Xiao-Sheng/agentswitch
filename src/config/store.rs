//! 配置存储管理
//!
//! 负责配置文件的读写、初始化和持久化

use crate::config::models::{AppConfig, ModelConfig};
use anyhow::Context;
use std::fs;
use std::path::{Path, PathBuf};

/// 配置目录名称
const CONFIG_DIR_NAME: &str = ".agentswitch";

/// 配置文件名称
const CONFIG_FILE_NAME: &str = "config.toml";

/// 配置存储管理器
pub struct ConfigStore {
    /// 配置文件路径
    config_path: PathBuf,

    /// 应用配置
    config: AppConfig,
}

impl ConfigStore {
    /// 创建新的 ConfigStore 实例
    ///
    /// 自动检测并初始化配置目录和文件
    pub fn new() -> anyhow::Result<Self> {
        let config_dir = ConfigStore::get_config_dir()?;
        let config_path = config_dir.join(CONFIG_FILE_NAME);

        // 确保配置已初始化
        Self::ensure_initialized(&config_dir)?;

        // 加载配置
        let config = Self::load_config(&config_path)?;

        Ok(Self {
            config_path,
            config,
        })
    }

    /// 获取配置目录路径
    fn get_config_dir() -> anyhow::Result<PathBuf> {
        let mut config_dir =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("无法获取用户主目录"))?;

        config_dir.push(CONFIG_DIR_NAME);
        Ok(config_dir)
    }

    /// 确保配置已初始化
    ///
    /// 检测配置目录是否存在，如果不存在则创建
    fn ensure_initialized(config_dir: &Path) -> anyhow::Result<()> {
        if !config_dir.exists() {
            println!("⚠️  首次使用，正在自动创建配置...");

            // 创建配置目录
            fs::create_dir_all(config_dir).context("无法创建配置目录")?;

            println!("✓ 配置目录已创建: {}", config_dir.display());

            // 创建默认配置文件
            let config_path = config_dir.join(CONFIG_FILE_NAME);
            let default_config = AppConfig::new();
            let toml_str = toml::to_string_pretty(&default_config).context("无法序列化默认配置")?;

            fs::write(&config_path, toml_str).context("无法写入配置文件")?;

            println!("✓ 配置文件已创建: {}", config_path.display());

            // 设置文件权限
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&config_path)?.permissions();
                perms.set_mode(0o600);
                fs::set_permissions(&config_path, perms)?;
                println!("✓ 文件权限已设置: 0600（仅所有者可读写）");
            }
        }

        Ok(())
    }

    /// 从文件加载配置
    fn load_config(config_path: &Path) -> anyhow::Result<AppConfig> {
        if !config_path.exists() {
            // 文件不存在，返回默认配置
            return Ok(AppConfig::new());
        }

        let content = fs::read_to_string(config_path).context("无法读取配置文件")?;

        let config: AppConfig =
            toml::from_str(&content).context("配置文件格式错误，请检查 TOML 格式")?;

        Ok(config)
    }

    /// 保存配置到文件
    pub fn save(&self) -> anyhow::Result<()> {
        // 确保配置目录存在
        if let Some(config_dir) = self.config_path.parent() {
            fs::create_dir_all(config_dir).context("无法创建配置目录")?;
        }

        // 序列化为 TOML
        let toml_str = toml::to_string_pretty(&self.config).context("无法序列化配置")?;

        // 写入文件
        fs::write(&self.config_path, toml_str).context("无法写入配置文件")?;

        // 设置文件权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(mut perms) = fs::metadata(&self.config_path).map(|m| m.permissions()) {
                perms.set_mode(0o600);
                if let Err(e) = fs::set_permissions(&self.config_path, perms) {
                    eprintln!("⚠️  警告: 无法设置文件权限: {}", e);
                }
            }
        }

        Ok(())
    }

    /// 添加模型配置
    pub fn add_model(&mut self, model: ModelConfig) -> anyhow::Result<()> {
        self.config.add_model(model)?;
        self.save()
    }

    /// 获取所有模型配置
    pub fn list_models(&self) -> &[ModelConfig] {
        &self.config.models
    }

    /// 删除模型配置
    pub fn remove_model(&mut self, name: &str) -> anyhow::Result<()> {
        self.config.remove_model(name)?;
        self.save()
    }

    /// 编辑模型配置
    pub fn edit_model<F>(&mut self, name: &str, updater: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut ModelConfig) -> anyhow::Result<()>,
    {
        self.config.edit_model(name, updater)?;
        self.save()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_dir_path() {
        let config_dir = ConfigStore::get_config_dir().unwrap();
        assert!(config_dir.ends_with(".agentswitch"));
    }
}
