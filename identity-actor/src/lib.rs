// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests;

pub mod actor;
pub mod actor_builder;
pub mod errors;
#[cfg(feature = "account")]
pub mod storage_handler;
pub mod types;

pub use actor::Actor;
pub use errors::{Error, Result};
pub use libp2p::{Multiaddr, PeerId};
#[cfg(feature = "account")]
pub use storage_handler::IdentityStorageHandler;
pub use types::{RequestHandler, StorageRequest, StorageResponse, NamedMessage};
