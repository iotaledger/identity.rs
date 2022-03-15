// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests;

pub mod actor;
pub mod didcomm;
mod p2p;
#[cfg(feature = "account")]
pub mod remote_account;

pub use actor::*;

pub use libp2p::Multiaddr;
pub use libp2p::PeerId;

#[cfg(feature = "primitives")]
pub mod primitives {

  pub use crate::actor::actor_request::SyncMode;
  pub use crate::actor::traits::AnyFuture;
  pub use crate::actor::traits::RequestHandler;
  pub use crate::p2p::NetCommander;
  pub use crate::p2p::RequestMessage;
  pub use crate::p2p::ResponseMessage;
  pub use crate::traits::request_handler_clone_object;
  pub use crate::traits::request_handler_deserialize_request;
  pub use crate::traits::request_handler_serialize_response;
}
