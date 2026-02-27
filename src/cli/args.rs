// Command arguments and options
use clap::Args;

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
