// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![doc = include_str!("./../README.md")]
#![allow(clippy::upper_case_acronyms)]
#![warn(
  rust_2018_idioms,
  unreachable_pub,
  missing_docs,
  rustdoc::missing_crate_level_docs,
  rustdoc::broken_intra_doc_links,
  rustdoc::private_intra_doc_links,
  rustdoc::private_doc_tests,
  clippy::missing_safety_doc
)]

#[macro_use]
extern crate serde;

mod error;
pub mod jose;
pub mod verification_method;
pub use error::Error;
pub use error::Result;
pub use jose::jwk;
pub use jose::jws;
pub use jose::jwu;
pub use verification_method::*;

pub trait ProofT {
  type VerificationMethod;

  fn algorithm(&self) -> &str;
  fn signature(&self) -> &[u8];
  fn signing_input(&self) -> &[u8];
  fn verification_method(&self) -> Self::VerificationMethod;
}

impl<'a, P> ProofT for &'a P
where
  P: ProofT,
{
  type VerificationMethod = P::VerificationMethod;
  fn algorithm(&self) -> &str {
    P::algorithm(self)
  }
  fn signature(&self) -> &[u8] {
    P::signature(self)
  }
  fn signing_input(&self) -> &[u8] {
    P::signature(self)
  }
  fn verification_method(&self) -> Self::VerificationMethod {
    P::verification_method(self)
  }
}

pub trait VerifierT<K> {
  type Error;

  fn verify<P: ProofT>(&self, proof: &P, key: &K) -> Result<(), Self::Error>;
}
