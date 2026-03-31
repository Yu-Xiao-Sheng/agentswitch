// Command arguments and options
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct StatusArgs {
    /// Show detailed status
    #[arg(long, short)]
    pub detailed: bool,
}

impl StatusArgs {
    pub fn run(&self) -> anyhow::Result<()> {
        println!("Showing current status...");
        // TODO: Implement status display
        Ok(())
    }
}

/// Preset management commands
#[derive(Subcommand, Debug)]
pub enum PresetCommands {
    /// Create a new preset
    Create {
        /// Preset name
        name: String,
        /// Preset description
        #[arg(long)]
        description: Option<String>,
        /// Preset tags
        #[arg(long, value_parser = parse_kv)]
        tag: Vec<String>,
        /// Agent to model mappings (e.g., claude-code:glm)
        #[arg(long, value_parser = parse_kv)]
        agent: Vec<String>,
    },

    /// List all presets
    List {
        /// Filter by tag
        #[arg(long)]
        tag: Vec<String>,
        /// Output format
        #[arg(long, default_value = "table")]
        format: String,
    },

    /// Show preset details
    Show {
        /// Preset name
        name: String,
    },

    /// Apply a preset
    Apply {
        /// Preset name
        name: String,
        /// Apply only to specific agents
        #[arg(long)]
        agent: Vec<String>,
        /// Simulate run without applying
        #[arg(long)]
        dry_run: bool,
        /// Skip backup
        #[arg(long)]
        no_backup: bool,
    },

    /// Update a preset
    Update {
        /// Preset name
        name: String,
        /// Update description
        #[arg(long)]
        description: Option<String>,
        /// Add tags
        #[arg(long)]
        tag: Vec<String>,
        /// Update agent mappings
        #[arg(long, value_parser = parse_kv)]
        agent: Vec<String>,
    },

    /// Delete a preset
    Delete {
        /// Preset name
        name: String,
        /// Skip confirmation
        #[arg(long)]
        force: bool,
    },

    /// Validate a preset
    Validate {
        /// Preset name
        name: String,
    },

    /// Import presets
    Import {
        /// Input file path
        input: String,
        /// Import strategy (merge or overwrite)
        #[arg(long, default_value = "merge")]
        strategy: String,
        /// Simulate import without applying
        #[arg(long)]
        dry_run: bool,
    },

    /// Export presets
    Export {
        /// Output file path
        output: String,
        /// Export specific preset
        #[arg(long)]
        preset: Vec<String>,
        /// Include model configs (sanitized)
        #[arg(long)]
        include_models: bool,
        /// Include active configuration
        #[arg(long)]
        include_active: bool,
    },
}

/// Batch operation commands
#[derive(Subcommand, Debug)]
pub enum BatchCommands {
    /// Switch all agents to a model
    Switch {
        /// Target model name
        model: String,
        /// Switch only specific agents
        #[arg(long)]
        agent: Vec<String>,
        /// Number of parallel tasks
        #[arg(long, default_value = "0")]
        parallel: usize,
        /// Simulate run
        #[arg(long)]
        dry_run: bool,
    },

    /// Validate all agents
    Validate {
        /// Validate only specific agents
        #[arg(long)]
        agent: Vec<String>,
    },

    /// Show batch status
    Status {
        /// Output format
        #[arg(long, default_value = "table")]
        format: String,
    },
}

fn parse_kv(s: &str) -> Result<String, String> {
    // 只分割第一个冒号，允许值部分包含冒号（如 provider:model 格式）
    if let Some(pos) = s.find(':') {
        let key = &s[..pos];
        let value = &s[pos + 1..];
        if !key.is_empty() && !value.is_empty() {
            Ok(s.to_string())
        } else {
            Err(format!("Invalid key:value format: {} (key and value must be non-empty)", s))
        }
    } else {
        Err(format!("Invalid key:value format: {} (expected 'key:value')", s))
    }
}

// ============ Spec 004 新增命令类型 ============

/// 向导命令（Spec 004）
#[derive(Subcommand, Debug)]
pub enum WizardCommands {
    /// 初始化配置（向导）
    Init {
        /// 恢复之前的向导进度
        #[arg(long)]
        resume: bool,

        /// 重新开始（清除进度）
        #[arg(long)]
        reset: bool,

        /// 尝试非交互式模式（实验性）
        #[arg(long)]
        non_interactive: bool,
    },

    /// 添加模型配置（向导）
    Wizard {
        /// 预设模型名称（跳过输入）
        #[arg(long)]
        name: Option<String>,
    },
}

/// 诊断命令（Spec 004）
#[derive(Subcommand, Debug)]
pub enum DoctorCommands {
    /// 检测已安装工具（简化版）
    Detect,
}

/// 补全命令（Spec 004）
#[derive(Subcommand, Debug)]
pub enum CompletionCommands {
    /// 安装补全脚本
    Install {
        /// Shell 类型（bash/zsh/fish）
        shell: String,

        /// 自定义安装路径
        #[arg(long)]
        path: Option<String>,

        /// 不修改 Shell 配置文件
        #[arg(long)]
        no_modify_config: bool,
    },

    /// 卸载补全脚本
    Uninstall {
        /// Shell 类型（bash/zsh/fish）
        shell: String,

        /// 自定义安装路径
        #[arg(long)]
        path: Option<String>,
    },

    /// 生成补全脚本到标准输出
    Generate {
        /// Shell 类型（bash/zsh/fish）
        shell: String,
    },
}

/// 同步命令（Spec 004）
#[derive(Subcommand, Debug)]
pub enum SyncCommands {
    /// 初始化 Git 仓库
    Init {
        /// 启用加密
        #[arg(long)]
        encrypt: bool,

        /// 加密方法（aes-gcm/git-crypt）
        #[arg(long)]
        encryption_method: Option<String>,

        /// 禁用加密
        #[arg(long)]
        no_encrypt: bool,
    },

    /// 管理远程仓库
    Remote {
        #[command(subcommand)]
        command: RemoteSubCommands,
    },

    /// 推送到远程
    Push {
        /// 远程仓库名称
        #[arg(long)]
        remote: Option<String>,

        /// 分支名称
        #[arg(long)]
        branch: Option<String>,

        /// 跳过加密（不推荐）
        #[arg(long)]
        no_encrypt: bool,
    },

    /// 从远程拉取
    Pull {
        /// 远程仓库名称
        #[arg(long)]
        remote: Option<String>,

        /// 分支名称
        #[arg(long)]
        branch: Option<String>,

        /// 合并策略
        #[arg(long)]
        strategy: Option<String>,
    },

    /// 显示同步状态
    Status,
}

/// 远程仓库子命令（Spec 004）
#[derive(Subcommand, Debug)]
pub enum RemoteSubCommands {
    /// 添加远程仓库
    Add {
        /// 远程仓库 URL
        url: String,
    },

    /// 删除远程仓库
    Remove {
        /// 远仓库名称
        name: String,
    },

    /// 列出远程仓库
    List,

    /// 修改远程仓库 URL
    SetUrl {
        /// 远仓库名称
        name: String,
        /// 新的 URL
        url: String,
    },
}

// Spec 004 命令的 run() 方法实现

impl WizardCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        // 由 commands.rs 中的具体实现处理
        Ok(())
    }
}

impl DoctorCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        // 由 commands.rs 中的具体实现处理
        Ok(())
    }
}

impl CompletionCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        // 由 commands.rs 中的具体实现处理
        Ok(())
    }
}

impl SyncCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        // 由 commands.rs 中的具体实现处理
        Ok(())
    }
}

impl RemoteSubCommands {
    pub fn run(&self) -> anyhow::Result<()> {
        // 由 commands.rs 中的具体实现处理
        Ok(())
    }
}
