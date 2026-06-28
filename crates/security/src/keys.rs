use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// The master encryption key derived from the user's password via Argon2id.
/// Cleared from memory when dropped.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct MasterKey([u8; 32]);

impl MasterKey {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Derive a domain-specific subkey using HKDF-SHA256.
    pub fn derive_key(&self, domain: &str) -> [u8; 32] {
        let hk = Hkdf::<Sha256>::new(None, &self.0);
        let mut key = [0u8; 32];
        hk.expand(domain.as_bytes(), &mut key)
            .expect("HKDF expand failed — output length is valid");
        key
    }

    pub fn vault_key(&self) -> [u8; 32] {
        self.derive_key("phoenix:vault:v1")
    }

    pub fn document_key(&self) -> [u8; 32] {
        self.derive_key("phoenix:documents:v1")
    }

    pub fn sync_key(&self) -> [u8; 32] {
        self.derive_key("phoenix:sync:v1")
    }
}
