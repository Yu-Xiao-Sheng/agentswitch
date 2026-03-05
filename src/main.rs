use anyhow::Result;
use clap::Parser;

mod agents;
mod backup;
mod batch;
mod cli;
mod config;
mod error;
mod io;
mod output;
mod presets;
mod utils;

use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Err(e) = cli.command.run() {
        eprintln!("✗ 错误: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
