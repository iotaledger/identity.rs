// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libp2p::PeerId;

use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;
use crate::ActorBuilder;

pub trait GenericActor
where
  Self: Sized + Clone + Send + Sync + 'static,
{
  fn from_actor_builder(builder: ActorBuilder, peer_id: PeerId, commander: NetCommander) -> crate::Result<Self>;
  // TODO: Take &self and move cloning inside the function.
  fn handle_request(self, request: InboundRequest);
}
