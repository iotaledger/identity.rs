// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libp2p::PeerId;

use crate::actor::ActorConfig;
use crate::actor::ActorMap;

/// The internal state of a `System`.
#[derive(Debug)]
pub(crate) struct SystemState {
  pub(crate) peer_id: PeerId,
  pub(crate) config: ActorConfig,
  pub(crate) actors: ActorMap,
}
