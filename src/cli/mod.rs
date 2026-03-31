//! CLI 命令行界面模块

use clap::{Parser, Subcommand};

pub mod args;
pub mod commands;

pub use args::{BatchCommands, PresetCommands};
pub use commands::{AgentCommands, BackupCommands, CryptoCommands, ProviderCommands};

// Spec 004 新增命令导出 (从 commands.rs 导入，因为那里有 run 实现)
pub use commands::{CompletionCommands, DoctorCommands, SyncCommands, WizardCommands};

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

    /// Tool diagnostics (Spec 004)
    #[command(subcommand)]
    Doctor(DoctorCommands),

    /// Shell completion (Spec 004)
    #[command(subcommand)]
    Completion(CompletionCommands),

    /// Git sync (Spec 004)
    #[command(subcommand)]
    Sync(SyncCommands),

    /// Crypto key management
    #[command(subcommand)]
    Crypto(CryptoCommands),
}

impl Command {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Command::Provider(cmd) => cmd.run(),
            Command::Agent(cmd) => cmd.run(),
            Command::Backup(cmd) => cmd.run(),
            Command::Preset(cmd) => cmd.run(),
            Command::Batch(cmd) => cmd.run(),
            Command::Status { detailed: _ } => commands::execute_show_status(),
            Command::Switch { agent, provider, model } => commands::execute_switch(agent, provider, model),
            // Spec 004 新命令
            Command::Wizard(cmd) => cmd.run(),
            Command::Doctor(cmd) => cmd.run(),
            Command::Completion(cmd) => cmd.run(),
            Command::Sync(cmd) => cmd.run(),
            Command::Crypto(cmd) => cmd.run(),
        }
    }
}

impl Cli {
    /// 获取 clap Command 实例（用于生成补全脚本）
    pub fn command() -> clap::Command {
        <Self as clap::CommandFactory>::command()
    }
}
