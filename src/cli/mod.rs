pub mod commands;
pub mod args;

use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[command(name = "asw")]
#[command(about = "A universal model configuration switcher for code agent CLI tools", long_about = None)]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn run(&self) -> anyhow::Result<()> {
        self.command.run()
    }
}
