// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod document;
mod properties;
mod traits;

pub use self::document::Public;
pub use self::document::Secret;
pub use self::properties::Properties;
pub use self::traits::Revocation;

#[cfg(test)]
mod tests;
