//! CLI 命令行界面模块

use clap::{Parser, Subcommand};

pub mod commands;

pub use commands::{ModelCommands, AgentCommands, BackupCommands};

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
    /// Model configuration management
    #[command(subcommand)]
    Model(ModelCommands),

    /// Agent tool management
    #[command(subcommand)]
    Agent(AgentCommands),

    /// Backup management
    #[command(subcommand)]
    Backup(BackupCommands),

    /// Show current configuration status
    Status {
        #[arg(long, short)]
        detailed: bool,
    },

    /// Switch agent tool to a different model
    Switch {
        /// Agent name (e.g., claude-code, codex, gemini-cli)
        agent: String,
        /// Model configuration name
        model: String,
    },
}

impl Command {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Command::Model(cmd) => cmd.run(),
            Command::Agent(cmd) => cmd.run(),
            Command::Backup(cmd) => cmd.run(),
            Command::Status { detailed: _ } => {
                commands::execute_show_status()
            }
            Command::Switch { agent, model } => {
                commands::execute_switch(agent, model)
            }
        }
    }
}
