use agentswitch::agents::codex::CodexAdapter;
use agentswitch::agents::AgentAdapter;
use std::fs;

#[test]
fn test_codex_adapter_name() {
    let adapter = CodexAdapter::new();
    assert_eq!(adapter.name(), "codex");
}

#[test]
fn test_codex_config_path() {
    let adapter = CodexAdapter::new();
    let config_path = adapter.config_path();
    assert!(config_path.is_ok());
    let path = config_path.unwrap();
    assert!(path.ends_with(".codex/config.toml"));
}

#[test]
fn test_codex_config_dir() {
    let adapter = CodexAdapter::new();
    let config_dir = adapter.config_dir();
    assert!(config_dir.is_ok());
    let path = config_dir.unwrap();
    assert!(path.ends_with(".codex"));
}
