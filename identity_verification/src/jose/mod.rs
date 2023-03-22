// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Provides JWK and JWS types and functionality.  

// Re-export necessary types from `identity_jose`.
pub mod jwk {
  pub use identity_jose::jwk::*;
}

pub mod jws {
  pub use identity_jose::jws::*;
}

pub mod error {
  pub use identity_jose::error::*;
}
