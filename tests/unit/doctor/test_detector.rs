use agentswitch::doctor::{ToolDetection, ToolStatus, ConfigFormat};

#[test]
fn test_tool_detection_creation() {
    let detection = ToolDetection {
        name: "test-tool".to_string(),
        display_name: "Test Tool".to_string(),
        status: ToolStatus::Installed { healthy: true },
        version: Some("1.0.0".to_string()),
        executable_path: None,
        config_path: None,
        config_format: None,
    };

    assert_eq!(detection.name, "test-tool");
    assert!(matches!(detection.status, ToolStatus::Installed { healthy: true }));
}

#[test]
fn test_tool_status_not_installed() {
    let status = ToolStatus::NotInstalled;
    assert!(matches!(status, ToolStatus::NotInstalled));
}

#[test]
fn test_config_format_detection() {
    // Test that ConfigFormat variants exist
    let _ = ConfigFormat::Json;
    let _ = ConfigFormat::Toml;
    let _ = ConfigFormat::Yaml;
}
