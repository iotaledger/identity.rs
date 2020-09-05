use zeroize::Zeroize;

use crate::key::SecretKey;

/// The intended method of generating a `KeyPair` with the `KeyGen` trait.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyGenerator {
    /// Generate a new random keypair
    None,
    /// Generate a new keypair from a specific seed
    Seed(Vec<u8>),
    /// Generate a new keypair from a secret key
    Load(SecretKey),
}

impl Default for KeyGenerator {
    fn default() -> Self {
        Self::None
    }
}

impl Drop for KeyGenerator {
    fn drop(&mut self) {
        match self {
            Self::None => {}
            Self::Seed(inner) => inner.zeroize(),
            Self::Load(inner) => inner.zeroize(),
        }
    }
}

impl Zeroize for KeyGenerator {
    fn zeroize(&mut self) {
        match self {
            Self::None => {}
            Self::Seed(inner) => inner.zeroize(),
            Self::Load(inner) => inner.zeroize(),
        }
    }
}
