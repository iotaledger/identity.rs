// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libp2p::PeerId;

use crate::actor::Endpoint;

/// A request paired with some context such as the sender's peer id.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct RequestContext<T> {
  /// The request type.
  pub input: T,
  /// The peer id of the sender.
  pub peer_id: PeerId,
  /// The [`Endpoint`] of this request.
  pub endpoint: Endpoint,
}

impl<T> RequestContext<T> {
  pub(crate) fn new(input: T, peer_id: PeerId, endpoint: Endpoint) -> Self {
    Self {
      input,
      peer_id,
      endpoint,
    }
  }

  /// Convert this context's inner type to another one.
  pub(crate) fn convert<I>(self, input: I) -> RequestContext<I> {
    RequestContext::new(input, self.peer_id, self.endpoint)
  }
}
