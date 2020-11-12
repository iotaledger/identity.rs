use zeroize::Zeroize;

use crate::{
    error::Result,
    key::{KeyGenerator, PublicKey, SecretKey},
    traits::KeyGen,
};

/// A convenience for storing a pair of public/secret keys
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyPair {
    public: PublicKey,
    secret: SecretKey,
}

impl KeyPair {
    /// Creates a new `KeyPair` from the given keys.
    pub const fn new(public: PublicKey, secret: SecretKey) -> Self {
        Self { public, secret }
    }

    /// Returns a reference to the `PublicKey` object.
    pub const fn public(&self) -> &PublicKey {
        &self.public
    }

    /// Returns a reference to the `SecretKey` object.
    pub const fn secret(&self) -> &SecretKey {
        &self.secret
    }

    /// Generates a new `KeyPair`.
    pub fn generate<K>(suite: &K) -> Result<Self>
    where
        K: KeyGen + ?Sized,
    {
        suite.generate(KeyGenerator::None)
    }

    /// Generates a new `KeyPair` with the given `KeyGenerator`.
    pub fn generate_opts<K>(suite: &K, generator: KeyGenerator) -> Result<Self>
    where
        K: KeyGen + ?Sized,
    {
        suite.generate(generator)
    }
}

impl Drop for KeyPair {
    fn drop(&mut self) {
        self.public.zeroize();
        self.secret.zeroize();
    }
}

impl Zeroize for KeyPair {
    fn zeroize(&mut self) {
        self.public.zeroize();
        self.secret.zeroize();
    }
}
