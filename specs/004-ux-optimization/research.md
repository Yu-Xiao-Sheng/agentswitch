# 研究文档: 用户体验优化与高级功能

**功能分支**: `004-ux-optimization`
**创建日期**: 2026-03-10
**状态**: 完成

## 研究目标

本文档研究 Spec 004 实施过程中的关键技术选择和最佳实践，包括：
1. 交互式终端输入库的选择
2. Shell 补全脚本生成方案
3. Git 集成和配置同步方案
4. API Key 加密存储方案
5. 跨平台工具检测实现

---

## 1. 交互式终端输入库

### 需求分析
- 友好的 CLI 交互式向导
- 支持多种输入类型（文本、密码、选择、确认）
- 实时输入验证
- 支持 TTY 终端检测
- 跨平台支持（Linux/macOS/Windows）

### 评估的选项

#### 选项 A: inquire
**优点**:
- 现代化的 API 设计
- 内置验证支持
- 丰富的输入类型（Text、Password、Confirm、Select、MultiSelect）
- 自动处理 TTY 检测
- 良好的彩色输出支持
- 与 clap 4.x 兼容性好

**缺点**:
- 相对较新的库（但活跃维护中）
- 社区相对较小

**版本**: 0.7.5

#### 选项 B: dialoguer
**优点**:
- 成熟稳定的库
- 广泛使用（ripgrep、gitui 等知名项目使用）
- 丰富的功能集
- 良好的文档

**缺点**:
- API 相对老旧
- 验证支持不如 inquire 便捷
- 某些高级功能需要额外代码

**版本**: 0.11.0

#### 选项 C: console (interactive)
**优点**:
- 功能全面（包含终端操作、颜色、样式）
- 与其他 console 模块集成良好

**缺点**:
- 主要用于终端操作，交互式输入功能有限
- 需要更多手动实现

### 决策: 选择 **inquire**

**理由**:
1. **现代化 API**: inquire 提供了更符合 Rust 2024 标准的 API
2. **内置验证**: 支持自定义验证器，完美匹配实时验证需求（FR-003）
3. **TTY 检测**: 自动处理非交互式环境（FR-007）
4. **密码掩码**: 原生支持密码输入的掩码显示（FR-004）
5. **类型丰富**: 支持 Select（预设选择）、Confirm（确认退出）等高级类型

**使用示例**:
```rust
use inquire::{Text, Password, Confirm, CustomType};

// 文本输入（带验证）
let model_name = Text::new("模型配置名称:")
    .with_validator(|s: &str| {
        if s.is_empty() {
            Ok(inquire::validator::Validation::Invalid("名称不能为空".into()))
        } else {
            Ok(inquire::validator::Validation::Valid)
        }
    })
    .prompt()?;

// URL 输入（带格式验证）
let base_url = CustomType::<String>::new("Base URL:")
    .with_formatter(&|s| format!("{}", s))
    .with_error_message("请输入有效的 URL")
    .prompt()?;

// 密码输入（自动掩码）
let api_key = Password::new("API Key:")
    .with_validator(min_length!(32))
    .prompt()?;

// 确认对话框
let save = Confirm::new("是否保存配置?")
    .with_default(false)
    .prompt()?;
```

**依赖添加**:
```toml
[dependencies]
inquire = "0.7"
```

---

## 2. Shell 补全脚本生成方案

### 需求分析
- 支持 Bash、Zsh、Fish 三种 Shell
- 生成静态补全脚本（命令、子命令结构）
- 支持动态补全（模型名称、工具名称、预设名称）
- 自动安装到 Shell 配置文件
- 支持卸载功能

### 评估的选项

#### 选项 A: 使用 clap_generate
**优点**:
- clap 官方提供的补全生成工具
- 自动从 clap Derive 宏生成补全脚本
- 支持多种 Shell
- 维护良好，与项目现有架构完美集成

**缺点**:
- 动态补全需要自定义脚本
- 需要额外的补全命令实现

**实现方案**:
1. 使用 `clap_complete` 生成静态补全
2. 创建自定义补全脚本处理动态补全

#### 选项 B: 手写补全脚本
**优点**:
- 完全控制补全逻辑
- 可以实现复杂的动态补全

**缺点**:
- 维护成本高
- 容易与实际命令定义不同步
- 需要为每个 Shell 单独维护

#### 选项 C: 使用第三方框架（如 shellingham）
**优点**:
- 检测当前 Shell
- 提供一些通用功能

**缺点**:
- 不直接生成补全脚本
- 仍需手动实现补全逻辑

### 决策: 选择 **clap_complete + 自定义动态补全**

**理由**:
1. **自动同步**: clap_complete 从 clap 定义自动生成，保证与命令定义一致
2. **维护成本低**: 命令变更时补全自动更新
3. **混合方案**: 静态补全自动生成，动态补全通过自定义脚本实现

**架构设计**:
```
src/completion/
├── mod.rs
├── static.rs     # 使用 clap_complete 生成静态补全
├── dynamic.rs    # 动态补全逻辑（读取配置文件）
└── install.rs    # 安装/卸载脚本
```

**实现细节**:

1. **静态补全生成**:
```rust
use clap_complete::{generate_to, shells::Bash, shells::Fish, shells::Zsh};
use std::fs;

fn generate_completions(out_dir: &std::path::Path) -> anyhow::Result<()> {
    let cmd = Cli::command();
    let shells = [Bash, Zsh, Fish];

    for shell in shells {
        generate_to(shell, &mut cmd.clone(), "asw", out_dir)?;
    }

    Ok(())
}
```

2. **动态补全脚本** (Bash):
```bash
_asw_dynamic_completion() {
    local cur prev words cword
    _init_completion || return

    case "${prev}" in
        asw)
            # 静态补全已处理
            ;;
        switch)
            # 补全工具名称（从配置文件读取）
            local agents=$(asw agent list --json 2>/dev/null | jq -r '.[].name')
            COMPREPLY=($(compgen -W "${agents}" -- "${cur}"))
            ;;
        asw|switch|apply)
            # 补全模型名称（从配置文件读取）
            local models=$(asw model list --json 2>/dev/null | jq -r '.[].name')
            COMPREPLY=($(compgen -W "${models}" -- "${cur}"))
            ;;
    esac
}
```

3. **安装脚本**:
```rust
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn install_completion(shell: &str) -> anyhow::Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow!("无法找到主目录"))?;
    let (script_path, config_file) = match shell {
        "bash" => (
            home.join(".local/share/bash-completion/completions/asw"),
            home.join(".bashrc"),
        ),
        "zsh" => (
            home.join(".zsh/completion/_asw"),
            home.join(".zshrc"),
        ),
        "fish" => (
            home.join(".config/fish/completions/asw.fish"),
            home.join(".config/fish/config.fish"),
        ),
        _ => return Err(anyhow!("不支持的 Shell: {}", shell)),
    };

    // 创建补全脚本
    fs::create_dir_all(script_path.parent().unwrap())?;
    generate_completion_script(&script_path, shell)?;

    // 添加到 Shell 配置文件
    let source_line = match shell {
        "bash" => "source ~/.local/share/bash-completion/completions/asw",
        "zsh" => "fpath=(~/.zsh/completion $fpath)\nautoload -U compinit && compinit",
        "fish" => "",  // Fish 自动发现
        _ => "",
    };

    if !source_line.is_empty() {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&config_file)?;
        writeln!(file, "\n# AgentSwitch completion\n{}", source_line)?;
    }

    Ok(script_path)
}
```

**依赖添加**:
```toml
[dependencies]
clap_complete = "4.5"
```

---

## 3. Git 集成和配置同步方案

### 需求分析
- 将配置目录初始化为 Git 仓库
- 支持远程仓库管理
- 推送/拉取配置
- 配置冲突解决
- API Key 加密

### 评估的选项

#### 选项 A: git2 crate (Rust Git 库)
**优点**:
- 纯 Rust 实现，无需外部依赖
- 类型安全
- 更好的错误处理
- 可以直接操作 Git 内部

**缺点**:
- API 相对复杂
- 学习曲线较陡
- 某些高级功能支持有限

**版本**: 0.18

#### 选项 B: 调用 Git CLI (Command)
**优点**:
- 简单直接
- 利用现有 Git 安装
- 用户熟悉的 Git 命令
- 易于调试

**缺点**:
- 需要系统已安装 Git
- 依赖外部进程
- 错误处理相对复杂
- 性能稍差

#### 选项 C: gitplex (高级 Git 库)
**优点**:
- 更高级的抽象
- 更友好的 API

**缺点**:
- 社区较小
- 维护不如 git2 活跃

### 决策: 选择 **git2 crate**

**理由**:
1. **类型安全**: Rust 类型系统提供更好的安全性
2. **错误处理**: 可以返回 anyhow::Result，与项目其他部分一致
3. **无外部依赖**: 不依赖系统 Git，更可靠
4. **性能**: 直接操作 Git 库，性能更好
5. **检测 Git**: 已有要求（FR-030），可以提前检查 Git 依赖

**架构设计**:
```rust
src/sync/
├── mod.rs
├── git.rs          # Git 操作封装（使用 git2）
├── crypto.rs       # 加密/解密 API Key
└── conflict.rs     # 冲突解决策略
```

**实现细节**:

1. **初始化 Git 仓库**:
```rust
use git2::{Repository, Signature, Time};

fn init_repo(config_dir: &Path) -> anyhow::Result<Repository> {
    let repo = Repository::init(config_dir)?;

    // 创建 .gitignore
    let gitignore_path = config_dir.join(".gitignore");
    fs::write(&gitignore_path, "*.key\nwizard_state.toml")?;

    // 创建初始提交
    let mut index = repo.index()?;
    index.add_all(["*"], git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let sig = Signature::now("AgentSwitch", "agentswitch@local")?;
    let parent_commit = None;
    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "Initial commit: Initialize AgentSwitch config",
        &tree,
        parent_commit.iter(),
    )?;

    Ok(repo)
}
```

2. **推送到远程**:
```rust
fn push_to_remote(repo: &Repository, remote_name: &str, branch: &str) -> anyhow::Result<()> {
    let refspec = format!("refs/heads/{}:refs/heads/{}", branch, branch);
    let mut remote = repo.find_remote(remote_name)?;

    // 推送
    remote.push(&[refspec.as_str()], None)?;

    Ok(())
}
```

3. **拉取并合并**:
```rust
fn pull_from_remote(repo: &Repository, remote_name: &str) -> anyhow::Result<()> {
    let mut remote = repo.find_remote(remote_name)?;
    remote.fetch(&[Some("main")], None, None)?;

    // 检查冲突
    let head = repo.head()?;
    let local_commit = repo.annotate_commit(head.target().unwrap())?;
    let remote_oid = repo.refname_to_id("refs/remotes/origin/main")?;
    let remote_commit = repo.annotate_commit(remote_oid)?;

    let analysis = repo.merge_analysis(&[remote_commit])?;

    if analysis.is_up_to_date() {
        return Ok(());
    } else if analysis.is_fast_forward() {
        // 快进合并
        let refname = "refs/heads/main";
        repo.reference_matching(
            &refname,
            remote_commit.id(),
            true,
            "fast-forward",
        )?;
        repo.checkout_tree(remote_commit.as_object(), None)?;
        repo.set_head(&refname)?;
    } else if analysis.is_normal() {
        // 需要合并
        return Err(anyhow!("需要手动合并冲突"));
    }

    Ok(())
}
```

**依赖添加**:
```toml
[dependencies]
git2 = "0.18"
```

---

## 4. API Key 加密存储方案

### 需求分析
- Git 同步时 API Key 必须加密
- 加密后仍可恢复（需要解密密钥）
- 透明加密（用户无感知）
- 支持多种加密策略

### 评估的选项

#### 选项 A: git-crypt (Git 加密工具)
**优点**:
- 专为 Git 设计
- 透明加密（特定文件自动加密）
- 支持 GPG 密钥
- 成熟稳定

**缺点**:
- 需要外部工具
- 配置相对复杂
- Windows 支持有限

#### 选项 B: AES-GCM 加密 (Rust 实现)
**优点**:
- 纯 Rust 实现
- 不依赖外部工具
- 灵活的密钥管理

**缺点**:
- 需要自己管理加密密钥
- 需要实现文件过滤逻辑

#### 选项 C: 环境变量 + .gitignore
**优点**:
- 简单直接
- 不泄露敏感信息

**缺点**:
- 不满足需求（用户希望同步配置，包括 API Key）

### 决策: 选择 **AES-GCM 加密 (Rust 实现) + 密钥派生**

**理由**:
1. **无外部依赖**: 不需要 git-crypt 等工具
2. **灵活性好**: 可以加密特定字段而非整个文件
3. **用户控制**: 用户可以选择是否加密、使用什么密钥
4. **跨平台**: 纯 Rust 实现，支持所有平台

**架构设计**:
```rust
src/crypto/
├── mod.rs
├── cipher.rs      # AES-GCM 加密实现
├── keyring.rs     # 密钥管理（使用系统密钥链）
└── filter.rs      # 配置文件加密/解密过滤
```

**实现细节**:

1. **加密 API Key**:
```rust
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{Argon2, PasswordHasher};

pub struct CryptoManager {
    cipher: Aes256Gcm,
}

impl CryptoManager {
    // 从用户密码派生密钥
    pub fn from_password(password: &str) -> anyhow::Result<Self> {
        // 使用 Argon2 派生密钥
        let salt = [0u8; 32];  // 实际应该是随机 salt
        let key = argon2(password, &salt)?;

        let cipher = Aes256Gcm::new(&key.into());
        Ok(Self { cipher })
    }

    // 加密 API Key
    pub fn encrypt_api_key(&self, api_key: &str) -> anyhow::Result<String> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = self.cipher.encrypt(&nonce, api_key.as_bytes())?;

        // 格式: nonce + ciphertext (base64)
        let mut result = nonce.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(base64::encode(result))
    }

    // 解密 API Key
    pub fn decrypt_api_key(&self, encrypted: &str) -> anyhow::Result<String> {
        let data = base64::decode(encrypted)?;
        let (nonce, ciphertext) = data.split_at(12);

        let nonce = Nonce::from_slice(nonce);
        let plaintext = self.cipher.decrypt(nonce, ciphertext)?;

        Ok(String::from_utf8(plaintext)?)
    }
}
```

2. **配置文件过滤**:
```toml
# models.toml (加密后)
[models.glm]
name = "glm"
base_url = "https://open.bigmodel.cn/api/v1"
api_key = "enc:AES256:base64data"  # 加密的 API Key
model_id = "glm-4"
```

3. **Git Hook 自动加密**:
```rust
// 在 commit 前自动加密敏感字段
fn pre_commit_hook(repo: &Repository) -> anyhow::Result<()> {
    let config_path = repo.workdir()?.unwrap().join("models.toml");
    let content = fs::read_to_string(&config_path)?;
    let encrypted = encrypt_sensitive_fields(&content)?;
    fs::write(&config_path, encrypted)?;
    Ok(())
}
```

**依赖添加**:
```toml
[dependencies]
aes-gcm = "0.10"
argon2 = "0.5"
base64 = "0.21"
```

**替代方案 - 使用 git-crypt** (如果用户已安装):
```bash
# 检测 git-crypt 是否可用
if command -v git-crypt &> /dev/null; then
    # 使用 git-crypt 加密 models.toml
    echo "models.toml filter=git-crypt diff=git-crypt" >> .gitattributes
    git-crypt enable
else
    # 使用内置加密
    asw sync init --encrypt-with-password
fi
```

---

## 5. 跨平台工具检测实现

### 需求分析
- 检测系统中已安装的 Code Agent 工具
- 跨平台支持（Linux/macOS/Windows）
- 检测工具版本
- 查找配置文件位置

### 现有实现
项目已有 `src/agents/` 模块，包含：
- `adapter.rs`: AgentAdapter trait 定义
- `registry.rs`: 工具注册表
- 各个工具的适配器实现

### 扩展方案

**架构设计**:
```rust
src/doctor/  # 新增模块
├── mod.rs
├── detector.rs    # 工具检测
├── health.rs      # 健康检查
└── reporter.rs    # 报告生成
```

**实现细节**:

1. **工具检测**:
```rust
use which::which;
use std::path::PathBuf;

pub struct ToolDetection {
    pub name: String,
    pub version: Option<String>,
    pub executable_path: Option<PathBuf>,
    pub config_path: Option<PathBuf>,
    pub status: ToolStatus,
}

pub enum ToolStatus {
    Installed { healthy: bool },
    NotInstalled,
    DetectionFailed(String),
}

pub fn detect_all_tools() -> Vec<ToolDetection> {
    let tools = [
        ("claude-code", detect_claude_code),
        ("codex", detect_codex),
        ("gemini-cli", detect_gemini_cli),
        ("qwen-cli", detect_qwen_cli),
        ("grok-cli", detect_grok_cli),
    ];

    tools.iter()
        .map(|(name, detector)| detector())
        .collect()
}

fn detect_claude_code() -> ToolDetection {
    match which("claude") {
        Ok(path) => {
            let version = get_version(&path, &["--version"]);
            let config_path = find_config_path(&[
                dirs::home_dir().unwrap().join(".claude/config.json"),
                dirs::config_dir().unwrap().join("claude/config.json"),
            ]);

            ToolDetection {
                name: "claude-code".to_string(),
                version,
                executable_path: Some(path),
                config_path,
                status: ToolStatus::Installed { healthy: true },
            }
        }
        Err(_) => ToolDetection {
            name: "claude-code".to_string(),
            version: None,
            executable_path: None,
            config_path: None,
            status: ToolStatus::NotInstalled,
        },
    }
}
```

2. **配置文件查找**:
```rust
fn find_config_path(candidates: &[PathBuf]) -> Option<PathBuf> {
    candidates.iter()
        .find(|path| path.exists())
        .cloned()
}
```

3. **健康检查**:
```rust
pub fn check_tool_health(adapter: &dyn AgentAdapter) -> HealthCheckResult {
    match adapter.config_path() {
        Ok(path) => {
            if !path.exists() {
                return HealthCheckResult {
                    status: HealthStatus::Error,
                    message: format!("配置文件不存在: {}", path.display()),
                    suggestion: "运行配置向导创建配置文件".to_string(),
                };
            }

            // 尝试解析配置文件
            match fs::read_to_string(&path) {
                Ok(content) => {
                    // 验证配置格式
                    match validate_config(&content) {
                        Ok(_) => HealthCheckResult {
                            status: HealthStatus::Healthy,
                            message: "配置正常".to_string(),
                            suggestion: String::new(),
                        },
                        Err(e) => HealthCheckResult {
                            status: HealthStatus::Error,
                            message: format!("配置格式错误: {}", e),
                            suggestion: "检查配置文件格式或重新配置".to_string(),
                        },
                    }
                }
                Err(e) => HealthCheckResult {
                    status: HealthStatus::Error,
                    message: format!("无法读取配置文件: {}", e),
                    suggestion: "检查文件权限".to_string(),
                },
            }
        }
        Err(e) => HealthCheckResult {
            status: HealthStatus::Error,
            message: format!("无法确定配置路径: {}", e),
            suggestion: "检查工具是否正确安装".to_string(),
        },
    }
}
```

4. **报告生成**:
```rust
use comfy_table::Table;
use colored::*;

pub fn generate_detection_report(detections: &[ToolDetection]) -> String {
    let mut table = Table::new();
    table.set_header(vec!["工具", "状态", "版本", "配置路径"]);

    for detection in detections {
        let status = match &detection.status {
            ToolStatus::Installed { healthy: true } => "✓ 已安装".green(),
            ToolStatus::Installed { healthy: false } => "⚠ 已安装（异常）".yellow(),
            ToolStatus::NotInstalled => "✗ 未安装".red(),
            ToolStatus::DetectionFailed(err) => format!("? 检测失败: {}", err).red(),
        };

        table.add_row(vec![
            detection.name.clone(),
            format!("{}", status),
            detection.version.as_ref().unwrap_or(&"-".to_string()),
            detection.config_path.as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "-".to_string()),
        ]);
    }

    table.to_string()
}
```

**依赖添加**:
```toml
[dependencies]
which = "6.0"  # 已有依赖，无需添加
```

---

## 6. 向导状态管理

### 需求分析
- 向导被中断时保存进度
- 下次启动时询问是否恢复
- 支持部分保存
- 临时文件权限 600

### 实现方案

**架构**:
```rust
src/wizard/
├── mod.rs
├── state.rs       # 向导状态管理
├── steps.rs       # 向导步骤定义
└── progress.rs    # 进度持久化
```

**实现细节**:

1. **状态结构**:
```rust
#[derive(Serialize, Deserialize)]
pub struct WizardState {
    pub current_step: usize,
    pub completed_steps: Vec<usize>,
    pub data: HashMap<String, String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl WizardState {
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;

        // 设置文件权限 600
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(path, perms)?;
        }

        Ok(())
    }

    pub fn load(path: &Path) -> anyhow::Result<Option<Self>> {
        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(path)?;
        let state: WizardState = toml::from_str(&content)?;
        Ok(Some(state))
    }
}
```

2. **恢复逻辑**:
```rust
use inquire::Confirm;

pub fn run_wizard_with_resume() -> anyhow::Result<ModelConfig> {
    let state_path = dirs::cache_dir()
        .unwrap()
        .join("agentswitch")
        .join("wizard_state.toml");

    // 检查是否有未完成的向导
    if let Some(state) = WizardState::load(&state_path)? {
        let resume = Confirm::new(&format!(
            "检测到未完成的配置（{}），是否继续？",
            format_timestamp(state.timestamp)
        ))
        .with_default(true)
        .prompt()?;

        if resume {
            return resume_wizard(state, &state_path);
        } else {
            // 删除旧状态
            fs::remove_file(&state_path)?;
        }
    }

    // 运行新向导
    run_new_wizard(&state_path)
}
```

---

## 总结

### 技术选型汇总

| 功能 | 技术选择 | 版本 | 理由 |
|------|----------|------|------|
| 交互式向导 | inquire | 0.7 | 现代 API、内置验证、TTY 检测 |
| Shell 补全 | clap_complete + 自定义 | 4.5 | 自动同步、维护成本低 |
| Git 集成 | git2 | 0.18 | 类型安全、无外部依赖、性能好 |
| API Key 加密 | AES-GCM + Argon2 | 0.10, 0.5 | 无外部依赖、灵活、跨平台 |
| 工具检测 | 扩展现有 agents 模块 | - | 复用现有架构 |
| 向导状态管理 | TOML + 文件权限 | - | 简单可靠 |

### 新增依赖

```toml
[dependencies]
# 交互式输入
inquire = "0.7"

# Shell 补全
clap_complete = "4.5"

# Git 操作
git2 = "0.18"

# 加密
aes-gcm = "0.10"
argon2 = "0.5"
base64 = "0.21"
```

### 架构概览

```
src/
├── wizard/           # 交互式向导（新增）
│   ├── mod.rs
│   ├── state.rs
│   ├── steps.rs
│   └── progress.rs
├── doctor/           # 工具检测和健康检查（新增）
│   ├── mod.rs
│   ├── detector.rs
│   ├── health.rs
│   └── reporter.rs
├── completion/       # Shell 补全（新增）
│   ├── mod.rs
│   ├── static.rs
│   ├── dynamic.rs
│   └── install.rs
├── sync/             # Git 同步（新增）
│   ├── mod.rs
│   ├── git.rs
│   ├── crypto.rs
│   └── conflict.rs
├── cli/              # 扩展现有 CLI
│   └── commands.rs   # 添加新命令
└── agents/           # 现有模块（无需修改）
```

### 关键设计决策

1. **模块化设计**: 每个功能独立模块，降低耦合
2. **复用现有架构**: 基于现有的 agents、config、presets 模块
3. **渐进式实现**: 可以按优先级逐步实现（P1 → P2）
4. **安全性优先**: API Key 加密、文件权限控制
5. **用户体验**: 友好的错误提示、进度保存、TTY 检测

### 风险和缓解

| 风险 | 缓解措施 |
|------|----------|
| inquire API 可能变更 | 选择稳定版本 (0.7)，关注更新 |
| Git 操作复杂 | 使用 git2 简化常见操作，提供详细错误信息 |
| 加密密钥管理 | 支持多种密钥来源（密码、系统密钥链） |
| Shell 配置文件位置不统一 | 检测多个常见位置，提供手动安装选项 |
| 工具检测误报 | 使用多种检测方法，提供手动刷新 |

### 下一步

Phase 1 将基于本研究生成：
1. `data-model.md` - 数据模型设计
2. `contracts/CLI.md` - CLI 命令接口定义
3. `quickstart.md` - 快速开始指南

---

**研究完成日期**: 2026-03-10
**研究状态**: ✅ 所有关键技术问题已解决
