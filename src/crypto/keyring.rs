//! 密钥管理模块
//!
//! 提供密钥的生成、存储、加载、导入/导出功能。

use crate::crypto::error::CryptoError;
use crate::utils::permissions::{create_directory_with_700_perms, set_file_permissions};
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::OsRng;
use argon2::{Algorithm, Argon2, Params, Version};
use base64::{engine::general_purpose, Engine as _};
use std::path::PathBuf;

/// 密钥长度（AES-256 需要 32 字节）
const KEY_LEN: usize = 32;

/// 获取密钥目录路径 (~/.agentswitch/keys/)
fn keys_dir() -> Result<PathBuf, CryptoError> {
    let home = dirs::home_dir()
        .ok_or_else(|| CryptoError::KeyNotFound("无法确定用户主目录".to_string()))?;
    Ok(home.join(".agentswitch").join("keys"))
}

/// 获取主密钥文件路径 (~/.agentswitch/keys/master.key)
fn master_key_path() -> Result<PathBuf, CryptoError> {
    Ok(keys_dir()?.join("master.key"))
}

/// 生成 32 字节随机密钥
pub fn generate_key() -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    OsRng.fill_bytes(&mut key);
    key
}

/// 将密钥保存到文件
///
/// 密钥文件权限设置为 600（仅所有者可读写），
/// 密钥目录权限设置为 700。
fn save_key_to_file(key: &[u8; KEY_LEN], path: &PathBuf) -> Result<(), CryptoError> {
    // 确保目录存在且有正确权限
    if let Some(parent) = path.parent() {
        create_directory_with_700_perms(parent)
            .map_err(|e| CryptoError::Io(std::io::Error::other(e.to_string())))?;
    }

    // 写入密钥文件（二进制格式）
    std::fs::write(path, key)?;

    // 设置文件权限为 600
    set_file_permissions(path)
        .map_err(|e| CryptoError::Io(std::io::Error::other(e.to_string())))?;

    Ok(())
}

/// 从文件读取密钥
fn load_key_from_file(path: &PathBuf) -> Result<[u8; KEY_LEN], CryptoError> {
    if !path.exists() {
        return Err(CryptoError::KeyNotFound(format!(
            "密钥文件不存在: {}",
            path.display()
        )));
    }

    let data = std::fs::read(path)?;

    if data.len() != KEY_LEN {
        return Err(CryptoError::KeyInvalid(format!(
            "密钥文件长度无效: 期望 {} 字节，实际 {} 字节",
            KEY_LEN,
            data.len()
        )));
    }

    let mut key = [0u8; KEY_LEN];
    key.copy_from_slice(&data);
    Ok(key)
}

/// 生成新密钥并保存到 ~/.agentswitch/keys/master.key
///
/// 如果密钥已存在，返回错误以防止意外覆盖。
pub fn generate_and_save_master_key() -> Result<[u8; KEY_LEN], CryptoError> {
    let path = master_key_path()?;

    if path.exists() {
        return Err(CryptoError::KeyAlreadyExists(format!(
            "主密钥已存在: {}。如需重新生成，请先删除旧密钥文件。",
            path.display()
        )));
    }

    let key = generate_key();
    save_key_to_file(&key, &path)?;

    Ok(key)
}

/// 加载主密钥
///
/// 从 ~/.agentswitch/keys/master.key 读取密钥。
pub fn load_master_key() -> Result<[u8; KEY_LEN], CryptoError> {
    let path = master_key_path()?;
    load_key_from_file(&path)
}

/// 检查主密钥是否存在
pub fn master_key_exists() -> Result<bool, CryptoError> {
    let path = master_key_path()?;
    Ok(path.exists())
}

/// 将密钥导出为 Base64 字符串
pub fn export_key_to_base64(key: &[u8; KEY_LEN]) -> String {
    general_purpose::STANDARD.encode(key)
}

/// 从 Base64 字符串导入密钥
pub fn import_key_from_base64(b64: &str) -> Result<[u8; KEY_LEN], CryptoError> {
    let decoded = general_purpose::STANDARD
        .decode(b64.trim())
        .map_err(|e| CryptoError::KeyInvalid(format!("Base64 解码失败: {}", e)))?;

    if decoded.len() != KEY_LEN {
        return Err(CryptoError::KeyInvalid(format!(
            "密钥长度无效: 期望 {} 字节，实际 {} 字节",
            KEY_LEN,
            decoded.len()
        )));
    }

    let mut key = [0u8; KEY_LEN];
    key.copy_from_slice(&decoded);
    Ok(key)
}

/// 从 Base64 导入密钥并保存为主密钥
///
/// 如果密钥已存在，返回错误以防止意外覆盖。
pub fn import_and_save_master_key(b64: &str) -> Result<[u8; KEY_LEN], CryptoError> {
    let path = master_key_path()?;

    if path.exists() {
        return Err(CryptoError::KeyAlreadyExists(format!(
            "主密钥已存在: {}。如需覆盖，请先删除旧密钥文件。",
            path.display()
        )));
    }

    let key = import_key_from_base64(b64)?;
    save_key_to_file(&key, &path)?;

    Ok(key)
}

/// 获取主密钥文件路径（供状态显示使用）
pub fn get_master_key_path_str() -> Result<String, CryptoError> {
    let path = master_key_path()?;
    Ok(path.display().to_string())
}

/// 密钥派生函数
pub struct KeyDerivation {
    argon: Argon2<'static>,
}

impl KeyDerivation {
    /// 创建新的密钥派生实例
    pub fn new() -> Self {
        Self {
            argon: Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default()),
        }
    }

    /// 使用自定义参数
    pub fn with_params(m_cost: u32, t_cost: u32, p_cost: u32) -> Result<Self, CryptoError> {
        let params = Params::new(m_cost, t_cost, p_cost, None)
            .map_err(|e| CryptoError::KeyDerivationFailed(format!("无效的参数: {}", e)))?;

        Ok(Self {
            argon: Argon2::new(Algorithm::Argon2id, Version::V0x13, params),
        })
    }

    /// 从密码派生密钥
    pub fn derive_key(&self, password: &str, salt: &[u8; 32]) -> Result<[u8; 32], CryptoError> {
        let mut key = [0u8; 32];

        self.argon
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| CryptoError::KeyDerivationFailed(format!("密钥派生失败: {}", e)))?;

        Ok(key)
    }

    /// 生成随机盐值
    pub fn generate_salt() -> Result<[u8; 32], CryptoError> {
        let mut salt = [0u8; 32];
        OsRng.fill_bytes(&mut salt);
        Ok(salt)
    }
}

impl Default for KeyDerivation {
    fn default() -> Self {
        Self::new()
    }
}
