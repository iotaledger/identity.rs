use zeroize::Zeroize;

use crate::crypto::{PublicKey, SecretKey};

/// A convenience for storing a pair of public/secret keys
#[derive(Clone, Debug)]
pub struct KeyPair(PublicKey, SecretKey);

impl KeyPair {
    /// Creates a new `KeyPair` from the given keys.
    pub const fn new(public: PublicKey, secret: SecretKey) -> Self {
        Self(public, secret)
    }

    /// Returns a reference to the `PublicKey` object.
    pub const fn public(&self) -> &PublicKey {
        &self.0
    }

    /// Returns a reference to the `SecretKey` object.
    pub const fn secret(&self) -> &SecretKey {
        &self.1
    }
}

impl Drop for KeyPair {
    fn drop(&mut self) {
        self.0.zeroize();
        self.1.zeroize();
    }
}

impl Zeroize for KeyPair {
    fn zeroize(&mut self) {
        self.0.zeroize();
        self.1.zeroize();
    }
}
