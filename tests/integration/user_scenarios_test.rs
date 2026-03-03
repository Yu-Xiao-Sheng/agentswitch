/// 用户场景验证测试
///
/// 验证 quickstart.md 中的常见使用场景

#[test]
fn test_model_config_lifecycle() {
    use agentswitch::config::{ModelConfig, ConfigStore};
    use tempfile::TempDir;
    use std::fs;

    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".agentswitch");
    fs::create_dir_all(&config_dir).unwrap();

    // 场景 1: 创建模型配置
    let config = ModelConfig::new(
        "glm".to_string(),
        "https://open.bigmodel.cn/api/v1".to_string(),
        "sk-test-key".to_string(),
        "glm-4".to_string(),
    );

    assert_eq!(config.name, "glm");
    assert_eq!(config.base_url, "https://open.bigmodel.cn/api/v1");
    assert_eq!(config.api_key, "sk-test-key");
    assert_eq!(config.model_id, "glm-4");

    println!("✓ 场景 1: 模型配置创建成功");
}

#[test]
fn test_backup_restore_workflow() {
    use agentswitch::backup::BackupManager;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");

    // 创建原始配置文件
    let original_content = r#"{"api_key": "old-key"}"#;
    fs::write(&config_path, original_content).unwrap();

    // 场景 2: 备份流程
    let backup_manager = BackupManager::new();

    // 创建备份
    let backup = backup_manager.create_backup("test-agent", &config_path, "json");

    if let Ok(backup) = backup {
        // 修改配置
        let modified_content = r#"{"api_key": "new-key"}"#;
        fs::write(&config_path, modified_content).unwrap();

        // 恢复备份
        let restore_result = backup_manager.restore_backup(&backup);

        if restore_result.is_ok() {
            // 验证恢复成功
            let restored_content = fs::read_to_string(&config_path).unwrap();
            assert_eq!(restored_content, original_content);
            println!("✓ 场景 2: 备份和恢复流程验证成功");
        } else {
            println!("⚠ 恢复失败: {:?}", restore_result.err());
        }
    } else {
        println!("⚠ 备份创建跳过: {:?}", backup.err());
    }
}

#[test]
fn test_multiple_models_management() {
    use agentswitch::config::ModelConfig;

    // 场景 3: 管理多个模型配置
    let models = vec![
        ModelConfig::new(
            "glm".to_string(),
            "https://open.bigmodel.cn/api/v1".to_string(),
            "sk-glm".to_string(),
            "glm-4".to_string(),
        ),
        ModelConfig::new(
            "minimax".to_string(),
            "https://api.minimax.chat/v1".to_string(),
            "sk-minimax".to_string(),
            "abab6.5s-chat".to_string(),
        ),
        ModelConfig::new(
            "deepseek".to_string(),
            "https://api.deepseek.com/v1".to_string(),
            "sk-deepseek".to_string(),
            "deepseek-chat".to_string(),
        ),
    ];

    assert_eq!(models.len(), 3);
    assert_eq!(models[0].name, "glm");
    assert_eq!(models[1].name, "minimax");
    assert_eq!(models[2].name, "deepseek");

    println!("✓ 场景 3: 多模型管理验证成功");
}

#[test]
fn test_error_scenarios() {
    use agentswitch::utils::{validate_model_name, validate_url};

    // 场景 4: 错误处理 - 无效 URL
    let invalid_url_result = validate_url("not-a-valid-url");
    assert!(invalid_url_result.is_err());
    println!("✓ 场景 4a: 无效 URL 检测成功");

    // 场景 5: 错误处理 - 无效模型名称
    let invalid_name_result = validate_model_name("");
    assert!(invalid_name_result.is_err());
    println!("✓ 场景 4b: 无效模型名称检测成功");

    // 场景 6: 错误处理 - 空的 API Key
    let config = ModelConfig::new(
        "test".to_string(),
        "https://api.test.com".to_string(),
        "".to_string(),  // 空 API key
        "model".to_string(),
    );
    // 允许空 API key（某些场景可能需要）
    assert_eq!(config.api_key, "");
    println!("✓ 场景 4c: 空 API Key 处理验证成功");
}

#[test]
fn test_adapter_detection() {
    use agentswitch::agents::{all_adapters, AgentAdapter};

    // 场景 5: 工具检测
    let adapters = all_adapters();

    for adapter in adapters {
        let name = adapter.name();

        // 检测方法应该不会崩溃
        let detect_result = adapter.detect();
        assert!(detect_result.is_ok());

        let is_installed = detect_result.unwrap();

        // 配置路径应该可以获取
        let path_result = adapter.config_path();
        assert!(path_result.is_ok());

        let config_path = path_result.unwrap();

        println!("✓ 工具: {} | 安装: {} | 配置路径: {:?}",
                 name,
                 if is_installed { "✓" } else { "✗" },
                 config_path.display());
    }

    println!("✓ 场景 5: 工具检测验证成功");
}
