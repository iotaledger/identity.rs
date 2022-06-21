// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::actor::AgentId;
use crate::actor::Endpoint;

/// A request paired with some context such as the sender's peer id.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct RequestContext<T> {
  /// The request type.
  pub input: T,
  /// The peer id of the sender.
  pub agent_id: AgentId,
  /// The [`Endpoint`] of this request.
  pub endpoint: Endpoint,
}

impl<T> RequestContext<T> {
  pub(crate) fn new(input: T, agent_id: AgentId, endpoint: Endpoint) -> Self {
    Self {
      input,
      agent_id,
      endpoint,
    }
  }

  /// Convert this context's inner type to another one.
  pub(crate) fn convert<I>(self, input: I) -> RequestContext<I> {
    RequestContext::new(input, self.agent_id, self.endpoint)
  }
}
