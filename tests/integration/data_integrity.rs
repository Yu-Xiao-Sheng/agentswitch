//! 数据往返一致性测试

#[cfg(test)]
mod tests {
    use agentswitch::config::models::{ModelConfig, AppConfig};
    use serde_json;
    use std::collections::HashMap;
    
    #[test]
    fn test_round_trip_consistency() {
        let mut original = AppConfig::new();
        let model = ModelConfig::new(
            "glm".to_string(),
            "https://open.bigmodel.cn/api/v1".to_string(),
            "sk-abc123".to_string(),
            "glm-4".to_string(),
        );
        original.add_model(model).unwrap();
        
        let toml_str = toml::to_string_pretty(&original).unwrap();
        let restored: AppConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(original, restored);
    }
    
    #[test]
    fn test_extra_params_round_trip() {
        let mut extra = HashMap::new();
        extra.insert("temperature".to_string(), serde_json::Value::Number(7.into()));
        
        let model = ModelConfig::new(
            "test".to_string(),
            "https://api.test.com".to_string(),
            "key".to_string(),
            "model".to_string(),
        ).with_extra_params(extra);
        
        let toml_str = toml::to_string_pretty(&model).unwrap();
        let restored: ModelConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(model, restored);
    }
}
