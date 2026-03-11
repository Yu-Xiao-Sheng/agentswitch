use crate::crypto::error::CryptoError;
use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use argon2::{Algorithm, Argon2, Params, Version};

/// AES-256-GCM 加密器
pub struct Aes256GcmCipher {
    cipher: Aes256Gcm,
}

impl Aes256GcmCipher {
    /// 创建新的加密器
    pub fn new(key: &[u8; 32]) -> Self {
        let cipher = Aes256Gcm::new(key.into());
        Self { cipher }
    }

    /// 从密码派生密钥
    pub fn from_password(password: &str, salt: &[u8; 32]) -> Result<Self, CryptoError> {
        // 使用 Argon2 派生密钥
        let mut key = [0u8; 32];

        let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default());

        argon
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| CryptoError::KeyDerivationFailed(format!("Argon2 哈希失败: {}", e)))?;

        Ok(Self::new(&key))
    }

    /// 加密数据
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        self.cipher
            .encrypt(&nonce, plaintext)
            .map(|mut encrypted| {
                // 将 nonce 添加到加密数据前面
                let mut result = nonce.to_vec();
                result.append(&mut encrypted);
                result
            })
            .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))
    }

    /// 解密数据
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if ciphertext.len() < 12 {
            return Err(CryptoError::DecryptionFailed("密文太短".to_string()));
        }

        let (nonce_bytes, encrypted) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        self.cipher
            .decrypt(nonce, encrypted)
            .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
    }
}
