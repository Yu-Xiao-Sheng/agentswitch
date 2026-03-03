use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// 检查文件权限，检测是否为只读
pub fn check_file_readonly(path: &Path) -> Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    let metadata = fs::metadata(path)
        .context("读取文件元数据失败")?;

    #[cfg(unix)]
    {
        let perms = metadata.permissions();
        let mode = perms.mode();
        // 检查是否所有者没有写权限
        let is_readonly = (mode & 0o200) == 0;
        Ok(is_readonly)
    }

    #[cfg(not(unix))]
    {
        // 在非 Unix 系统上，尝试简单检查
        let readonly = metadata.permissions().readonly();
        Ok(readonly)
    }
}

/// 跟随符号链接，返回实际文件路径
pub fn resolve_symlink(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Ok(path.to_path_buf());
    }

    // 在 Unix 系统上跟随符号链接
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if let Ok(metadata) = fs::metadata(path) {
            let file_type = metadata.file_type();
            if file_type.is_symlink() {
                return fs::canonicalize(path)
                    .context("解析符号链接失败");
            }
        }
    }

    Ok(path.to_path_buf())
}

/// 检查磁盘可用空间
///
/// # 参数
/// - `path`: 检查路径
/// - `required_bytes`: 需要的字节数
///
/// # 返回
/// 如果空间不足，返回错误
pub fn check_disk_space(path: &Path, required_bytes: u64) -> Result<()> {
    #[cfg(unix)]
    {
        if let Ok(stat) = fs2::statvfs(path) {
            let available = stat.available_space();
            if available < required_bytes {
                anyhow::bail!(
                    "磁盘空间不足。需要 {} 字节，可用 {} 字节",
                    required_bytes,
                    available
                );
            }
        }
    }

    #[cfg(not(unix))]
    {
        // 在非 Unix 系统上，跳过磁盘空间检查
        // 或者可以使用其他方法
    }

    Ok(())
}

/// 原子写入文件
///
/// 先写入临时文件，然后原子重命名
pub fn atomic_write(path: &Path, content: &[u8]) -> Result<()> {
    // 创建临时文件
    let temp_path = path.with_extension("tmp");

    // 写入临时文件
    fs::write(&temp_path, content)
        .context("写入临时文件失败")?;

    // 原子重命名
    fs::rename(&temp_path, path)
        .context("移动文件失败")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_write() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let content = b"Hello, World!";
        atomic_write(&file_path, content).unwrap();

        assert_eq!(fs::read(&file_path).unwrap(), content);
    }
}
