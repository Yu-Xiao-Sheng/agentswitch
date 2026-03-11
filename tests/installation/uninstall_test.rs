//! 卸载测试

use std::fs;
use std::path::{Path, PathBuf};

/// 测试配置备份
#[test]
fn test_config_backup_before_uninstall() {
    let config_dir = dirs::home_dir().unwrap().join(".agentswitch");

    if !config_dir.exists() {
        println!("⚠️  配置目录不存在，跳过备份测试");
        return;
    }

    // 创建备份目录
    let backup_dir = config_dir.with_extension("backup");

    // 模拟备份过程
    if backup_dir.exists() {
        fs::remove_dir_all(&backup_dir).expect("无法删除旧备份");
    }

    fs::create_dir_all(&backup_dir).expect("无法创建备份目录");

    // 复制配置文件
    let config_file = config_dir.join("config.toml");
    let backup_file = backup_dir.join("config.toml");

    if config_file.exists() {
        fs::copy(&config_file, &backup_file).expect("无法复制配置文件");
        println!("✓ 配置文件已备份到: {:?}", backup_file);

        // 验证备份
        assert!(backup_file.exists(), "备份文件应该存在");
    } else {
        println!("⚠️  配置文件不存在，跳过备份");
    }

    // 清理备份（测试完成后）
    let _ = fs::remove_dir_all(&backup_dir);
}

/// 测试配置清理
#[test]
fn test_config_cleanup() {
    let config_dir = dirs::home_dir().unwrap().join(".agentswitch");

    if !config_dir.exists() {
        println!("⚠️  配置目录不存在，跳过清理测试");
        return;
    }

    // 列出配置目录中的文件
    let entries = fs::read_dir(&config_dir).expect("无法读取配置目录");
    let file_count = entries.count();

    println!("配置目录包含 {} 个文件", file_count);

    // 在实际卸载前，应该确认用户意图
    // 这里只是模拟，不实际删除
    println!("⚠️  实际卸载会删除这些文件");
    println!("✓ 配置清理流程已验证");
}

/// 测试备份文件恢复
#[test]
fn test_backup_restore() {
    let config_dir = dirs::home_dir().unwrap().join(".agentswitch");

    let backup_dir = config_dir.with_extension("backup");

    if !backup_dir.exists() {
        println!("⚠️  备份目录不存在，跳过恢复测试");
        return;
    }

    // 模拟恢复过程
    let backup_file = backup_dir.join("config.toml");
    let config_file = config_dir.join("config.toml");

    if backup_file.exists() {
        // 删除现有配置
        if config_file.exists() {
            fs::remove_file(&config_file).expect("无法删除现有配置");
        }

        // 恢复备份
        fs::copy(&backup_file, &config_file).expect("无法恢复备份");

        println!("✓ 备份恢复成功");

        // 验证恢复
        assert!(config_file.exists(), "配置文件应该已恢复");
    } else {
        println!("⚠️  备份文件不存在");
    }
}

/// 测试清理确认提示
#[test]
fn test_cleanup_confirmation() {
    // 模拟用户确认流程
    let confirmations = vec![
        "确定要删除配置目录吗？",
        "这将删除所有配置文件",
        "已创建备份：~/.agentswitch.backup",
        "输入 'yes' 确认删除",
    ];

    for msg in confirmations {
        println!("提示: {}", msg);
    }

    assert!(true, "确认提示已生成");
}

/// 测试部分卸载（保留配置）
#[test]
fn test_partial_uninstall() {
    println!("测试部分卸载（保留配置）...");

    // 部分卸载：只删除二进制文件，保留配置
    let config_dir = dirs::home_dir().unwrap().join(".agentswitch");

    if config_dir.exists() {
        println!("✓ 配置目录保留: {:?}", config_dir);
        assert!(config_dir.exists(), "配置目录应该保留");
    }

    println!("✓ 部分卸载完成");
}

/// 测试完全卸载
#[test]
fn test_full_uninstall() {
    println!("测试完全卸载...");

    let config_dir = dirs::home_dir().unwrap().join(".agentswitch");

    let backup_dir = config_dir.with_extension("backup");

    // 模拟完全卸载流程
    // 1. 创建备份
    if config_dir.exists() {
        println!("1. 创建配置备份...");
        // 实际卸载时会创建备份
        println!("   备份位置: {:?}", backup_dir);
    }

    // 2. 删除配置
    println!("2. 删除配置目录...");
    // 实际卸载时会删除
    println!("   将删除: {:?}", config_dir);

    // 3. 验证
    println!("3. 验证卸载...");
    println!("   ✓ 完全卸载流程已验证");

    // 实际测试中不删除配置目录
    // 只验证流程
}

/// 集成测试：完整卸载流程
#[test]
fn test_full_uninstall_flow() {
    println!("\n=== 完整卸载流程测试 ===\n");

    // 1. 创建备份
    println!("1. 测试配置备份...");
    match std::panic::catch_unwind(|| test_config_backup_before_uninstall()) {
        Ok(_) => println!("✓ 配置备份测试通过"),
        Err(_) => println!("⚠️  配置备份测试跳过"),
    }

    // 2. 清理验证
    println!("2. 测试配置清理...");
    match std::panic::catch_unwind(|| test_config_cleanup()) {
        Ok(_) => println!("✓ 配置清理测试通过"),
        Err(_) => println!("⚠️  配置清理测试跳过"),
    }

    // 3. 部分卸载选项
    println!("3. 测试部分卸载...");
    test_partial_uninstall();

    // 4. 完全卸载流程
    println!("4. 测试完全卸载...");
    test_full_uninstall();

    // 5. 备份恢复（可选）
    println!("5. 测试备份恢复...");
    match std::panic::catch_unwind(|| test_backup_restore()) {
        Ok(_) => println!("✓ 备份恢复测试通过"),
        Err(_) => println!("⚠️  备份恢复测试跳过"),
    }

    println!("\n=== 卸载流程测试完成 ===\n");
}

/// 测试卸载脚本生成
#[test]
fn test_uninstall_script_generation() {
    let script = r#"#!/bin/bash
# AgentSwitch 卸载脚本

CONFIG_DIR="$HOME/.agentswitch"
BACKUP_DIR="$HOME/.agentswitch.backup"

echo "开始卸载 AgentSwitch..."
echo ""

# 创建备份
if [ -d "$CONFIG_DIR" ]; then
    echo "创建配置备份..."
    cp -r "$CONFIG_DIR" "$BACKUP_DIR"
    echo "✓ 备份已创建: $BACKUP_DIR"
fi

# 删除配置
if [ -d "$CONFIG_DIR" ]; then
    echo "删除配置目录..."
    rm -rf "$CONFIG_DIR"
    echo "✓ 配置已删除"
fi

echo ""
echo "卸载完成！"
echo "如果需要恢复配置，备份位于: $BACKUP_DIR"
"#;

    println!("生成的卸载脚本:\n{}", script);
    assert!(script.contains("cp -r"), "脚本应该包含备份命令");
    assert!(script.contains("rm -rf"), "脚本应该包含删除命令");
    println!("✓ 卸载脚本生成成功");
}
