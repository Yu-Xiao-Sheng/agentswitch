//! 配置存储管理
//!
//! 负责配置文件的读写、初始化和持久化

use crate::config::config::Config;
use crate::config::models::ModelConfig;
use crate::config::provider::{ActiveModel, Protocol, Provider};
use anyhow::Context;
use std::collections::HashMap;
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
    config: Config,
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

        // 加载配置（含自动迁移）
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
    fn ensure_initialized(config_dir: &Path) -> anyhow::Result<()> {
        if !config_dir.exists() {
            println!("⚠️  首次使用，正在自动创建配置...");

            fs::create_dir_all(config_dir).context("无法创建配置目录")?;
            println!("✓ 配置目录已创建: {}", config_dir.display());

            let config_path = config_dir.join(CONFIG_FILE_NAME);
            let default_config = Config::new();
            let toml_str = toml::to_string_pretty(&default_config).context("无法序列化默认配置")?;

            fs::write(&config_path, toml_str).context("无法写入配置文件")?;
            println!("✓ 配置文件已创建: {}", config_path.display());

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

    /// 从文件加载配置（含自动迁移逻辑）
    fn load_config(config_path: &Path) -> anyhow::Result<Config> {
        if !config_path.exists() {
            return Ok(Config::new());
        }

        let content = fs::read_to_string(config_path).context("无法读取配置文件")?;

        // 尝试解析为新格式
        if let Ok(config) = toml::from_str::<Config>(&content) {
            // 检查是否有 providers 字段（新格式）
            if !config.providers.is_empty() || !content.contains("\n[[models]]") {
                return Ok(config);
            }
        }

        // 回退：尝试解析为旧格式并迁移
        let old_config: crate::config::models::AppConfig =
            toml::from_str(&content).context("配置文件格式错误，请检查 TOML 格式")?;

        // 迁移旧格式到新格式
        let migrated = Self::migrate_legacy_config(&old_config);

        // 保存迁移后的配置
        let toml_str = toml::to_string_pretty(&migrated).context("无法序列化迁移后的配置")?;
        fs::write(config_path, toml_str).context("无法写入迁移后的配置文件")?;

        println!("✓ 配置已从旧格式自动迁移为新格式（Provider-Model 架构）");

        Ok(migrated)
    }

    /// 从旧格式迁移到新格式
    fn migrate_legacy_config(old: &crate::config::models::AppConfig) -> Config {
        Config::migrate_from_legacy(&old.models, &old.active_models)
    }

    /// 保存配置到文件
    pub fn save(&self) -> anyhow::Result<()> {
        if let Some(config_dir) = self.config_path.parent() {
            fs::create_dir_all(config_dir).context("无法创建配置目录")?;
        }

        let toml_str = toml::to_string_pretty(&self.config).context("无法序列化配置")?;
        fs::write(&self.config_path, toml_str).context("无法写入配置文件")?;

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

    // ==================== Provider 操作 ====================

    /// 添加供应商
    pub fn add_provider(&mut self, provider: Provider) -> anyhow::Result<()> {
        self.config.add_provider(provider)?;
        self.save()
    }

    /// 获取所有供应商
    pub fn list_providers(&self) -> &[Provider] {
        &self.config.providers
    }

    /// 获取供应商
    pub fn get_provider(&self, name: &str) -> Option<&Provider> {
        self.config.get_provider(name)
    }

    /// 删除供应商
    pub fn remove_provider(&mut self, name: &str) -> anyhow::Result<()> {
        self.config.remove_provider(name)?;
        self.save()
    }

    /// 检查供应商是否存在
    pub fn has_provider(&self, name: &str) -> bool {
        self.config.providers.iter().any(|p| p.name == name)
    }

    /// 编辑供应商
    pub fn edit_provider<F>(&mut self, name: &str, updater: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut Provider) -> anyhow::Result<()>,
    {
        let provider = self
            .config
            .providers
            .iter_mut()
            .find(|p| p.name == name)
            .ok_or_else(|| anyhow::anyhow!("供应商 '{}' 不存在", name))?;

        updater(provider)?;
        self.save()
    }

    // ==================== Active Model 操作 ====================

    /// 获取所有活跃模型映射
    pub fn get_all_active(&self) -> &HashMap<String, ActiveModel> {
        &self.config.active
    }

    /// 设置活跃模型
    pub fn set_active(
        &mut self,
        tool: &str,
        provider: &str,
        model: &str,
    ) -> anyhow::Result<()> {
        self.config
            .set_active(tool, provider.to_string(), model.to_string());
        self.save()
    }

    /// 获取指定 tool 的活跃模型
    pub fn get_active(&self, tool: &str) -> Option<&ActiveModel> {
        self.config.active.get(tool)
    }

    // ==================== 兼容性接口（供旧代码过渡使用）====================

    /// 将 provider + model 解析为 ModelConfig（供 AgentAdapter 使用）
    pub fn resolve_model_config(
        &self,
        provider_name: &str,
        model_name: &str,
    ) -> anyhow::Result<ModelConfig> {
        let provider = self
            .get_provider(provider_name)
            .ok_or_else(|| anyhow::anyhow!("供应商 '{}' 不存在", provider_name))?;
        provider.resolve_model_config(model_name)
    }

    /// 添加模型配置（兼容旧接口，自动创建 provider）
    pub fn add_model(&mut self, model: ModelConfig) -> anyhow::Result<()> {
        // 将旧的 ModelConfig 转换为 Provider
        let protocol = if model.base_url.contains("anthropic")
            || model.name.contains("anthropic")
            || model.name.contains("claude")
        {
            Protocol::Anthropic
        } else {
            Protocol::OpenAI
        };

        let provider = Provider::new(
            model.name.clone(),
            model.base_url.clone(),
            model.api_key.clone(),
            protocol,
            model.models.clone(),
        );
        self.add_provider(provider)
    }

    /// 获取所有模型配置（兼容旧接口，将 providers 转换为 ModelConfig 列表）
    pub fn list_models(&self) -> Vec<ModelConfig> {
        self.config
            .providers
            .iter()
            .map(|p| {
                ModelConfig::new(
                    p.name.clone(),
                    p.base_url.clone(),
                    p.api_key.clone(),
                    p.models.clone(),
                )
            })
            .collect()
    }

    /// 删除模型配置（兼容旧接口）
    pub fn remove_model(&mut self, name: &str) -> anyhow::Result<()> {
        self.remove_provider(name)
    }

    /// 编辑模型配置（兼容旧接口）
    pub fn edit_model<F>(&mut self, name: &str, updater: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut ModelConfig) -> anyhow::Result<()>,
    {
        // 先转换为 ModelConfig，执行更新，再写回
        let provider = self
            .config
            .providers
            .iter_mut()
            .find(|p| p.name == name)
            .ok_or_else(|| anyhow::anyhow!("模型配置 '{}' 不存在", name))?;

        let mut mc = ModelConfig::new(
            provider.name.clone(),
            provider.base_url.clone(),
            provider.api_key.clone(),
            provider.models.clone(),
        );

        updater(&mut mc)?;

        provider.base_url = mc.base_url;
        provider.api_key = mc.api_key;
        provider.models = mc.models;

        self.save()
    }

    /// 获取指定名称的模型配置（兼容旧接口）
    pub fn get_model(&self, name: &str) -> Option<ModelConfig> {
        self.get_provider(name).map(|p| {
            ModelConfig::new(
                p.name.clone(),
                p.base_url.clone(),
                p.api_key.clone(),
                p.models.clone(),
            )
        })
    }

    /// 检查模型配置是否存在（兼容旧接口）
    pub fn has_model(&self, name: &str) -> bool {
        self.has_provider(name)
    }

    /// 更新指定 agent 的激活模型（兼容旧接口）
    pub fn update_active_model(
        &mut self,
        agent_name: &str,
        model_name: &str,
    ) -> anyhow::Result<()> {
        // 旧格式 model_name 就是 provider 名称
        let provider = self
            .get_provider(model_name)
            .ok_or_else(|| anyhow::anyhow!("模型配置 '{}' 不存在", model_name))?;
        let model = provider
            .get_default_model()
            .unwrap_or("unknown")
            .to_string();
        self.set_active(agent_name, model_name, &model)
    }

    /// 获取所有激活的模型映射（兼容旧接口）
    pub fn get_all_active_models(&self) -> HashMap<String, String> {
        self.config
            .active
            .iter()
            .map(|(k, v)| (k.clone(), v.provider.clone()))
            .collect()
    }

    /// 加载所有模型配置为 HashMap（兼容旧接口）
    pub fn load_all(&self) -> HashMap<String, ModelConfig> {
        self.config
            .providers
            .iter()
            .map(|p| {
                (
                    p.name.clone(),
                    ModelConfig::new(
                        p.name.clone(),
                        p.base_url.clone(),
                        p.api_key.clone(),
                        p.models.clone(),
                    ),
                )
            })
            .collect()
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
