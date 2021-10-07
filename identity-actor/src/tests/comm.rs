// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libp2p::{Multiaddr, PeerId};

use crate::{
  comm::handler::{presentation_holder_handler, DidCommActor, DidCommHandler},
  errors::Result,
  Actor,
};

use super::{default_listening_actor, default_sending_actor};

async fn default_comm_listening_actor() -> Result<(Actor, Multiaddr, PeerId)> {
  let (mut listening_actor, addr, peer_id) = default_listening_actor().await;

  let handler = DidCommHandler::new().await;

  listening_actor.add_handler(handler).add_method(
    "didcomm/presentation_offer",
    DidCommHandler::presentation_verifier_actor_handler,
  )?;

  Ok((listening_actor, addr, peer_id))
}

#[tokio::test]
async fn test_didcomm() -> Result<()> {
  pretty_env_logger::init();

  let mut sending_actor = default_sending_actor().await;

  let (listening_actor, addr, peer_id) = default_comm_listening_actor().await?;

  log::info!("verifier peer id: {}", listening_actor.peer_id());
  log::info!("holder peer id: {}", sending_actor.peer_id());

  sending_actor.add_peer(peer_id, addr.clone()).await;

  let sending_didcomm_actor = DidCommActor::new(sending_actor.clone());

  sending_actor
    .add_handler(sending_didcomm_actor.clone())
    .add_method("didcomm/*", DidCommActor::catch_all_handler)?;

  presentation_holder_handler(sending_didcomm_actor, peer_id, None)
    .await
    .unwrap();

  listening_actor.stop_handling_requests().await.unwrap();

  Ok(())
}
