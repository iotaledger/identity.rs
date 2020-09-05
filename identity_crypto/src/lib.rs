#[macro_use]
extern crate identity_common;

pub use sha2;
pub use sha3;

#[macro_use]
mod macros;

pub mod error;
pub mod key;
pub mod traits;

pub use error::{Error, Result};
pub use key::{KeyGenerator, KeyPair, PublicKey, SecretKey};
pub use traits::{Digest, KeyGen, Proof, Sign, Verify};
