use agentswitch::config::ModelConfig;

#[test]
fn test_model_config_creation() {
    let config = ModelConfig::new(
        "test-model".to_string(),
        "https://api.test.com".to_string(),
        "test-key".to_string(),
        "test-model-id".to_string(),
    );

    assert_eq!(config.name, "test-model");
    assert_eq!(config.base_url, "https://api.test.com");
    assert_eq!(config.api_key, "test-key");
    assert_eq!(config.model_id, "test-model-id");
}
