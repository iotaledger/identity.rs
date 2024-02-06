// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Provides JWK and JWS types and functionality.  

// Re-export necessary types from `identity_jose`.

pub mod jwk {
  //! Reexport of [identity_jose::jwk].

  pub use identity_jose::jwk::*;
}

pub mod jws {
  //! Reexport of [identity_jose::jwk].

  pub use identity_jose::jws::*;
}

pub mod jwu {
  //! Reexport of [identity_jose::jwu].

  pub use identity_jose::jwu::*;
}

pub mod error {
  //! Reexport of [identity_jose::error].

  pub use identity_jose::error::*;
}
