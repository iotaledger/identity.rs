// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! # IOTA Identity
//! IOTA Identity is a [Rust](https://www.rust-lang.org/) implementation of decentralized identity, also known as Self Sovereign Identity (SSI), through the [W3C Decentralized Identifiers (DID)](https://w3c.github.io/did-core/) and [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/) standards alongside supporting methods, utilizing the [IOTA Distributed Ledger](https://www.iota.org).
//!
//!
//! # Example
//! ```
//! use identity::crypto::KeyPair;
//! use identity::iota::Client;
//! use identity::iota::Document;
//! use identity::iota::Network;
//! use identity::iota::Result;
//! use identity::iota::TangleRef;
//!
//! #[smol_potat::main]
//! async fn main() -> Result<()> {
//!   // Create a client to interact with the IOTA Tangle.
//!   let client: Client = Client::new()?;
//!
//!   // Create a DID Document (an identity).
//!   let keypair: KeyPair = KeyPair::new_ed25519()?;
//!   let mut document: Document = Document::from_keypair(&keypair)?;
//!
//!   // Sign the DID Document with the default authentication key.
//!   document.sign(keypair.secret())?;
//!
//!   // Use the client to publish the DID Document to the IOTA Tangle.
//!   document.publish(&client).await?;
//!
//!   // Print the DID Document transaction link.
//!   let network: Network = document.id().into();
//!   let explore: String = format!("{}/transaction/{}", network.explorer_url(), document.message_id());
//!
//!   println!("DID Document Transaction > {}", explore);
//!
//!   Ok(())
//! }
//! ```
//!
//! **Output**: Example DID Document in the [Tangle Explorer](https://explorer.iota.org/mainnet/transaction/LESUXJUMJCOWGHU9CQQUIHCIPYELOBMHZT9CHCYHJPO9BONQ9IQIFJSREYNOCTYCTQYBHBMBBWJJZ9999).
//!
//! # Documentation & Community Resources
//! - [identity.rs](https://github.com/iotaledger/identity.rs): Rust source code of this library on GitHub.
//! - [Identity Documentation Pages](https://identity.docs.iota.org/welcome.html): Supplementing documentation with
//!   simple examples on library usage to get you started.
//! - [More Examples](https://github.com/iotaledger/identity.rs/tree/dev/examples): Practical examples to get started
//!   with the library.
//! - [IOTA Identity Experience Team Website](https://iota-community.github.io/X-Team_IOTA_Identity/): Website of
//!   aforementioned team.
//!
//! # Structure (Temporary)
//!
//! - Resources
//!   - Docs Link (Website & User Guide)
//!   - X-Team
//! - Simple Example
//! - Architecture/Overview
//! - Get

#![warn(
  rust_2018_idioms,
  unreachable_pub,
  missing_docs,
  missing_crate_level_docs,
  broken_intra_doc_links,
  private_intra_doc_links,
  private_doc_tests,
  clippy::missing_safety_doc,
  clippy::missing_errors_doc
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

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
