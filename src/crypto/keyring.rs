use crate::crypto::error::CryptoError;
use argon2::{Algorithm, Argon2, Params, Version};

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
        use aes_gcm::aead::OsRng;
        use aes_gcm::aead::rand_core::RngCore;
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
