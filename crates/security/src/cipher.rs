use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CipherError {
    #[error("Encryption failed")]
    EncryptFailed,
    #[error("Decryption failed — data may be tampered or key is wrong")]
    DecryptFailed,
    #[error("Invalid base64 encoding")]
    InvalidBase64,
    #[error("Invalid nonce length")]
    InvalidNonce,
}

/// Encrypt `plaintext` with AES-256-GCM using the provided 32-byte key.
/// Returns a base64-encoded string: `nonce || ciphertext`.
pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<String, CipherError> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, plaintext).map_err(|_| CipherError::EncryptFailed)?;

    let mut output = nonce.to_vec();
    output.extend_from_slice(&ciphertext);
    Ok(BASE64.encode(output))
}

/// Decrypt a base64-encoded `nonce || ciphertext` blob produced by [`encrypt`].
pub fn decrypt(key: &[u8; 32], encoded: &str) -> Result<Vec<u8>, CipherError> {
    let data = BASE64.decode(encoded).map_err(|_| CipherError::InvalidBase64)?;
    if data.len() < 12 {
        return Err(CipherError::InvalidNonce);
    }
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    cipher.decrypt(nonce, ciphertext).map_err(|_| CipherError::DecryptFailed)
}
