use agentswitch::agents::claude_code::ClaudeCodeAdapter;
use agentswitch::agents::AgentAdapter;

#[test]
fn test_adapter_name() {
    let adapter = ClaudeCodeAdapter::new();
    assert_eq!(adapter.name(), "claude-code");
}

#[test]
fn test_config_path() {
    let adapter = ClaudeCodeAdapter::new();
    let path = adapter.config_path().unwrap();
    assert!(path.ends_with(".claude/settings.json"));
}

#[test]
fn test_current_model_none() {
    let adapter = ClaudeCodeAdapter::new();
    let result = adapter.current_model();
    assert!(result.is_ok());
}
