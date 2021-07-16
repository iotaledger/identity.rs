// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests;

pub mod actor;
pub mod actor_builder;
pub mod asyncfn;
pub mod comm;
pub mod errors;
pub mod storage;
pub mod traits;
pub mod types;

pub use actor::Actor;
pub use errors::{Error, Result};
pub use libp2p::{Multiaddr, PeerId};
#[cfg(feature = "account")]
pub use storage::handler::StorageHandler;
pub use storage::requests::{IdentityList, IdentityResolve};
pub use types::NamedMessage;

pub use communication_refactored::InitKeypair;
