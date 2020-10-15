use crate::{
    error::Result,
    key::{KeyGenerator, KeyPair, PublicKey, SecretKey},
};

pub trait KeyGen {
    fn generate(&self, generator: KeyGenerator) -> Result<KeyPair>;
}

pub trait Sign {
    fn sign(&self, message: &[u8], secret: &SecretKey) -> Result<Vec<u8>>;
}

pub trait Verify {
    fn verify(&self, message: &[u8], signature: &[u8], public: &PublicKey) -> Result<bool>;
}

pub trait Digest {
    fn digest(&self, message: &[u8]) -> Result<Vec<u8>>;
}

pub trait Proof {
    type Document: ?Sized;
    type Output;

    fn create(&self, document: &Self::Document, secret: &SecretKey) -> Result<Self::Output>;

    fn verify(&self, document: &Self::Document, proof: &Self::Output, public: &PublicKey) -> Result<bool>;
}
