// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests;

pub mod communicator;
pub mod errors;
#[cfg(feature = "account")]
pub mod storage_handler;
pub mod types;

pub use communicator::Communicator;
pub use errors::{Error, Result};
pub use libp2p::{Multiaddr, PeerId};
pub use storage_handler::IdentityStorageHandler;
pub use types::{IdentityRequestHandler, IdentityStorageRequest, IdentityStorageResponse, NamedMessage};
