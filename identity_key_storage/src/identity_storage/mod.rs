// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod error;
#[cfg(test)]
mod identity_memstore;
#[allow(clippy::module_inception)]
mod identity_storage;

#[cfg(test)]
pub use identity_memstore::*;
pub use identity_storage::*;
