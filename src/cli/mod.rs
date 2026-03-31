//! CLI 命令行界面模块

use clap::{Parser, Subcommand};

pub mod args;
pub mod commands;

pub use args::{BatchCommands, PresetCommands};
pub use commands::{
    AgentCommands, BackupCommands, CryptoCommands, ProviderCommands, UpdateCommands,
};

// Spec 004 新增命令导出 (从 commands.rs 导入，因为那里有 run 实现)
pub use commands::{CompletionCommands, SyncCommands, WizardCommands};

/// AgentSwitch CLI
#[derive(Parser, Debug)]
#[command(name = "asw")]
#[command(about = "代码终端代理工具配置切换器", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Provider configuration management
    #[command(subcommand)]
    Provider(ProviderCommands),

    /// Agent tool management
    #[command(subcommand)]
    Agent(AgentCommands),

    /// Backup management
    #[command(subcommand)]
    Backup(BackupCommands),

    /// Preset management
    #[command(subcommand)]
    Preset(PresetCommands),

    /// Batch operations
    #[command(subcommand)]
    Batch(BatchCommands),

    /// Show current configuration status
    Status {
        #[arg(long, short)]
        detailed: bool,
    },

    /// Switch agent tool to a different provider/model
    Switch {
        /// Agent name (e.g., claude-code, codex, gemini-cli)
        agent: String,
        /// Provider name
        provider: String,
        /// Model name
        model: String,
    },

    // ============ Spec 004 新增命令 ============
    /// Interactive configuration wizard (Spec 004)
    #[command(subcommand)]
    Wizard(WizardCommands),

    /// 运行诊断工具检查环境和配置
    Doctor {
        /// 显示详细信息
        #[arg(short, long)]
        verbose: bool,

        /// 以 JSON 格式输出
        #[arg(short, long)]
        json: bool,

        /// 尝试自动修复问题
        #[arg(long)]
        fix: bool,

        /// 子命令
        #[command(subcommand)]
        command: Option<DoctorSubcommand>,
    },

    /// Shell completion (Spec 004)
    #[command(subcommand)]
    Completion(CompletionCommands),

    /// Git sync (Spec 004)
    #[command(subcommand)]
    Sync(SyncCommands),

    /// Crypto key management
    #[command(subcommand)]
    Crypto(CryptoCommands),

    /// 检查更新
    #[command(subcommand)]
    Update(UpdateCommands),
}

/// Doctor 子命令
#[derive(Subcommand, Debug)]
pub enum DoctorSubcommand {
    /// 检测已安装工具（简化版）
    Detect,
}

impl Command {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Command::Provider(cmd) => cmd.run(),
            Command::Agent(cmd) => cmd.run(),
            Command::Backup(cmd) => cmd.run(),
            Command::Preset(cmd) => cmd.run(),
            Command::Batch(cmd) => cmd.run(),
            Command::Status { detailed } => commands::execute_show_status(*detailed),
            Command::Switch {
                agent,
                provider,
                model,
            } => commands::execute_switch(agent, provider, model),
            // Spec 004 新命令
            Command::Wizard(cmd) => cmd.run(),
            Command::Doctor {
                verbose,
                json,
                fix,
                command,
            } => match command {
                Some(DoctorSubcommand::Detect) => crate::doctor::run_detect(),
                None => crate::doctor::run_doctor(*verbose, *json, *fix),
            },
            Command::Completion(cmd) => cmd.run(),
            Command::Sync(cmd) => cmd.run(),
            Command::Crypto(cmd) => cmd.run(),
            // 更新检查命令
            Command::Update(cmd) => cmd.run(),
        }
    }
}

impl Cli {
    /// 获取 clap Command 实例（用于生成补全脚本）
    pub fn command() -> clap::Command {
        <Self as clap::CommandFactory>::command()
    }
}
