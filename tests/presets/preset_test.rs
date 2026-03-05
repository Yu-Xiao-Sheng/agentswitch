//! 预设单元测试

use crate::presets::preset::{Preset, is_valid_preset_name, is_valid_version};
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_preset_name() {
        assert!(is_valid_preset_name("development"));
        assert!(is_valid_preset_name("prod"));
        assert!(is_valid_preset_name("test-123"));
        assert!(is_valid_preset_name("my_preset"));
    }

    #[test]
    fn test_invalid_preset_name() {
        assert!(!is_valid_preset_name("")); // 空
        assert!(!is_valid_preset_name("a")); // 太短
        assert!(!is_valid_preset_name(&"x".repeat(65))); // 太长
        assert!(!is_valid_preset_name("invalid name")); // 包含空格
        assert!(!is_valid_preset_name("invalid/name")); // 包含斜杠
    }

    #[test]
    fn test_valid_version() {
        assert!(is_valid_version("1.0.0"));
        assert!(is_valid_version("2.1.3"));
        assert!(is_valid_version("0.1.0-beta"));
    }

    #[test]
    fn test_invalid_version() {
        assert!(!is_valid_version("1.0"));
        assert!(!is_valid_version("1"));
        assert!(!is_valid_version("a.b.c"));
    }

    #[test]
    fn test_preset_validation_success() {
        let mut mappings = HashMap::new();
        mappings.insert("claude-code".to_string(), "glm".to_string());

        let preset = Preset::new(
            "test".to_string(),
            "Test preset".to_string(),
            mappings,
        );

        let mut models = HashSet::new();
        models.insert("glm".to_string());

        assert!(preset.validate(&models).is_ok());
    }

    #[test]
    fn test_preset_validation_empty_mappings() {
        let preset = Preset::new(
            "test".to_string(),
            "Test preset".to_string(),
            HashMap::new(),
        );

        let models = HashSet::new();
        assert!(preset.validate(&models).is_err());
    }

    #[test]
    fn test_preset_validation_model_not_found() {
        let mut mappings = HashMap::new();
        mappings.insert("claude-code".to_string(), "unknown-model".to_string());

        let preset = Preset::new(
            "test".to_string(),
            "Test preset".to_string(),
            mappings,
        );

        let models = HashSet::new();
        assert!(preset.validate(&models).is_err());
    }
}
