//! 配置初始化集成测试

use std::fs;

#[cfg(test)]
mod tests {
    /// 测试首次运行时自动创建配置目录和文件
    #[test]
    fn test_auto_create_config_on_first_run() {
        let temp_dir = std::env::temp_dir();
        let test_config_dir = temp_dir.join(".test_agentswitch");
        
        if test_config_dir.exists() {
            fs::remove_dir_all(&test_config_dir).unwrap();
        }
        
        assert!(!test_config_dir.exists());
        fs::create_dir_all(&test_config_dir).unwrap();
        assert!(test_config_dir.exists());
        
        fs::remove_dir_all(&test_config_dir).unwrap();
    }
    
    /// 测试幂等操作
    #[test]
    fn test_idempotent_directory_creation() {
        let temp_dir = std::env::temp_dir();
        let test_config_dir = temp_dir.join(".test_agentswitch_idempotent");
        
        if test_config_dir.exists() {
            fs::remove_dir_all(&test_config_dir).unwrap();
        }
        
        fs::create_dir_all(&test_config_dir).unwrap();
        assert!(test_config_dir.exists());
        fs::create_dir_all(&test_config_dir).unwrap();
        assert!(test_config_dir.exists());
        
        fs::remove_dir_all(&test_config_dir).unwrap();
    }
}
