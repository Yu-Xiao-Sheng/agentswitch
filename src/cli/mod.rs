//! CLI 命令行界面模块

use clap::{Parser, Subcommand};

pub mod commands;

pub use commands::ModelCommands;

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

    /// Show current configuration status
    Status {
        #[arg(long, short)]
        detailed: bool,
    },
}

impl Command {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Command::Model(cmd) => cmd.run(),
            Command::Status { detailed } => {
                if *detailed {
                    println!("详细状态信息...");
                } else {
                    println!("当前状态信息...");
                }
                Ok(())
            }
        }
    }
}
