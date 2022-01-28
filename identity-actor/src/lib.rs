// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests;

pub mod actor;
pub mod didcomm;
pub mod p2p;
pub mod storage;

pub use actor::*;

pub use libp2p::Multiaddr;
pub use libp2p::PeerId;
#[cfg(feature = "account")]
pub use storage::handler::StorageHandler;
pub use storage::requests::IdentityList;
pub use storage::requests::IdentityResolve;
