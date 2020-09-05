#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate serde;

pub mod canonicalize;
pub mod document;
pub mod error;
pub mod signature;
pub mod utils;

pub use canonicalize::{CanonicalJson, Canonicalize, Urdna2015, Urgna2012};
pub use document::LinkedDataDocument;
pub use error::{Error, Result};
pub use signature::{LinkedDataSignature, SignatureOptions, SignatureProof, SignatureSuite};
pub use utils::{decode_b64, encode_b64};
