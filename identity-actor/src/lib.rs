// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests;

pub mod actor;
pub mod actor_builder;
pub mod asyncfn;
pub mod comm;
pub mod endpoint;
pub mod errors;
pub mod storage;
pub mod traits;
pub mod types;

pub use actor::Actor;
pub use errors::Error;
pub use errors::Result;
pub use libp2p::Multiaddr;
pub use libp2p::PeerId;
#[cfg(feature = "account")]
pub use storage::handler::StorageHandler;
pub use storage::requests::IdentityList;
pub use storage::requests::IdentityResolve;
pub use types::RequestMessage;

pub use p2p::InitKeypair;
