//! API Key 脱敏测试

use crate::io::sanitizer::sanitize_api_key;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_empty_api_key() {
        assert_eq!(sanitize_api_key(""), "");
    }

    #[test]
    fn test_sanitize_short_api_key() {
        let result = sanitize_api_key("sk-1234");
        assert!(result.contains("REDACTED"));
    }

    #[test]
    fn test_sanitize_long_api_key() {
        let result = sanitize_api_key("sk-1234567890abcdefghijklmnop");
        assert!(result.contains("REDACTED"));
        assert!(result.starts_with("sk-1"));
    }
}
