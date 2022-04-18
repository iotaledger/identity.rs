// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libp2p::PeerId;

use crate::actor::ActorConfig;
use crate::actor::ActorStateExtension;
use crate::actor::Result as ActorResult;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;

use super::actor::HandlerMap;
use super::actor::ObjectMap;

pub trait GenericActor
where
  Self: Sized + Clone + Send + Sync + 'static,
{
  type Extension: ActorStateExtension;

  fn from_actor_builder(
    handlers: HandlerMap,
    objects: ObjectMap,
    config: ActorConfig,
    peer_id: PeerId,
    commander: NetCommander,
    extension: Self::Extension,
  ) -> ActorResult<Self>;
  // TODO: Take &self and move cloning inside the function.
  fn handle_request(self, request: InboundRequest);
}
