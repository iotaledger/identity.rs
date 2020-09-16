//! # jose
//!
//! A library for JOSE (JSON Object Signing and Encryption).
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

#[macro_use]
extern crate serde;

mod crypto;

pub mod error;
pub mod jwa;
pub mod jwe;
pub mod jwk;
pub mod jws;
pub mod jwt;
#[doc(hidden)]
pub mod utils;
