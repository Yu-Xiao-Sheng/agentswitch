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
