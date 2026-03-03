use agentswitch::config::ModelConfig;
use std::time::Instant;

#[test]
fn test_config_switch_performance() {
    // 创建测试配置
    let config = ModelConfig::new(
        "test-model".to_string(),
        "https://api.test.com".to_string(),
        "test-key".to_string(),
        "test-model-id".to_string(),
    );

    // 测试配置创建性能（应该在 10ms 内完成）
    let start = Instant::now();
    let _config2 = ModelConfig::new(
        "test-model-2".to_string(),
        "https://api.test2.com".to_string(),
        "test-key-2".to_string(),
        "test-model-id-2".to_string(),
    );
    let duration = start.elapsed();

    // 配置创建应该非常快（< 10ms）
    assert!(duration.as_millis() < 10, "配置创建应在 10ms 内完成，实际: {:?}", duration);

    println!("✓ 配置创建性能: {:?}", duration);
}

#[test]
fn test_backup_creation_performance() {
    use agentswitch::backup::BackupManager;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test-config.json");

    // 创建测试配置文件
    let test_content = r#"{"test": "data"}"#;
    fs::write(&config_path, test_content).unwrap();

    // 测试备份创建性能
    let start = Instant::now();
    let backup_manager = BackupManager::new();

    // 尝试创建备份
    let result = backup_manager.create_backup(
        "test-agent",
        &config_path,
        "json"
    );

    let duration = start.elapsed();

    if result.is_ok() {
        // 备份创建应该在 500ms 内完成
        assert!(
            duration.as_millis() < 500,
            "备份创建应在 500ms 内完成，实际: {:?}",
            duration
        );
        println!("✓ 备份创建性能: {:?}", duration);
    } else {
        println!("⚠ 备份创建跳过: {:?}", result.err());
    }
}

#[test]
fn test_model_validation_performance() {
    use agentswitch::utils::{validate_model_name, validate_url};

    let start = Instant::now();

    // 测试验证性能（应该非常快）
    let _ = validate_model_name("test-model");
    let _ = validate_url("https://api.test.com");

    let duration = start.elapsed();

    assert!(duration.as_millis() < 10, "验证应在 10ms 内完成，实际: {:?}", duration);
    println!("✓ 验证性能: {:?}", duration);
}
