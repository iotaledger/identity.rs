// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod p2p;
#[cfg(test)]
mod tests;

pub mod actor;
pub mod didcomm;

pub use libp2p::Multiaddr;
pub use libp2p::PeerId;

#[cfg(feature = "primitives")]
pub mod primitives {
  pub use crate::p2p::NetCommander;
  pub use crate::p2p::RequestMessage;
  pub use crate::p2p::ResponseMessage;
}
