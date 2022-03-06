// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests;

pub mod actor;
pub mod didcomm;
pub mod p2p;
#[cfg(feature = "account")]
pub mod remote_account;

pub use actor::*;

pub use libp2p::Multiaddr;
pub use libp2p::PeerId;
#[cfg(feature = "account")]
pub use remote_account::handler::RemoteAccount;
#[cfg(feature = "account")]
pub use remote_account::requests::IdentityGet;
#[cfg(feature = "account")]
pub use remote_account::requests::IdentityList;
