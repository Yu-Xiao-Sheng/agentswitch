pub mod adapter;
pub mod claude_code;
pub mod codex;
pub mod gemini;
pub mod qwen;
pub mod grok;
pub mod registry;

pub use adapter::{
    AgentAdapter,
    Backup,
};

pub use registry::{
    AdapterRegistry,
    AdapterInfo,
    ValidationResult,
    global_registry,
};

/// Get all available agent adapters
pub fn all_adapters() -> Vec<Box<dyn AgentAdapter>> {
    vec![
        Box::new(claude_code::ClaudeCodeAdapter::new()),
        Box::new(codex::CodexAdapter::new()),
        Box::new(gemini::GeminiAdapter::new()),
        Box::new(qwen::QwenAdapter::new()),
        Box::new(grok::GrokAdapter::new()),
    ]
}

/// Get an adapter by name
pub fn get_adapter(name: &str) -> Option<Box<dyn AgentAdapter>> {
    all_adapters()
        .into_iter()
        .find(|adapter| adapter.name() == name)
}
