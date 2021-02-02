// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod core {
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
  pub use identity_core::crypto::*;
}

pub mod credential {
  pub use identity_credential::credential::*;
  pub use identity_credential::error::*;
  pub use identity_credential::presentation::*;
}

pub mod did {
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

pub mod iota {
  pub use identity_iota::chain::*;
  pub use identity_iota::client::*;
  pub use identity_iota::credential::*;
  pub use identity_iota::did::*;
  pub use identity_iota::error::*;
  pub use identity_iota::tangle::*;
  pub use identity_iota::utils::*;
}
