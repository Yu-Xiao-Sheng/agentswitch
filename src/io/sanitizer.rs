//! API Key 脱敏模块

/// 脱敏 API Key
pub fn sanitize_api_key(api_key: &str) -> String {
    if api_key.is_empty() {
        return String::new();
    }

    // 如果已经脱敏，直接返回
    if api_key.contains("REDACTED") {
        return api_key.to_string();
    }

    // 脱敏处理：只显示前 4 个和后 4 个字符
    if api_key.len() <= 8 {
        "***REDACTED***".to_string()
    } else {
        format!("{}...{}", &api_key[..4], "***REDACTED***")
    }
}

/// 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_api_key() {
        assert_eq!(sanitize_api_key(""), "");
        assert_eq!(sanitize_api_key("sk-1234"), "***REDACTED***");
        assert!(sanitize_api_key("sk-1234567890abcdefghijklmnop").contains("REDACTED"));
    }
}
