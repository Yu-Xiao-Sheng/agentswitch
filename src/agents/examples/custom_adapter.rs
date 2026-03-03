//! 自定义适配器示例
//!
//! 本示例展示如何为新的 Code Agent 工具实现自定义适配器
//!
//! ## 使用步骤
//!
//! 1. 将此文件复制为 `src/agents/mytool.rs`
//! 2. 将 `MyToolAdapter` 重命名为你的工具名称
//! 3. 实现所有必需的方法
//! 4. 在 `src/agents/mod.rs` 中注册你的适配器

use crate::agents::{AgentAdapter, Backup};
use crate::backup::BackupManager;
use crate::config::ModelConfig;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// 自定义工具适配器示例
///
/// 这个示例适配器假设你的工具：
/// - 可执行文件名为 `mytool`
/// - 配置文件位于 `~/.mytool/config.json`
/// - 使用 JSON 格式配置
pub struct MyToolAdapter;

impl MyToolAdapter {
    /// 创建新的适配器实例
    pub fn new() -> Self {
        Self
    }

    /// 获取工具配置目录
    fn config_dir(&self) -> Result<PathBuf> {
        let home = dirs::home_dir()
            .context("无法获取用户主目录")?;
        Ok(home.join(".mytool"))
    }
}

impl AgentAdapter for MyToolAdapter {
    fn name(&self) -> &str {
        "mytool"
    }

    fn detect(&self) -> Result<bool> {
        // 方法 1: 检查可执行文件
        if let Ok(Some(_)) = which::which("mytool") {
            return Ok(true);
        }

        // 方法 2: 检查 npm 全局安装
        if let Ok(home) = dirs::home_dir() {
            let npm_global = home.join(".npm-global")
                .join("bin")
                .join("mytool");
            if npm_global.exists() {
                return Ok(true);
            }
        }

        // 方法 3: 检查配置文件存在性
        Ok(self.config_dir()?.exists())
    }

    fn config_path(&self) -> Result<PathBuf> {
        Ok(self.config_dir()?.join("config.json"))
    }

    fn current_model(&self) -> Result<Option<String>> {
        let config_path = self.config_path()?;

        if !config_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&config_path)
            .context("读取配置文件失败")?;

        // 解析 JSON 配置
        let json: serde_json::Value = serde_json::from_str(&content)
            .context("解析配置文件失败")?;

        // 从配置中读取当前模型
        if let Some(model) = json.get("model").and_then(|v| v.as_str()) {
            return Ok(Some(model.to_string()));
        }

        Ok(None)
    }

    fn backup(&self) -> Result<Backup> {
        let config_path = self.config_path()?;
        let backup_manager = BackupManager::new()?;

        backup_manager.create_backup(
            self.name(),
            &config_path,
            "json"  // 配置文件格式
        )
    }

    fn apply(&self, model_config: &ModelConfig) -> Result<()> {
        let config_path = self.config_path()?;

        // 1. 读取现有配置或创建默认配置
        let mut config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .context("读取配置文件失败")?;
            serde_json::from_str::<serde_json::Value>(&content)
                .unwrap_or_else(|_| serde_json::json!({}))
        } else {
            // 创建默认配置
            serde_json::json!({})
        };

        // 2. 确保配置是对象类型
        if !config.is_object() {
            *config = serde_json::json!({});
        }

        let config_obj = config.as_object_mut()
            .context("配置根节点不是对象")?;

        // 3. 更新 API 配置字段
        config_obj.insert("api_key".to_string(),
                          serde_json::json!(model_config.api_key.clone()));
        config_obj.insert("base_url".to_string(),
                          serde_json::json!(model_config.base_url.clone()));
        config_obj.insert("model".to_string(),
                          serde_json::json!(model_config.model_id.clone()));

        // 4. 确保配置目录存在
        if let Some(config_dir) = config_path.parent() {
            fs::create_dir_all(config_dir)
                .context("创建配置目录失败")?;
        }

        // 5. 写入配置文件（使用 pretty 格式化）
        let content = serde_json::to_string_pretty(&config)
            .context("序列化配置失败")?;

        fs::write(&config_path, content)
            .context("写入配置文件失败")?;

        // 6. 设置文件权限为 0600（仅所有者可读写）
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&config_path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&config_path, perms)?;
        }

        Ok(())
    }

    fn restore(&self, backup: &Backup) -> Result<()> {
        let backup_manager = BackupManager::new()?;
        backup_manager.restore_backup(backup)
    }
}

impl Default for MyToolAdapter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 高级示例：支持多种配置格式
// ============================================================================

/// 示例：支持 YAML 配置的适配器
///
/// 某些工具可能使用 YAML 而非 JSON 格式
#[allow(dead_code)]
pub struct MyToolYamlAdapter;

#[allow(dead_code)]
impl MyToolYamlAdapter {
    fn apply_yaml(&self, model_config: &ModelConfig) -> Result<()> {
        use std::io::Write;

        let config_path = dirs::home_dir()
            .context("无法获取用户主目录")?
            .join(".mytool")
            .join("config.yaml");

        // 构建 YAML 配置
        let yaml_content = format!(
            r#"# MyTool Configuration
api_key: {}
base_url: {}
model: {}
"#,
            model_config.api_key,
            model_config.base_url,
            model_config.model_id
        );

        // 写入文件
        fs::write(&config_path, yaml_content)
            .context("写入 YAML 配置失败")?;

        Ok(())
    }
}

// ============================================================================
// 高级示例：支持多个配置文件
// ============================================================================

/// 示例：支持多个配置文件的适配器
///
/// 某些工具可能将配置分散在多个文件中
#[allow(dead_code)]
pub struct MyToolMultiFileAdapter;

#[allow(dead_code)]
impl MyToolMultiFileAdapter {
    fn apply_multi_file(&self, model_config: &ModelConfig) -> Result<()> {
        let config_dir = dirs::home_dir()
            .context("无法获取用户主目录")?
            .join(".mytool");

        // 创建配置目录
        fs::create_dir_all(&config_dir)
            .context("创建配置目录失败")?;

        // 1. 写入主配置文件
        let main_config = config_dir.join("config.json");
        let main_config_content = serde_json::json!({
            "model": model_config.model_id,
        });
        fs::write(&main_config, serde_json::to_string_pretty(&main_config_content)?)
            .context("写入主配置失败")?;

        // 2. 写入认证文件
        let auth_config = config_dir.join("auth.json");
        let auth_config_content = serde_json::json!({
            "api_key": model_config.api_key,
        });
        fs::write(&auth_config, serde_json::to_string_pretty(&auth_config_content)?)
            .context("写入认证配置失败")?;

        // 3. 写入环境配置文件
        let env_config = config_dir.join(".env");
        let env_content = format!(
            "MYTOOL_BASE_URL={}\nMYTOOL_MODEL={}\n",
            model_config.base_url,
            model_config.model_id
        );
        fs::write(&env_config, env_content)
            .context("写入环境配置失败")?;

        Ok(())
    }
}

// ============================================================================
// 高级示例：环境变量配置
// ============================================================================

/// 示例：通过环境变量配置的适配器
///
/// 某些工具主要通过环境变量读取配置
#[allow(dead_code)]
pub struct MyToolEnvAdapter;

#[allow(dead_code)]
impl MyToolEnvAdapter {
    fn apply_env(&self, model_config: &ModelConfig) -> Result<()> {
        let config_dir = dirs::home_dir()
            .context("无法获取用户主目录")?
            .join(".mytool");

        fs::create_dir_all(&config_dir)
            .context("创建配置目录失败")?;

        // 写入 .env 文件
        let env_path = config_dir.join(".env");
        let env_content = format!(
            r#"# MyTool Environment Variables
MYTOOL_API_KEY={}
MYTOOL_BASE_URL={}
MYTOOL_MODEL={}
"#,
            model_config.api_key,
            model_config.base_url,
            model_config.model_id
        );

        fs::write(&env_path, env_content)
            .context("写入 .env 文件失败")?;

        // 设置文件权限为 0600（包含敏感信息）
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&env_path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&env_path, perms)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter_name() {
        let adapter = MyToolAdapter::new();
        assert_eq!(adapter.name(), "mytool");
    }

    #[test]
    fn test_config_path() {
        let adapter = MyToolAdapter::new();
        let path = adapter.config_path().unwrap();
        assert!(path.ends_with(".mytool/config.json"));
    }

    #[test]
    fn test_current_model_no_config() {
        let adapter = MyToolAdapter::new();
        // 当配置文件不存在时，应返回 None
        let model = adapter.current_model().unwrap();
        assert!(model.is_none());
    }
}
