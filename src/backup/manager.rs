use anyhow::{Context, Result};
use chrono::Utc;
use fs2::FileExt;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use crate::agents::Backup;

/// 备份信息（用于列表显示）
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BackupInfo {
    pub agent_name: String,
    pub timestamp: String,
    pub file_path: PathBuf,
    pub size_bytes: u64,
}

/// 备份管理器
pub struct BackupManager {
    /// 备份目录路径
    backup_dir: PathBuf,
    /// 每个工具最大备份数量
    max_per_agent: usize,
}

impl BackupManager {
    /// 创建新的备份管理器
    pub fn new() -> Result<Self> {
        let backup_dir = dirs::home_dir()
            .context("无法获取用户主目录")?
            .join(".agentswitch")
            .join("backups");

        // 创建备份目录
        fs::create_dir_all(&backup_dir).context("创建备份目录失败")?;

        Ok(Self {
            backup_dir,
            max_per_agent: 10,
        })
    }

    /// 设置每个工具的最大备份数量
    pub fn with_max_backups(mut self, max: usize) -> Self {
        self.max_per_agent = max;
        self
    }

    /// 创建配置文件备份
    ///
    /// # 参数
    /// - `agent_name`: 工具名称（如 "claude-code"）
    /// - `config_path`: 配置文件路径
    /// - `format`: 配置文件格式（json/toml/yaml）
    pub fn create_backup(
        &self,
        agent_name: &str,
        config_path: &Path,
        format: &str,
    ) -> Result<Backup> {
        // 检查配置文件是否存在
        if !config_path.exists() {
            anyhow::bail!("配置文件不存在: {:?}", config_path);
        }

        // 检查磁盘空间（简单检查：至少需要原文件大小的2倍空间）
        let file_size = fs::metadata(config_path)?.len();
        self.check_disk_space(file_size * 2)?;

        // 生成时间戳
        let timestamp = Utc::now();

        // 生成备份文件名
        let backup_filename = format!("backup-{}.{}", timestamp.format("%Y%m%d-%H%M%S"), format);
        let backup_dir = self.backup_dir.join(agent_name);

        // 创建工具特定的备份目录
        fs::create_dir_all(&backup_dir).context("创建备份目录失败")?;

        let backup_path = backup_dir.join(&backup_filename);

        // 使用文件锁确保原子性
        let _lock = self.acquire_lock()?;

        // 复制配置文件到备份位置
        fs::copy(config_path, &backup_path).context("创建备份文件失败")?;

        // 设置备份文件权限为 0600
        #[cfg(unix)]
        {
            #[allow(unused_imports)]
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&backup_path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&backup_path, perms).context("设置备份文件权限失败")?;
        }

        // 清理旧备份（保留最新的 max_per_agent 个）
        self.cleanup_old_backups(agent_name)?;

        Ok(Backup {
            agent_name: agent_name.to_string(),
            original_config_path: config_path.to_path_buf(),
            backup_path,
            timestamp,
        })
    }

    /// 恢复备份
    pub fn restore_backup(&self, backup: &Backup) -> Result<()> {
        if !backup.backup_path.exists() {
            anyhow::bail!("备份文件不存在: {:?}", backup.backup_path);
        }

        // 使用文件锁确保原子性
        let _lock = self.acquire_lock()?;

        // 创建当前配置的备份（防止恢复失败）
        let original_path = &backup.original_config_path;
        if original_path.exists() {
            let timestamp = Utc::now().format("%Y%m%d-%H%M%S");
            let restore_backup_name = format!("restore-{}.bak", timestamp);
            let restore_backup_path = self
                .backup_dir
                .join(&backup.agent_name)
                .join(&restore_backup_name);

            let _ = fs::copy(original_path, &restore_backup_path);
        }

        // 复制备份文件到原位置
        fs::copy(&backup.backup_path, original_path).context("恢复配置文件失败")?;

        Ok(())
    }

    /// 清理指定时间之前的备份
    ///
    /// # 参数
    /// - `older_than`: 备份文件的最长保留时间（秒）
    pub fn clean_old_backups_by_duration(&self, older_seconds: i64) -> Result<usize> {
        let now = Utc::now();
        let entries = fs::read_dir(&self.backup_dir).context("读取备份目录失败")?;

        let mut cleaned_count = 0;

        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() {
                if let Ok(metadata) = fs::metadata(&path) {
                    if let Ok(modified) = metadata.modified() {
                        let modified_time: chrono::DateTime<chrono::Utc> = modified.into();
                        let age = now.signed_duration_since(modified_time);

                        if age.num_seconds() > older_seconds {
                            fs::remove_file(&path)?;
                            cleaned_count += 1;
                        }
                    }
                }
            }
        }

        Ok(cleaned_count)
    }

    /// 列出所有备份（所有工具）
    pub fn list_all_backups(&self) -> Result<Vec<BackupInfo>> {
        let mut backups = Vec::new();

        if !self.backup_dir.exists() {
            return Ok(backups);
        }

        let entries = fs::read_dir(&self.backup_dir).context("读取备份目录失败")?;

        for entry in entries.flatten() {
            let agent_dir = entry.path();

            if agent_dir.is_dir() {
                let agent_name = agent_dir
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                if let Ok(entries) = fs::read_dir(&agent_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();

                        if path.is_file()
                            && (path.extension().and_then(|s| s.to_str()) == Some("json")
                                || path.extension().and_then(|s| s.to_str()) == Some("toml"))
                        {
                            #[allow(clippy::collapsible_if)]
                            if let Ok(metadata) = fs::metadata(&path) {
                                if let Ok(modified) = metadata.modified() {
                                    let modified_time: chrono::DateTime<chrono::Utc> =
                                        modified.into();
                                    backups.push(BackupInfo {
                                        agent_name: agent_name.to_string(),
                                        timestamp: modified_time
                                            .format("%Y-%m-%d %H:%M:%S")
                                            .to_string(),
                                        file_path: path.clone(),
                                        size_bytes: metadata.len(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        // 按时间戳排序（最新的在前）
        backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(backups)
    }

    /// 根据时间戳查找备份
    pub fn find_backup(&self, agent: &str, timestamp: &str) -> Result<Backup> {
        let agent_dir = self.backup_dir.join(agent);

        if !agent_dir.exists() {
            anyhow::bail!("未找到 {} 的备份", agent);
        }

        let entries = fs::read_dir(&agent_dir).context("读取备份目录失败")?;

        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() {
                let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                if filename.contains(timestamp) {
                    let modified = fs::metadata(&path)?.modified()?;
                    let timestamp_dt: chrono::DateTime<chrono::Utc> = modified.into();

                    return Ok(Backup {
                        agent_name: agent.to_string(),
                        original_config_path: PathBuf::new(), // 需要恢复时才知道
                        backup_path: path,
                        timestamp: timestamp_dt,
                    });
                }
            }
        }

        anyhow::bail!("未找到时间戳为 {} 的备份", timestamp)
    }

    /// 清理旧备份（保留最新的 max_per_agent 个）
    fn cleanup_old_backups(&self, agent_name: &str) -> Result<()> {
        let backup_dir = self.backup_dir.join(agent_name);

        if !backup_dir.exists() {
            return Ok(());
        }

        let mut backups: Vec<_> = fs::read_dir(&backup_dir)
            .context("读取备份目录失败")?
            .filter_map(|entry| entry.ok())
            .collect();

        // 按修改时间排序（最新的在前）
        backups.sort_by(|a, b| {
            let a_time = b.metadata().ok().and_then(|m| m.modified().ok());
            let b_time = a.metadata().ok().and_then(|m| m.modified().ok());
            b_time.cmp(&a_time)
        });

        // 如果备份数量超过限制，删除最旧的
        if backups.len() > self.max_per_agent {
            for old_backup in backups.into_iter().skip(self.max_per_agent) {
                let path = old_backup.path();
                if path.is_file() {
                    let _ = fs::remove_file(path);
                }
            }
        }

        Ok(())
    }

    /// 检查磁盘空间
    fn check_disk_space(&self, required_bytes: u64) -> Result<()> {
        // 简化实现：在 Unix 系统上检查 statvfs
        #[cfg(unix)]
        {
            
            if let Ok(stat) = fs2::statvfs(&self.backup_dir) {
                let available = stat.available_space();
                if available < required_bytes {
                    anyhow::bail!(
                        "磁盘空间不足。需要 {} 字节，可用 {} 字节。\n\
                        建议运行 'asw backup clean' 清理旧备份或清理磁盘空间",
                        required_bytes,
                        available
                    );
                }
            }
        }

        Ok(())
    }

    /// 获取文件锁
    fn acquire_lock(&self) -> Result<File> {
        let lock_path = self.backup_dir.join(".lock");
        let file = File::create(&lock_path).context("创建锁文件失败")?;

        // 尝试获取独占锁
        #[allow(clippy::let_unit_value)]
        let _try_lock = file
            .try_lock_exclusive()
            .context("获取文件锁失败，可能有其他进程正在操作")?;

        Ok(file)
    }
}

impl Default for BackupManager {
    fn default() -> Self {
        Self::new().expect("创建 BackupManager 失败")
    }
}
