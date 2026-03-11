# 数据模型: 用户体验优化与高级功能

**功能分支**: `004-ux-optimization`
**创建日期**: 2026-03-10
**状态**: 完成

## 概述

本文档定义 Spec 004 中各功能模块的数据模型，包括交互式向导、工具检测、Shell 补全和 Git 同步的核心数据结构。

---

## 1. 交互式配置向导 (Wizard)

### 1.1 WizardState

向导状态，用于保存和恢复配置进度。

```rust
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// 向导状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardState {
    /// 当前步骤索引
    pub current_step: usize,

    /// 已完成的步骤索引列表
    pub completed_steps: Vec<usize>,

    /// 已收集的数据（字段名 -> 值）
    pub data: HashMap<String, String>,

    /// 状态保存时间
    pub timestamp: DateTime<Utc>,

    /// 向导类型（首次配置、添加模型等）
    pub wizard_type: WizardType,
}

/// 向导类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WizardType {
    /// 首次配置向导
    InitialSetup,

    /// 添加单个模型配置
    AddModel,

    /// 批量配置向导
    BatchSetup,
}
```

**字段说明**:
- `current_step`: 记录用户当前所处的步骤，用于恢复时跳转到正确位置
- `completed_steps`: 已完成的步骤列表，用于验证数据完整性
- `data`: 存储用户在各步骤输入的数据，使用 HashMap 灵活存储
- `timestamp`: 用于判断状态是否过期（如超过 24 小时则提示用户）
- `wizard_type`: 区分不同类型的向导，支持多种配置场景

**持久化**: 保存到 `~/.cache/agentswitch/wizard_state.toml`，文件权限 600

### 1.2 WizardStep

向导步骤定义。

```rust
/// 向导步骤
#[derive(Debug, Clone)]
pub struct WizardStep {
    /// 步骤 ID
    pub id: usize,

    /// 步骤名称
    pub name: String,

    /// 步骤描述
    pub description: String,

    /// 输入字段定义
    pub fields: Vec<InputField>,

    /// 是否可选
    pub optional: bool,
}

/// 输入字段
#[derive(Debug, Clone)]
pub struct InputField {
    /// 字段名称（用于存储到 data）
    pub name: String,

    /// 字段类型
    pub field_type: FieldType,

    /// 显示标签
    pub label: String,

    /// 帮助文本
    pub help_text: Option<String>,

    /// 默认值
    pub default: Option<String>,

    /// 验证规则
    pub validators: Vec<Validator>,
}

/// 字段类型
#[derive(Debug, Clone)]
pub enum FieldType {
    /// 单行文本
    Text,

    /// 密码（掩码显示）
    Password,

    /// 多行文本
    MultilineText,

    /// 确认（是/否）
    Confirm {
        default: bool,
    },

    /// 单选
    Select {
        options: Vec<String>,
    },

    /// 多选
    MultiSelect {
        options: Vec<String>,
    },
}

/// 验证器
#[derive(Debug, Clone)]
pub enum Validator {
    /// 必填
    Required,

    /// 最小长度
    MinLength(usize),

    /// 最大长度
    MaxLength(usize),

    /// URL 格式
    Url,

    /// 自定义验证函数
    Custom(Box<dyn Fn(&str) -> Result<(), String>>),
}
```

**使用示例**:
```rust
let steps = vec![
    WizardStep {
        id: 0,
        name: "模型名称".to_string(),
        description: "为模型配置指定一个易记的名称".to_string(),
        fields: vec![
            InputField {
                name: "model_name".to_string(),
                field_type: FieldType::Text,
                label: "模型配置名称".to_string(),
                help_text: Some("例如: glm, gpt-4, minimax".to_string()),
                default: None,
                validators: vec![
                    Validator::Required,
                    Validator::MinLength(2),
                    Validator::MaxLength(50),
                ],
            }
        ],
        optional: false,
    },
    // ... 更多步骤
];
```

---

## 2. 工具检测和健康检查 (Doctor)

### 2.1 ToolDetection

工具检测结果。

```rust
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
```

### 2.2 HealthCheckResult

健康检查结果。

```rust
/// 健康检查结果
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    /// 工具名称
    pub tool_name: String,

    /// 健康状态
    pub status: HealthStatus,

    /// 状态消息
    pub message: String,

    /// 修复建议
    pub suggestion: String,

    /// 错误详情（如果有）
    pub error_details: Option<String>,
}

/// 健康状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    /// 健康
    Healthy,

    /// 警告（非致命问题）
    Warning,

    /// 错误（需要修复）
    Error,
}
```

### 2.3 DoctorReport

诊断报告。

```rust
/// 完整诊断报告
#[derive(Debug, Clone)]
pub struct DoctorReport {
    /// 所有工具检测结果
    pub detections: Vec<ToolDetection>,

    /// 健康检查结果
    pub health_results: Vec<HealthCheckResult>,

    /// 系统信息
    pub system_info: SystemInfo,

    /// 检测时间
    pub timestamp: DateTime<Utc>,
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
```

---

## 3. Shell 补全 (Completion)

### 3.1 CompletionConfig

补全配置。

```rust
/// Shell 补全配置
#[derive(Debug, Clone)]
pub struct CompletionConfig {
    /// Shell 类型
    pub shell_type: ShellType,

    /// 补全脚本内容
    pub script_content: String,

    /// 安装路径
    pub install_path: PathBuf,

    /// Shell 配置文件路径
    pub shell_config_path: PathBuf,

    /// 需要添加到配置文件的内容
    pub config_append: Option<String>,
}

/// Shell 类型
#[derive(Debug, Clone, Copy)]
pub enum ShellType {
    /// Bash
    Bash,

    /// Zsh
    Zsh,

    /// Fish
    Fish,
}

impl ShellType {
    /// 获取 Shell 名称
    pub fn name(&self) -> &str {
        match self {
            ShellType::Bash => "bash",
            ShellType::Zsh => "zsh",
            ShellType::Fish => "fish",
        }
    }

    /// 获取补全脚本文件名
    pub fn completion_filename(&self) -> &str {
        match self {
            ShellType::Bash => "asw.bash",
            ShellType::Zsh => "_asw",
            ShellType::Fish => "asw.fish",
        }
    }

    /// 获取默认安装目录
    pub fn default_install_dir(&self) -> PathBuf {
        let home = dirs::home_dir().unwrap();
        match self {
            ShellType::Bash => home.join(".local/share/bash-completion/completions"),
            ShellType::Zsh => home.join(".zsh/completion"),
            ShellType::Fish => home.join(".config/fish/completions"),
        }
    }

    /// 获取配置文件路径
    pub fn config_file(&self) -> PathBuf {
        let home = dirs::home_dir().unwrap();
        match self {
            ShellType::Bash => home.join(".bashrc"),
            ShellType::Zsh => home.join(".zshrc"),
            ShellType::Fish => home.join(".config/fish/config.fish"),
        }
    }
}
```

### 3.2 DynamicCompletionData

动态补全数据（运行时生成）。

```rust
/// 动态补全数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicCompletionData {
    /// 已配置的模型列表
    pub models: Vec<String>,

    /// 已检测的工具列表
    pub tools: Vec<String>,

    /// 已保存的预设列表
    pub presets: Vec<String>,

    /// 数据生成时间
    pub generated_at: DateTime<Utc>,
}
```

**缓存位置**: `~/.cache/agentswitch/completion_cache.json`

---

## 4. Git 同步 (Sync)

### 4.1 SyncConfig

同步配置。

```rust
/// Git 同步配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    /// 远程仓库 URL
    pub remote_url: Option<String>,

    /// 远仓库名称
    pub remote_name: String,

    /// 分支名称
    pub branch: String,

    /// 加密配置
    pub encryption: EncryptionConfig,

    /// 用户信息
    pub user_info: GitUserInfo,
}

/// 加密配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// 是否启用加密
    pub enabled: bool,

    /// 加密方法
    pub method: EncryptionMethod,

    /// 密钥标识（用于查找密钥）
    pub key_id: Option<String>,
}

/// 加密方法
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionMethod {
    /// 不加密
    None,

    /// AES-GCM（使用密码）
    AesGcmPassword {
        /// 密码哈希（用于验证）
        password_hash: String,
        /// Salt
        salt: String,
    },

    /// git-crypt（如果可用）
    GitCrypt,
}

/// Git 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitUserInfo {
    /// 用户名
    pub name: String,

    /// 邮箱
    pub email: String,
}
```

**配置文件位置**: `~/.agentswitch/sync.toml`

### 4.2 SyncState

同步状态。

```rust
/// Git 同步状态
#[derive(Debug, Clone)]
pub struct SyncState {
    /// 是否已初始化 Git 仓库
    pub initialized: bool,

    /// 当前分支
    pub current_branch: Option<String>,

    /// 远程仓库状态
    pub remote_status: RemoteStatus,

    /// 本地与远程的差异
    pub divergence: Option<Divergence>,

    /// 加密状态
    pub encryption_status: EncryptionStatus,

    /// 最后同步时间
    pub last_sync: Option<DateTime<Utc>>,
}

/// 远程仓库状态
#[derive(Debug, Clone)]
pub enum RemoteStatus {
    /// 无远程仓库
    NoRemote,

    /// 远程仓库存在
    Connected {
        url: String,
        branch: String,
    },

    /// 无法连接
    Disconnected(String),
}

/// 本地与远程的差异
#[derive(Debug, Clone)]
pub struct Divergence {
    /// 本地独有提交数
    pub ahead: usize,

    /// 远程独有提交数
    pub behind: usize,

    /// 是否有冲突
    pub has_conflicts: bool,
}

/// 加密状态
#[derive(Debug, Clone)]
pub enum EncryptionStatus {
    /// 未加密
    NotEncrypted,

    /// 已加密
    Encrypted {
        method: EncryptionMethod,
        encrypted_files: Vec<String>,
    },

    /// 部分加密
    PartiallyEncrypted {
        encrypted_files: Vec<String>,
        unencrypted_files: Vec<String>,
    },

    /// 加密错误
    EncryptionError(String),
}
```

### 4.3 SyncResult

同步操作结果。

```rust
/// 同步操作结果
#[derive(Debug, Clone)]
pub struct SyncResult {
    /// 操作类型
    pub operation: SyncOperation,

    /// 是否成功
    pub success: bool,

    /// 变更的文件列表
    pub changed_files: Vec<String>,

    /// 新提交的哈希（如果有）
    pub new_commit: Option<String>,

    /// 冲突信息（如果有）
    pub conflicts: Option<Vec<ConflictInfo>>,

    /// 错误信息（如果失败）
    pub error: Option<String>,
}

/// 同步操作类型
#[derive(Debug, Clone)]
pub enum SyncOperation {
    /// 初始化
    Init,

    /// 添加远程仓库
    AddRemote,

    /// 推送
    Push,

    /// 拉取
    Pull,

    /// 获取状态
    Status,
}

/// 冲突信息
#[derive(Debug, Clone)]
pub struct ConflictInfo {
    /// 冲突文件路径
    pub file_path: String,

    /// 冲突类型
    pub conflict_type: ConflictType,

    /// 本地版本
    pub local_version: String,

    /// 远程版本
    pub remote_version: String,
}

/// 冲突类型
#[derive(Debug, Clone)]
pub enum ConflictType {
    /// 内容冲突
    Content,

    /// API Key 加密状态冲突
    EncryptionStatus,

    /// 结构冲突（如预设不存在）
    Structure,
}
```

---

## 5. 加密模块 (Crypto)

### 5.1 CryptoManager

加密管理器。

```rust
use aes_gcm::Aes256Gcm;

/// 加密管理器
pub struct CryptoManager {
    /// AES-GCM 密码器
    cipher: Aes256Gcm,

    /// 加密方法
    method: EncryptionMethod,
}

impl CryptoManager {
    /// 从密码创建加密管理器
    pub fn from_password(password: &str, salt: &[u8; 32]) -> anyhow::Result<Self>;

    /// 加密 API Key
    pub fn encrypt_api_key(&self, api_key: &str) -> anyhow::Result<String>;

    /// 解密 API Key
    pub fn decrypt_api_key(&self, encrypted: &str) -> anyhow::Result<String>;

    /// 加密整个配置文件
    pub fn encrypt_config(&self, config: &str) -> anyhow::Result<String>;

    /// 解密整个配置文件
    pub fn decrypt_config(&self, encrypted: &str) -> anyhow::Result<String>;
}
```

### 5.2 EncryptedValue

加密值标记。

```rust
/// 加密值标记
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedValue {
    /// 加密方法
    pub method: String,

    /// 加密数据（Base64）
    pub data: String,

    /// Nonce（Base64，用于 AES-GCM）
    pub nonce: Option<String>,
}

impl EncryptedValue {
    /// 创建加密值
    pub fn new(method: EncryptionMethod, data: Vec<u8>, nonce: Option<Vec<u8>>) -> Self;

    /// 尝试解密
    pub fn decrypt(&self, manager: &CryptoManager) -> anyhow::Result<String>;

    /// 序列化为字符串格式（如 "enc:AES256:..."）
    pub fn to_string(&self) -> String;

    /// 从字符串解析
    pub fn from_string(s: &str) -> anyhow::Result<Self>;
}
```

---

## 6. 数据流和状态转换

### 6.1 向导状态转换

```
[开始]
  ↓
[加载状态] → [有未完成状态?] → 是 → [询问是否恢复]
  ↓                                      ↓
否                                   是/否
  ↓                                      ↓
[欢迎页面]                          [恢复进度] / [重新开始]
  ↓
[步骤 1: 模型名称]
  ↓
[步骤 2: Base URL]
  ↓
[步骤 3: API Key]
  ↓
[步骤 4: Model ID]
  ↓
[确认信息]
  ↓
[保存配置]
  ↓
[完成] → [清理状态文件]
```

### 6.2 工具检测流程

```
[开始检测]
  ↓
[遍历已注册的工具]
  ↓
[检查可执行文件存在性] ─→ 不存在 → [标记为未安装]
  ↓ 存在
[获取工具版本]
  ↓
[查找配置文件]
  ↓
[读取并验证配置]
  ↓
[生成健康检查报告]
  ↓
[汇总所有工具结果]
  ↓
[生成最终报告]
```

### 6.3 Git 同步流程

```
[同步操作]
  ↓
[检查 Git 是否安装] ─→ 否 → [返回错误]
  ↓ 是
[检查仓库是否初始化] ─→ 否 → [返回错误]
  ↓ 是
[检查加密配置]
  ↓
[准备提交] → [加密敏感字段]
  ↓
[执行 Git 操作]
  ↓
[处理结果] ─→ 成功 → [返回成功]
  ↓ 失败
[处理冲突] ─→ 可解决 → [提供解决策略]
  ↓ 不可解决
[返回错误]
```

---

## 7. 文件组织

### 7.1 新增文件

```
~/.agentswitch/
├── config.toml              # 现有配置（不变）
├── models.toml              # 现有模型配置（不变）
├── presets.toml             # 现有预设配置（不变）
├── sync.toml                # [新增] Git 同步配置
└── .git/                    # [新增] Git 仓库目录
    ├── config               # Git 配置
    └── ...                  # 其他 Git 文件

~/.cache/agentswitch/
├── wizard_state.toml        # [新增] 向导状态
└── completion_cache.json    # [新增] 补全数据缓存

~/.local/share/bash-completion/completions/
└── asw.bash                 # [新增] Bash 补全脚本

~/.zsh/completion/
└── _asw                     # [新增] Zsh 补全脚本

~/.config/fish/completions/
└── asw.fish                 # [新增] Fish 补全脚本
```

### 7.2 配置文件格式

#### sync.toml

```toml
[remote]
url = "https://github.com/user/agentswitch-config.git"
name = "origin"
branch = "main"

[encryption]
enabled = true
method = "aes-gcm-password"
# password_hash 和 salt 实际存储在系统密钥链中

[user]
name = "AgentSwitch User"
email = "user@example.com"
```

#### wizard_state.toml

```toml
current_step = 2
completed_steps = [0, 1]
timestamp = "2026-03-10T10:30:00Z"
wizard_type = "InitialSetup"

[data]
model_name = "glm"
base_url = "https://open.bigmodel.cn/api/v1"
```

---

## 8. 数据验证规则

### 8.1 模型配置验证

```rust
pub fn validate_model_config(config: &ModelConfig) -> Result<Vec<ValidationError>> {
    let mut errors = Vec::new();

    // 名称验证
    if config.name.is_empty() {
        errors.push(ValidationError {
            field: "name".to_string(),
            message: "名称不能为空".to_string(),
        });
    }

    if config.name.len() > 50 {
        errors.push(ValidationError {
            field: "name".to_string(),
            message: "名称不能超过 50 个字符".to_string(),
        });
    }

    // URL 验证
    if let Err(e) = url::Url::parse(&config.base_url) {
        errors.push(ValidationError {
            field: "base_url".to_string(),
            message: format!("无效的 URL: {}", e),
        });
    }

    // API Key 验证
    if config.api_key.len() < 32 {
        errors.push(ValidationError {
            field: "api_key".to_string(),
            message: "API Key 长度不能少于 32 个字符".to_string(),
        });
    }

    // Model ID 验证
    if config.model_id.is_empty() {
        errors.push(ValidationError {
            field: "model_id".to_string(),
            message: "Model ID 不能为空".to_string(),
        });
    }

    if errors.is_empty() {
        Ok(errors)
    } else {
        Err(errors)
    }
}
```

### 8.2 加密配置验证

```rust
pub fn validate_encryption_config(config: &EncryptionConfig) -> Result<(), String> {
    if !config.enabled {
        return Ok(());
    }

    match config.method {
        EncryptionMethod::None => {
            Err("启用了加密但未选择加密方法".to_string())
        }
        EncryptionMethod::AesGcmPassword { .. } => {
            // 密码验证将在实际使用时进行
            Ok(())
        }
        EncryptionMethod::GitCrypt => {
            // 检查 git-crypt 是否可用
            if std::process::Command::new("git-crypt")
                .arg("--version")
                .output()
                .is_err()
            {
                Err("git-crypt 未安装".to_string())
            } else {
                Ok(())
            }
        }
    }
}
```

---

## 9. 错误处理

### 9.1 错误类型

```rust
use thiserror::Error;

/// 向导错误
#[derive(Error, Debug)]
pub enum WizardError {
    #[error("向导状态文件损坏: {0}")]
    CorruptedState(String),

    #[error("用户取消操作")]
    Cancelled,

    #[error("验证失败: {0}")]
    ValidationFailed(String),

    #[error("非交互式环境")]
    NotInteractive,
}

/// 同步错误
#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Git 未安装")]
    GitNotInstalled,

    #[error("不是 Git 仓库")]
    NotAGitRepository,

    #[error("远程仓库不可访问: {0}")]
    RemoteUnavailable(String),

    #[error("合并冲突")]
    MergeConflict,

    #[error("加密失败: {0}")]
    EncryptionError(String),

    #[error("解密失败: {0}")]
    DecryptionError(String),
}

/// 工具检测错误
#[derive(Error, Debug)]
pub enum DoctorError {
    #[error("工具 {0} 检测失败: {1}")]
    DetectionFailed(String, String),

    #[error("配置文件读取失败: {0}")]
    ConfigReadError(String),

    #[error("权限不足: {0}")]
    PermissionDenied(String),
}
```

---

## 10. 总结

### 关键数据结构汇总

| 模块 | 核心数据结构 | 持久化位置 |
|------|-------------|-----------|
| 向导 | `WizardState` | `~/.cache/agentswitch/wizard_state.toml` |
| 工具检测 | `ToolDetection`, `DoctorReport` | 无（运行时生成） |
| Shell 补全 | `CompletionConfig`, `DynamicCompletionData` | `~/.cache/agentswitch/completion_cache.json` |
| Git 同步 | `SyncConfig`, `SyncState` | `~/.agentswitch/sync.toml` |
| 加密 | `CryptoManager`, `EncryptedValue` | 系统密钥链 |

### 数据模型特点

1. **类型安全**: 使用 Rust 类型系统确保数据完整性
2. **序列化支持**: 所有持久化结构实现 `Serialize`/`Deserialize`
3. **验证友好**: 内置验证规则和错误类型
4. **扩展性**: 使用枚举和 trait 支持未来扩展
5. **安全性**: 敏感数据加密存储，文件权限控制

### 与现有模块的集成

- **复用** `ModelConfig`（Spec 001）
- **复用** `AgentAdapter`（Spec 002）
- **复用** `Preset`（Spec 003）
- **扩展** CLI 命令结构，添加新子命令

---

**文档完成日期**: 2026-03-10
**状态**: ✅ 数据模型设计完成
