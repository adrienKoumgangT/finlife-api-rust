use base64::{engine::general_purpose::STANDARD, Engine as _};
use chacha20poly1305::{
    aead::{Aead, Payload},
    Key, KeyInit, XChaCha20Poly1305, XNonce,
};
use rand::rngs::OsRng;
use rand::RngCore;
use zeroize::Zeroize;

pub const ENC_PREFIX_V1: &str = "v1:";
const ENC_VERSION: u8 = 1;
const NONCE_LEN: usize = 24;

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("missing encryption key env var {0}")]
    MissingKey(&'static str),
    #[error("invalid encryption key (expected 32 bytes base64)")]
    InvalidKey,
    #[error("invalid ciphertext format")]
    InvalidFormat,
    #[error("decryption failed")]
    DecryptFailed,
    #[error("encryption failed")]
    EncryptFailed,
}

/// 32-byte key for XChaCha20-Poly1305.
/// Keep it secret; generate randomly and store in env var base64.
pub struct EncryptionKey([u8; 32]);

impl Drop for EncryptionKey {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl EncryptionKey {
    /// Load key from env var containing **base64** of 32 random bytes.
    ///
    /// Example (bash):
    ///   head -c 32 /dev/urandom | base64
    pub fn from_env_b64(var: &'static str) -> Result<Self, CryptoError> {
        // Fix: Ensure intermediate variables are zeroized
        let mut raw = std::env::var(var).map_err(|_| CryptoError::MissingKey(var))?;

        let mut bytes = STANDARD
            .decode(raw.trim())
            .map_err(|_| {
                raw.zeroize(); // Zeroize raw string on error
                CryptoError::InvalidKey
            })?;

        // Zeroize the raw string immediately after decoding
        raw.zeroize();

        if bytes.len() != 32 {
            bytes.zeroize(); // Zeroize bytes on error
            return Err(CryptoError::InvalidKey);
        }

        let mut k = [0u8; 32];
        k.copy_from_slice(&bytes);

        // Zeroize the decoded bytes vector now that we have copied to safe storage
        bytes.zeroize();

        Ok(Self(k))
    }

    fn cipher(&self) -> XChaCha20Poly1305 {
        let key = Key::from_slice(&self.0);
        XChaCha20Poly1305::new(key)
    }
}

/// Encrypt bytes → returns a compact string you can store in MySQL (TEXT/VARCHAR).
///
/// `aad` should be stable metadata like: b"user:<uuid>|field:note"
pub fn encrypt_to_string(key: &EncryptionKey, aad: &[u8], plaintext: &[u8]) -> Result<String, CryptoError> {
    let cipher = key.cipher();

    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = XNonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(
            nonce,
            Payload {
                msg: plaintext,
                aad,
            },
        )
        .map_err(|_| CryptoError::EncryptFailed)?;

    // Format: [version(1)][nonce(24)][ciphertext...]
    let mut blob = Vec::with_capacity(1 + NONCE_LEN + ciphertext.len());
    blob.push(ENC_VERSION);
    blob.extend_from_slice(&nonce_bytes);
    blob.extend_from_slice(&ciphertext);

    Ok(format!("{}{}", ENC_PREFIX_V1, STANDARD.encode(blob)))
}

/// Decrypt string → bytes.
///
/// Must use the *same* `aad` you used for encryption.
pub fn decrypt_from_string(key: &EncryptionKey, aad: &[u8], enc: &str) -> Result<Vec<u8>, CryptoError> {
    let s = enc.trim();
    if !s.starts_with(ENC_PREFIX_V1) {
        return Err(CryptoError::InvalidFormat);
    }

    let b64 = &s[ENC_PREFIX_V1.len()..];
    let blob = STANDARD.decode(b64).map_err(|_| CryptoError::InvalidFormat)?;

    // 1 byte version + 24 bytes nonce + 16 bytes Poly1305 tag
    if blob.len() < 1 + NONCE_LEN + 16 {
        return Err(CryptoError::InvalidFormat);
    }
    if blob[0] != ENC_VERSION {
        return Err(CryptoError::InvalidFormat);
    }

    let nonce = XNonce::from_slice(&blob[1..1 + NONCE_LEN]);
    let ciphertext = &blob[1 + NONCE_LEN..];

    let cipher = key.cipher();
    cipher
        .decrypt(
            nonce,
            Payload {
                msg: ciphertext,
                aad,
            },
        )
        .map_err(|_| CryptoError::DecryptFailed)
}

/// Convenience helpers for UTF-8 strings.
pub fn encrypt_string(key: &EncryptionKey, aad: &[u8], plain: &str) -> Result<String, CryptoError> {
    encrypt_to_string(key, aad, plain.as_bytes())
}

pub fn decrypt_to_string(key: &EncryptionKey, aad: &[u8], enc: &str) -> Result<String, CryptoError> {
    let bytes = decrypt_from_string(key, aad, enc)?;
    String::from_utf8(bytes).map_err(|_| CryptoError::InvalidFormat)
}
