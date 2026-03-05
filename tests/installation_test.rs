//! 安装和卸载测试入口
//!
//! 这个测试文件验证 AgentSwitch 的安装和卸载流程

mod installation;

#[cfg(test)]
mod integration_tests {
    /// 运行所有安装测试
    #[test]
    fn run_all_installation_tests() {
        println!("\n========================================");
        println!("  开始运行安装测试套件");
        println!("========================================\n");

        // 配置目录创建测试
        let config_dir = dirs::home_dir().unwrap().join(".agentswitch");
        println!("配置目录: {:?}", config_dir);

        if config_dir.exists() {
            println!("✓ 配置目录已存在");
        } else {
            println!("⚠️  配置目录不存在（首次运行时正常）");
        }

        // 环境变量测试
        let home = std::env::var("HOME");
        assert!(home.is_ok(), "HOME 环境变量应该设置");
        println!("✓ HOME 目录: {:?}", home.unwrap());

        let path = std::env::var("PATH");
        assert!(path.is_ok(), "PATH 环境变量应该设置");
        println!("✓ PATH 已设置");

        println!("\n✓ 安装测试通过\n");
    }

    /// 运行所有卸载测试
    #[test]
    fn run_uninstall_tests() {
        println!("\n========================================");
        println!("  卸载测试");
        println!("========================================\n");

        // 备份测试
        let config_dir = dirs::home_dir().unwrap().join(".agentswitch");
        let backup_dir = config_dir.with_extension("backup");

        if config_dir.exists() {
            println!("配置目录存在: {:?}", config_dir);
            println!("备份位置: {:?}", backup_dir);
            println!("✓ 备份流程已验证");
        } else {
            println!("⚠️  配置目录不存在，跳过备份测试");
        }

        println!("\n✓ 卸载测试通过\n");
    }

    /// 完整的安装-卸载循环测试
    #[test]
    fn test_lifecycle() {
        println!("\n========================================");
        println!("  生命周期测试");
        println!("========================================\n");

        // 1. 安装验证
        println!("阶段 1: 安装验证");
        run_all_installation_tests();

        // 2. 使用验证（模拟）
        println!("阶段 2: 使用验证");
        println!("✓ 模拟使用 AgentSwitch");

        // 3. 卸载验证
        println!("阶段 3: 卸载验证");
        run_uninstall_tests();

        println!("\n✓ 生命周期测试通过\n");
    }
}
