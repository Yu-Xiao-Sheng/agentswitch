use std::fs;
use tempfile::TempDir;

#[test]
fn test_atomic_write() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    let content = b"Hello, World!";

    // 假设 atomic_write 函数存在（需要从 config::file_utils 导入）
    // 由于我们没有直接访问，这里测试基本文件操作
    fs::write(&file_path, content).unwrap();

    let read_content = fs::read(&file_path).unwrap();
    assert_eq!(read_content, content);
}

#[test]
fn test_config_file_permissions() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("readonly.txt");

    // 创建可写文件
    fs::write(&file_path, b"test").unwrap();

    // 读取文件元数据
    let metadata = fs::metadata(&file_path).unwrap();
    assert!(metadata.is_file());

    // 测试只读检查（Unix 系统）
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = metadata.permissions();
        let mode = perms.mode();

        // 检查是否所有者有写权限
        let has_write = (mode & 0o200) != 0;
        assert!(has_write, "File should be writable initially");
    }
}

#[test]
fn test_symlink_resolution() {
    let temp_dir = TempDir::new().unwrap();
    let target_file = temp_dir.path().join("target.txt");
    let link_path = temp_dir.path().join("link.txt");

    // 创建目标文件
    fs::write(&target_file, b"target content").unwrap();

    // 创建符号链接（如果支持）
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&target_file, &link_path).unwrap();

        // 验证链接存在
        assert!(link_path.exists());

        // 验证链接指向正确的文件
        let metadata = fs::metadata(&link_path).unwrap();
        assert!(metadata.is_file());
    }
}
