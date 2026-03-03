# Agent Adapter 实现指南

**功能分支**: `002-agent-adapter` | **目标受众**: 开发者 | **最后更新**: 2026-02-28

## 目录

1. [概述](#概述)
2. [Adapter Trait 契约](#adapter-trait-契约)
3. [完整实现示例](#完整实现示例)
4. [配置格式处理](#配置格式处理)
5. [测试指南](#测试指南)
6. [最佳实践](#最佳实践)
7. [常见问题](#常见问题)

---

## 概述

Agent Adapter 是 AgentSwitch 的核心抽象，用于统一不同 Code Agent 工具的配置管理。每个适配器负责：

- ✅ 检测工具是否已安装
- ✅ 定位配置文件路径
- ✅ 读取当前配置
- ✅ 应用新配置
- ✅ 创建配置备份
- ✅ 恢复配置备份

### 核心原则

1. **零破坏**: 所有操作必须先备份后修改
2. **原子性**: 配置更新要么完全成功，要么完全失败
3. **幂等性**: 多次应用相同配置不应产生副作用
4. **可恢复**: 所有变更都必须可回滚

---

## Adapter Trait 契约

所有适配器都必须实现 `AgentAdapter` trait：

```rust
use anyhow::Result;
use std::path::PathBuf;

/// Agent 配置备份信息
pub struct Backup {
    pub agent_name: String,
    pub original_config_path: PathBuf,
    pub backup_path: PathBuf,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Agent 适配器核心 trait
#[async_trait]
pub trait AgentAdapter: Send + Sync {
    /// 返回工具名称（如 "claude-code"）
    fn name(&self) -> &str;

    /// 检测工具是否已安装
    fn detect(&self) -> Result<bool>;

    /// 获取配置文件路径
    fn config_path(&self) -> Result<PathBuf>;

    /// 读取当前使用的模型
    fn current_model(&self) -> Result<Option<String>>;

    /// 创建配置备份
    fn backup(&self) -> Result<Backup>;

    /// 应用新的模型配置
    fn apply(&self, model_config: &ModelConfig) -> Result<()>;

    /// 恢复配置备份
    fn restore(&self, backup: &Backup) -> Result<()>;
}
```

---

## 完整实现示例

### 示例 1: Claude Code 适配器 (JSON 配置)

```rust
use crate::agents::{AgentAdapter, Backup, ModelConfig};
use crate::backup::BackupManager;
use anyhow::{Context, Result};
use serde_json::{json, Map, Value};
use std::fs;
use std::path::PathBuf;

pub struct ClaudeCodeAdapter;

impl ClaudeCodeAdapter {
    pub fn new() -> Self {
        Self
    }

    /// 获取 Claude Code 配置文件路径
    fn config_path(&self) -> Result<PathBuf> {
        let home = dirs::home_dir()
            .context("无法获取用户主目录")?;
        Ok(home.join(".claude").join("settings.json"))
    }
}

impl AgentAdapter for ClaudeCodeAdapter {
    fn name(&self) -> &str {
        "claude-code"
    }

    fn detect(&self) -> Result<bool> {
        // 方法 1: 检查可执行文件
        if let Ok(Some(_)) = which::which("claude-code") {
            return Ok(true);
        }

        // 方法 2: 检查全局 npm 安装
        if let Ok(home) = dirs::home_dir() {
            let npm_global = home.join(".npm-global")
                .join("bin")
                .join("claude-code");
            if npm_global.exists() {
                return Ok(true);
            }
        }

        // 方法 3: 检查配置文件存在性
        Ok(self.config_path()?.exists())
    }

    fn config_path(&self) -> Result<PathBuf> {
        let home = dirs::home_dir()
            .context("无法获取用户主目录")?;
        Ok(home.join(".claude").join("settings.json"))
    }

    fn current_model(&self) -> Result<Option<String>> {
        let config_path = self.config_path()?;

        if !config_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&config_path)
            .context("读取配置文件失败")?;

        let json: Value = serde_json::from_str(&content)
            .context("解析配置文件失败")?;

        // 从 env.ANTHROPIC_MODEL 读取模型
        if let Some(env) = json.get("env").and_then(|v| v.as_object()) {
            if let Some(model) = env.get("ANTHROPIC_MODEL").and_then(|v| v.as_str()) {
                return Ok(Some(model.to_string()));
            }
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
            serde_json::from_str::<Value>(&content)
                .unwrap_or_else(|_| json!({}))
        } else {
            json!({})
        };

        // 2. 确保 env 对象存在
        if !config.is_object() {
            *config = json!({});
        }

        let env_obj = config.as_object_mut()
            .context("配置根节点不是对象")?;

        if !env_obj.contains_key("env") {
            env_obj.insert("env".to_string(), json!({}));
        }

        let env = env_obj.get_mut("env")
            .and_then(|v| v.as_object_mut())
            .context("env 字段不是对象")?;

        // 3. 更新配置字段
        env.insert("ANTHROPIC_AUTH_TOKEN".to_string(),
                   json!(model_config.api_key.clone()));
        env.insert("ANTHROPIC_BASE_URL".to_string(),
                   json!(model_config.base_url.clone()));
        env.insert("ANTHROPIC_MODEL".to_string(),
                   json!(model_config.model_id.clone()));

        // 4. 写回文件（使用 pretty 格式化）
        let content = serde_json::to_string_pretty(&config)
            .context("序列化配置失败")?;

        fs::write(&config_path, content)
            .context("写入配置文件失败")?;

        Ok(())
    }

    fn restore(&self, backup: &Backup) -> Result<()> {
        let backup_manager = BackupManager::new()?;
        backup_manager.restore_backup(backup)
    }
}
```

### 示例 2: Codex 适配器 (TOML + JSON 双文件)

```rust
use crate::agents::{AgentAdapter, Backup, ModelConfig};
use crate::backup::BackupManager;
use anyhow::{Context, Result};
use serde_json::json;
use std::fs;
use std::path::PathBuf;

pub struct CodexAdapter;

impl CodexAdapter {
    pub fn new() -> Self {
        Self
    }

    /// 获取 Codex 配置文件路径
    fn config_dir(&self) -> Result<PathBuf> {
        let home = dirs::home_dir()
            .context("无法获取用户主目录")?;
        Ok(home.join(".codex"))
    }

    /// 获取 config.toml 路径
    fn config_toml_path(&self) -> Result<PathBuf> {
        Ok(self.config_dir()?.join("config.toml"))
    }

    /// 获取 auth.json 路径
    fn auth_json_path(&self) -> Result<PathBuf> {
        Ok(self.config_dir()?.join("auth.json"))
    }
}

/// Codex TOML 配置结构
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
struct CodexConfig {
    #[serde(rename = "model_provider")]
    model_provider: Option<String>,

    #[serde(rename = "model")]
    model: Option<String>,

    #[serde(rename = "model_providers")]
    model_providers: Option<CodexModelProviders>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
struct CodexModelProviders {
    #[serde(rename = "custom_provider")]
    custom_provider: Option<CodexCustomProvider>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CodexCustomProvider {
    #[serde(rename = "base_url")]
    base_url: Option<String>,

    #[serde(rename = "model")]
    model: Option<String>,
}

/// Codex JSON 认证结构
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
struct CodexAuth {
    #[serde(rename = "OPENAI_API_KEY")]
    openai_api_key: Option<String>,
}

impl AgentAdapter for CodexAdapter {
    fn name(&self) -> &str {
        "codex"
    }

    fn detect(&self) -> Result<bool> {
        // 检查配置目录是否存在
        Ok(self.config_dir()?.exists())
    }

    fn config_path(&self) -> Result<PathBuf> {
        // 返回主配置文件路径
        self.config_toml_path()
    }

    fn current_model(&self) -> Result<Option<String>> {
        let config_path = self.config_toml_path()?;

        if !config_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&config_path)
            .context("读取配置文件失败")?;

        let config: CodexConfig = toml::from_str(&content)
            .context("解析 TOML 配置失败")?;

        // 从 model_provider.custom_provider.model 读取
        if let Some(providers) = config.model_providers {
            if let Some(custom) = providers.custom_provider {
                if let Some(model) = custom.model {
                    return Ok(Some(model));
                }
            }
        }

        // 回退到顶级 model 字段
        if let Some(model) = config.model {
            return Ok(Some(model));
        }

        Ok(None)
    }

    fn backup(&self) -> Result<Backup> {
        let config_path = self.config_toml_path()?;
        let backup_manager = BackupManager::new()?;

        backup_manager.create_backup(
            self.name(),
            &config_path,
            "toml"  // 配置文件格式
        )
    }

    fn apply(&self, model_config: &ModelConfig) -> Result<()> {
        let config_path = self.config_toml_path()?;
        let auth_path = self.auth_json_path()?;

        // 1. 更新 config.toml
        let mut config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .context("读取 config.toml 失败")?;
            toml::from_str(&content)
                .unwrap_or_default()
        } else {
            CodexConfig::default()
        };

        // 设置模型提供商
        config.model_provider = Some("custom_provider".to_string());
        config.model = Some(model_config.model_id.clone());

        // 设置自定义提供商配置
        let custom_provider = CodexCustomProvider {
            base_url: Some(model_config.base_url.clone()),
            model: Some(model_config.model_id.clone()),
        };

        config.model_providers = Some(CodexModelProviders {
            custom_provider: Some(custom_provider),
        });

        // 写回 TOML
        let toml_str = toml::to_string_pretty(&config)
            .context("序列化 TOML 配置失败")?;

        fs::write(&config_path, toml_str)
            .context("写入 config.toml 失败")?;

        // 2. 更新 auth.json
        let mut auth = if auth_path.exists() {
            let content = fs::read_to_string(&auth_path)
                .context("读取 auth.json 失败")?;
            serde_json::from_str(&content)
                .unwrap_or_default()
        } else {
            CodexAuth::default()
        };

        auth.openai_api_key = Some(model_config.api_key.clone());

        // 写回 JSON
        let json_str = serde_json::to_string_pretty(&auth)
            .context("序列化 JSON 认证失败")?;

        fs::write(&auth_path, json_str)
            .context("写入 auth.json 失败")?;

        Ok(())
    }

    fn restore(&self, backup: &Backup) -> Result<()> {
        let backup_manager = BackupManager::new()?;
        backup_manager.restore_backup(backup)
    }
}
```

### 示例 3: Gemini CLI 适配器 (JSON + .env)

```rust
use crate::agents::{AgentAdapter, Backup, ModelConfig};
use crate::backup::BackupManager;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct GeminiCliAdapter;

impl GeminiCliAdapter {
    pub fn new() -> Self {
        Self
    }

    /// 获取 Gemini 配置目录
    fn config_dir(&self) -> Result<PathBuf> {
        let home = dirs::home_dir()
            .context("无法获取用户主目录")?;
        Ok(home.join(".gemini"))
    }

    /// 获取 settings.json 路径
    fn settings_json_path(&self) -> Result<PathBuf> {
        Ok(self.config_dir()?.join("settings.json"))
    }

    /// 获取 .env 文件路径
    fn env_file_path(&self) -> Result<PathBuf> {
        Ok(self.config_dir()?.join(".env"))
    }
}

/// Gemini Settings 结构
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
struct GeminiSettings {
    #[serde(rename = "defaultModel")]
    default_model: Option<String>,
}

impl AgentAdapter for GeminiCliAdapter {
    fn name(&self) -> &str {
        "gemini-cli"
    }

    fn detect(&self) -> Result<bool> {
        // 检查配置目录
        Ok(self.config_dir()?.exists())
    }

    fn config_path(&self) -> Result<PathBuf> {
        self.settings_json_path()
    }

    fn current_model(&self) -> Result<Option<String>> {
        let config_path = self.settings_json_path()?;

        if !config_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&config_path)
            .context("读取配置文件失败")?;

        let settings: GeminiSettings = serde_json::from_str(&content)
            .context("解析 JSON 配置失败")?;

        Ok(settings.default_model)
    }

    fn backup(&self) -> Result<Backup> {
        let config_path = self.settings_json_path()?;
        let backup_manager = BackupManager::new()?;

        // 注意：备份目录名称应该是 "gemini-cli" 而不是 "gemini"
        backup_manager.create_backup(
            self.name(),  // 使用 "gemini-cli"
            &config_path,
            "json"
        )
    }

    fn apply(&self, model_config: &ModelConfig) -> Result<()> {
        let settings_path = self.settings_json_path()?;
        let env_path = self.env_file_path()?;

        // 1. 更新 settings.json
        let mut settings = if settings_path.exists() {
            let content = fs::read_to_string(&settings_path)
                .context("读取 settings.json 失败")?;
            serde_json::from_str(&content)
                .unwrap_or_default()
        } else {
            GeminiSettings::default()
        };

        settings.default_model = Some(model_config.model_id.clone());

        let json_str = serde_json::to_string_pretty(&settings)
            .context("序列化 JSON 配置失败")?;

        fs::write(&settings_path, json_str)
            .context("写入 settings.json 失败")?;

        // 2. 更新 .env 文件
        let env_content = format!(
            "GOOGLE_GEMINI_BASE_URL={}\nGEMINI_API_KEY={}\nGEMINI_MODEL={}\n",
            model_config.base_url,
            model_config.api_key,
            model_config.model_id
        );

        fs::write(&env_path, env_content)
            .context("写入 .env 文件失败")?;

        // 3. 设置 .env 文件权限为 0600
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&env_path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&env_path, perms)?;
        }

        Ok(())
    }

    fn restore(&self, backup: &Backup) -> Result<()> {
        let backup_manager = BackupManager::new()?;
        backup_manager.restore_backup(backup)
    }
}
```

---

## 配置格式处理

### JSON 配置

```rust
use serde_json::{json, Value};

// 读取 JSON 配置
let content = fs::read_to_string(path)?;
let config: Value = serde_json::from_str(&content)?;

// 更新字段
if let Some(obj) = config.as_object_mut() {
    obj.insert("key".to_string(), json!("value"));
}

// 写回（带格式化）
let pretty = serde_json::to_string_pretty(&config)?;
fs::write(path, pretty)?;
```

### TOML 配置

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
struct Config {
    #[serde(rename = "field_name")]
    field_name: Option<String>,
}

// 读取 TOML
let content = fs::read_to_string(path)?;
let config: Config = toml::from_str(&content)?;

// 修改
config.field_name = Some("value".to_string());

// 写回（自动格式化）
let toml_str = toml::to_string_pretty(&config)?;
fs::write(path, toml_str)?;
```

### .env 文件

```rust
// 写入 .env 文件
let env_content = format!(
    "KEY1={}\nKEY2={}\n",
    value1, value2
);
fs::write(env_path, env_content)?;

// 读取 .env 文件（使用 dotenv）
use dotenv::dotenv;
dotenv().ok();
let value = std::env::var("KEY1")?;
```

---

## 测试指南

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_config(temp_dir: &Path, content: &str) -> PathBuf {
        let config_path = temp_dir.join("settings.json");
        fs::write(&config_path, content).unwrap();
        config_path
    }

    #[test]
    fn test_adapter_name() {
        let adapter = ClaudeCodeAdapter::new();
        assert_eq!(adapter.name(), "claude-code");
    }

    #[test]
    fn test_current_model() {
        let adapter = ClaudeCodeAdapter::new();
        let temp_dir = TempDir::new().unwrap();

        // 创建测试配置
        let config_content = r#"{
            "env": {
                "ANTHROPIC_MODEL": "test-model"
            }
        }"#;
        create_test_config(temp_dir.path(), config_content);

        // 注意：需要 mock config_path() 方法返回测试路径
        // 这通常需要依赖注入或 trait mock
    }
}
```

### 集成测试

```rust
// tests/integration/adapter_integration_test.rs
use agentswitch::agents::ClaudeCodeAdapter;
use agentswitch::config::ModelConfig;

#[test]
fn test_full_workflow() {
    // 1. 检测工具
    let adapter = ClaudeCodeAdapter::new();
    let is_installed = adapter.detect().unwrap();
    if !is_installed {
        println!("Tool not installed, skipping test");
        return;
    }

    // 2. 创建测试配置
    let model_config = ModelConfig::new(
        "test-model".to_string(),
        "https://api.test.com".to_string(),
        "test-key".to_string(),
        "test-model-id".to_string(),
    );

    // 3. 备份原配置
    let backup = adapter.backup().unwrap();

    // 4. 应用新配置
    adapter.apply(&model_config).unwrap();

    // 5. 验证配置
    let current = adapter.current_model().unwrap();
    assert_eq!(current, Some("test-model-id".to_string()));

    // 6. 恢复原配置
    adapter.restore(&backup).unwrap();
}
```

---

## 最佳实践

### 1. 错误处理

```rust
// ✅ 好的做法：提供上下文
fn config_path(&self) -> Result<PathBuf> {
    dirs::home_dir()
        .context("无法获取用户主目录")
        .map(|p| p.join(".tool").join("config.json"))
}

// ❌ 不好的做法：裸错误
fn config_path(&self) -> Result<PathBuf> {
    Ok(dirs::home_dir()?.join(".tool").join("config.json"))
}
```

### 2. 配置合并

```rust
// ✅ 好的做法：保留现有字段
let mut config = if config_path.exists() {
    let content = fs::read_to_string(&config_path)?;
    serde_json::from_str(&content).unwrap_or_else(|_| json!({}))
} else {
    json!({})
};

// 只更新需要的字段
config["env"]["ANTHROPIC_MODEL"] = json!(new_model);

// ❌ 不好的做法：覆盖整个配置
let config = json!({
    "env": {
        "ANTHROPIC_MODEL": new_model
    }
});
```

### 3. 备份策略

```rust
// ✅ 好的做法：总是先备份
if config_path.exists() {
    let _backup = self.backup()?;
}

// 然后修改配置
self.apply(&model_config)?;

// ❌ 不好的做法：直接修改
self.apply(&model_config)?;
```

### 4. 权限处理

```rust
// ✅ 好的做法：设置安全权限
#[cfg(unix)]
{
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(&config_path)?.permissions();
    perms.set_mode(0o600);  // 仅所有者可读写
    fs::set_permissions(&config_path, perms)?;
}
```

---

## 常见问题

### Q1: 如何处理非标准配置路径？

```rust
fn config_path(&self) -> Result<PathBuf> {
    // 1. 优先检查环境变量
    if let Ok(custom_path) = std::env::var("MY_TOOL_CONFIG_PATH") {
        return Ok(PathBuf::from(custom_path));
    }

    // 2. 检查 XDG 配置目录
    if let Ok(config_dir) = dirs::config_dir() {
        let xdg_path = config_dir.join("mytool").join("config.json");
        if xdg_path.exists() {
            return Ok(xdg_path);
        }
    }

    // 3. 使用默认路径
    let default = dirs::home_dir()?
        .join(".mytool")
        .join("config.json");

    Ok(default)
}
```

### Q2: 如何处理嵌套配置结构？

```rust
// 使用 serde 的 rename 功能
#[derive(Debug, Serialize, Deserialize)]
struct NestedConfig {
    #[serde(rename = "parent")]
    parent: Option<ParentConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ParentConfig {
    #[serde(rename = "child")]
    child: Option<ChildConfig>,
}

// 手动访问嵌套字段
if let Some(parent) = config.parent.as_mut() {
    if let Some(child) = parent.child.as_mut() {
        child.field = Some("value".to_string());
    }
}
```

### Q3: 如何验证配置文件格式？

```rust
fn validate_config(path: &Path) -> Result<()> {
    let content = fs::read_to_string(path)?;

    // 尝试解析
    let _: Value = serde_json::from_str(&content)
        .context("配置文件格式无效")?;

    Ok(())
}

// 使用
if let Err(e) = validate_config(&config_path) {
    eprintln!("警告: 配置文件格式错误: {}", e);
}
```

### Q4: 如何处理配置文件损坏？

```rust
fn apply(&self, model_config: &ModelConfig) -> Result<()> {
    let config_path = self.config_path()?;

    // 如果配置文件损坏，先备份，然后重新创建
    if config_path.exists() {
        // 尝试解析
        let content = fs::read_to_string(&config_path)?;
        if serde_json::from_str::<Value>(&content).is_err() {
            // 配置损坏，备份损坏的文件
            let backup_path = config_path.with_extension("corrupted.bak");
            fs::copy(&config_path, &backup_path)?;
            eprintln!("警告: 配置文件损坏，已备份到 {:?}", backup_path);
        }
    }

    // 继续应用配置...
}
```

---

## 注册新适配器

在 `src/agents/mod.rs` 中注册你的适配器：

```rust
pub mod mytool;

use mytool::MyToolAdapter;

pub fn all_adapters() -> Vec<Box<dyn AgentAdapter>> {
    vec![
        Box::new(ClaudeCodeAdapter::new()),
        Box::new(CodexAdapter::new()),
        Box::new(GeminiCliAdapter::new()),
        Box::new(MyToolAdapter::new()),  // 添加新适配器
    ]
}
```

---

## 下一步

- 查看现有实现：`src/agents/`
- 阅读测试示例：`tests/unit/agents/`
- 了解备份系统：`src/backup/manager.rs`
- 查看命令契约：`specs/002-agent-adapter/contracts/cli-commands.md`

---

## 贡献指南

如果你为新的 Code Agent 工具实现了适配器，请：

1. ✅ 遵循本文档的实现指南
2. ✅ 添加完整的单元测试
3. ✅ 更新 quickstart.md 添加工具使用示例
4. ✅ 更新 research.md 添加工具配置研究
5. ✅ 提交 PR 时参考本指南

---

## 相关资源

- [serde_json 文档](https://docs.rs/serde_json/)
- [toml crate 文档](https://docs.rs/toml/)
- [anyhow 错误处理](https://docs.rs/anyhow/)
- [AgentSwitch 项目 README](https://github.com/Yu-Xiao-Sheng/agentswitch)
