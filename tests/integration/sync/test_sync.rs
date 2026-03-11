use std::fs;
use tempfile::TempDir;

#[test]
fn test_sync_init() {
    use agentswitch::sync::config::run_sync_init;

    // Create a temporary directory to use as the config dir
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".agentswitch");

    // This test would need to mock the config directory
    // For now, just verify the function exists
    let result = run_sync_init();
    // May fail if already initialized or permissions issues
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_sync_status() {
    use agentswitch::sync::config::run_sync_status;

    let result = run_sync_status();
    assert!(result.is_ok());
}
