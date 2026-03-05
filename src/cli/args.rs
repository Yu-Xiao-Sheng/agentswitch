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
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() == 2 {
        // 返回原始字符串，后续在命令处理中解析
        Ok(s.to_string())
    } else {
        Err(format!("Invalid key:value format: {}", s))
    }
}
