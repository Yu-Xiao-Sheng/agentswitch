use agentswitch::agents::gemini::GeminiAdapter;
use agentswitch::agents::AgentAdapter;

#[test]
fn test_gemini_adapter_name() {
    let adapter = GeminiAdapter::new();
    assert_eq!(adapter.name(), "gemini-cli");
}

#[test]
fn test_gemini_config_path() {
    let adapter = GeminiAdapter::new();
    let config_path = adapter.config_path();
    assert!(config_path.is_ok());
    let path = config_path.unwrap();
    assert!(path.ends_with(".gemini/settings.json"));
}

#[test]
fn test_gemini_config_dir() {
    let adapter = GeminiAdapter::new();
    let config_dir = adapter.config_dir();
    assert!(config_dir.is_ok());
    let path = config_dir.unwrap();
    assert!(path.ends_with(".gemini"));
}
