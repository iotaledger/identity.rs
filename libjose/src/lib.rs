//! # jose
//!
//! A library for JSON Object Signing and Encryption (JOSE).
//!
//! ## References
//! - [IANA JOSE Registry](https://www.iana.org/assignments/jose/jose.xhtml)
//! - [JWT Handbook](https://auth0.com/e-books/jwt-handbook)
//!
//! ### RFCs
//! - [JSON Web Algorithms](https://tools.ietf.org/html/rfc7518)
//! - [JSON Web Encryption](https://tools.ietf.org/html/rfc7516)
//! - [JSON Web Key](https://tools.ietf.org/html/rfc7517)
//! - [JSON Web Key Thumbprint](https://tools.ietf.org/html/rfc7638)
//! - [JSON Web Signature](https://tools.ietf.org/html/rfc7515)
//! - [JSON Web Signature Unencoded Payload Option](https://tools.ietf.org/html/rfc7797)
//! - [JSON Web Token](https://tools.ietf.org/html/rfc7519)
//! - [CFRG Elliptic Curve ECDH and Signatures](https://tools.ietf.org/html/rfc8037)
//! - [JOSE Registrations for Web Authentication Algorithms](https://tools.ietf.org/html/rfc8812)
//! - [Chacha derived AEAD algorithms in JOSE](https://tools.ietf.org/html/draft-amringer-jose-chacha-01)
//! - [JSON Web Token Best Current Practices](https://tools.ietf.org/html/rfc8725)
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "alloc"))]
compile_error!("This crate does not yet support environments without liballoc.");

#[cfg(not(feature = "std"))]
extern crate alloc as alloc_;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate serde;

#[cfg(feature = "std")]
mod alloc {
  pub use ::std::boxed::Box;
  pub use ::std::collections::BTreeMap;
  pub use ::std::string::String;
  pub use ::std::string::ToString;
  pub use ::std::vec::Vec;
}

#[cfg(not(feature = "std"))]
mod alloc {
  pub use ::alloc_::boxed::Box;
  pub use ::alloc_::collections::BTreeMap;
  pub use ::alloc_::string::String;
  pub use ::alloc_::string::ToString;
  pub use ::alloc_::vec::Vec;
}

#[doc(hidden)]
pub mod crypto;
pub mod error;
pub mod jwa;
pub mod jwe;
pub mod jwk;
pub mod jws;
pub mod jwt;
#[doc(hidden)]
pub mod utils;
