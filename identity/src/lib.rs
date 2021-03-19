// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(docsrs, feature(doc_cfg, extended_key_value_attributes))]
#![cfg_attr(docsrs, cfg_attr(docsrs, doc = include_str!("../README.md")))]
#![cfg_attr(docsrs, warn(missing_docs))]
#![warn(
  rust_2018_idioms,
  unreachable_pub,
  missing_crate_level_docs,
  broken_intra_doc_links,
  private_intra_doc_links,
  private_doc_tests,
  clippy::missing_safety_doc,
  clippy::missing_errors_doc
)]

pub mod core {
  //! Core Traits and Types

  pub use identity_core::common::*;
  pub use identity_core::convert::*;
  pub use identity_core::error::*;
  pub use identity_core::utils::*;

  #[doc(inline)]
  pub use identity_core::diff;

  #[doc(inline)]
  pub use identity_core::json;
}

pub mod crypto {
  //! Cryptographic Utilities

  pub use identity_core::crypto::*;
}

#[cfg(feature = "credential")]
#[cfg_attr(docsrs, doc(cfg(feature = "credential")))]
pub mod credential {
  //! Verifiable Credentials
  //!
  //! [Specification](https://www.w3.org/TR/vc-data-model/)

  pub use identity_credential::credential::*;
  pub use identity_credential::error::*;
  pub use identity_credential::presentation::*;
}

#[cfg(feature = "identifier")]
#[cfg_attr(docsrs, doc(cfg(feature = "identifier")))]
pub mod did {
  //! Decentralized Identifiers
  //!
  //! [Specification](https://www.w3.org/TR/did-core/)

  pub use identity_did::document::*;
  pub use identity_did::error::*;
  pub use identity_did::service::*;
  pub use identity_did::utils::*;
  pub use identity_did::verification::*;

  pub use identity_did::did::did;
  pub use identity_did::did::Error as DIDError;
  pub use identity_did::did::DID;

  pub use identity_did::resolution;
  pub use identity_did::verifiable;
}

#[cfg(feature = "iota")]
#[cfg_attr(docsrs, doc(cfg(feature = "iota")))]
pub mod iota {
  //! IOTA Tangle DID Method

  pub use identity_iota::chain::*;
  pub use identity_iota::client::*;
  pub use identity_iota::credential::*;
  pub use identity_iota::did::*;
  pub use identity_iota::error::*;
  pub use identity_iota::tangle::*;

  #[doc(inline)]
  pub use identity_iota::did;

  #[doc(inline)]
  pub use identity_iota::try_did;
}

pub mod prelude {
  //! Prelude of commonly used types

  pub use identity_core::crypto::KeyPair;
  pub use identity_iota::client::Client;
  pub use identity_iota::did::Document;
  pub use identity_iota::Result;
}
