//! AgentSwitch - 代码终端代理工具配置切换器
//!
//! 支持将任意 OpenAI 协议模型接入到主流 Code Agent 工具中

// 允许未使用的代码（为将来功能准备的代码）
#![allow(dead_code)]

use anyhow::Result;
use clap::Parser;

mod agents;
mod backup;
mod batch;
mod cli;
mod completion;
mod config;
mod crypto;
mod doctor;
mod error;
mod io;
mod output;
mod presets;
mod sync;
mod utils;
mod wizard;

use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Err(e) = cli.command.run() {
        eprintln!("✗ 错误: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
