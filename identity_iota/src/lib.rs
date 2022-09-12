// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![allow(deprecated)]
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
  clippy::missing_safety_doc,
  clippy::missing_errors_doc
)]

pub mod core {
  //! Core Traits and Types

  pub use identity_core::common::*;
  pub use identity_core::convert::*;
  pub use identity_core::error::*;
  pub use identity_core::utils::*;

  #[deprecated(since = "0.5.0", note = "diff chain features are slated for removal")]
  #[doc(inline)]
  pub use identity_core::diff;

  #[doc(inline)]
  pub use identity_core::json;
}

pub mod crypto {
  //! Cryptographic Utilities

  pub use identity_core::crypto::*;
}

pub mod credential {
  //! Verifiable Credentials
  //!
  //! [Specification](https://www.w3.org/TR/vc-data-model/)

  pub use identity_credential::credential::*;
  pub use identity_credential::error::*;
  pub use identity_credential::presentation::*;
  pub use identity_credential::validator::*;
}

pub mod did {
  //! Decentralized Identifiers
  //!
  //! [Specification](https://www.w3.org/TR/did-core/)

  pub use identity_did::document::*;
  pub use identity_did::error::*;
  #[cfg(feature = "revocation-bitmap")]
  pub use identity_did::revocation::*;
  pub use identity_did::service::*;
  pub use identity_did::utils::*;
  pub use identity_did::verification::*;

  pub use identity_did::did::*;

  pub use identity_did::verifiable;
}

pub mod iota {
  //! The IOTA DID method implementation for the IOTA ledger.

  pub use identity_iota_core::*;
}

// #[cfg(feature = "comm")]
// #[cfg_attr(docsrs, doc(cfg(feature = "comm")))]
// pub mod comm {
//   //! DID Communications Message Specification
//   //!
//   //! [Specification](https://github.com/iotaledger/identity.rs/tree/dev/docs/DID%20Communications%20Research%20and%20Specification)

//   pub use identity_comm::envelope::*;
//   pub use identity_comm::error::*;
//   pub use identity_comm::message::*;
// }

pub mod prelude {
  //! Prelude of commonly used types

  pub use identity_iota_core::IotaDID;
  pub use identity_iota_core::IotaDocument;

  #[cfg(feature = "iota-client")]
  #[cfg_attr(docsrs, doc(cfg(feature = "iota-client")))]
  pub use identity_iota_core::IotaClientExt;
  #[cfg(feature = "client")]
  #[cfg_attr(docsrs, doc(cfg(feature = "client")))]
  pub use identity_iota_core::IotaIdentityClient;
  #[cfg(feature = "client")]
  #[cfg_attr(docsrs, doc(cfg(feature = "client")))]
  pub use identity_iota_core::IotaIdentityClientExt;

  pub use identity_core::crypto::KeyPair;
  pub use identity_core::crypto::KeyType;

  #[cfg(feature = "resolver")]
  #[cfg_attr(docsrs, doc(cfg(feature = "resolver")))]
  pub use identity_resolver::Resolver;
}

#[cfg(feature = "resolver")]
#[cfg_attr(docsrs, doc(cfg(feature = "resolver")))]
pub mod resolver {
  //! DID resolution utilities

  pub use identity_resolver::*;
}

#[cfg(feature = "unstable-agent")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable-agent")))]
pub mod agent {
  //! Identity agent types

  pub use identity_agent::agent::*;
  pub use identity_agent::didcomm::*;
  pub use identity_agent::IdentityKeypair;
  pub use identity_agent::Multiaddr;
}
