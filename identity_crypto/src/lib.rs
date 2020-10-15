pub use sha2;
pub use sha3;

#[macro_use]
mod macros;

pub mod error;
pub mod key;
pub mod signature;
pub mod traits;

pub use error::{Error, Result};
pub use key::{KeyGenerator, KeyPair, PublicKey, SecretKey};
pub use signature::{
    EcdsaP256Sha256, EcdsaP384Sha384, Ed25519, HmacSha256, HmacSha384, HmacSha512, RsaPkcs1Sha256, RsaPkcs1Sha384,
    RsaPkcs1Sha512, RsaPssSha256, RsaPssSha384, RsaPssSha512, Secp256k1,
};
pub use traits::{Digest, KeyGen, Proof, Sign, Verify};
