// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "v2")]
#[path = ""]
mod v2 {
  mod error;
  mod resolver;

  pub use error::Error;
  pub use error::Result;
  pub use resolver::Resolver;
}

#[cfg(all(feature = "iota", feature = "v2"))]
mod iota;

#[cfg(not(feature = "v2"))]
mod legacy;
#[cfg(not(feature = "v2"))]
pub use legacy::*;

#[cfg(feature = "v2")]
pub use v2::*;
