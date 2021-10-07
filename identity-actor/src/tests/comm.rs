// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
  comm::handler::{presentation_holder_handler, presentation_verifier_handler, DidCommActor, DidCommHandler},
  errors::Result,
};

use super::{default_listening_actor, default_sending_actor};

#[tokio::test]
async fn test_didcomm_presentation_holder_initiates() -> Result<()> {
  pretty_env_logger::init();

  let mut holder_actor = default_sending_actor().await;

  let (mut verifier_actor, addr, peer_id) = default_listening_actor().await;

  let handler = DidCommHandler::new().await;

  verifier_actor.add_state(handler).add_handler(
    "didcomm/presentation_offer",
    DidCommHandler::presentation_verifier_actor_handler,
  )?;

  log::debug!("verifier peer id: {}", verifier_actor.peer_id());
  log::debug!("holder peer id: {}", holder_actor.peer_id());

  holder_actor.add_peer(peer_id, addr.clone()).await;

  let holder_didcomm_actor = DidCommActor::new(holder_actor.clone());

  holder_actor
    .add_state(holder_didcomm_actor.clone())
    .add_handler("didcomm/*", DidCommActor::catch_all_handler)?;

  presentation_holder_handler(holder_didcomm_actor, peer_id, None)
    .await
    .unwrap();

  verifier_actor.stop_handling_requests().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_didcomm_presentation_verifier_initiates() -> Result<()> {
  pretty_env_logger::init();

  let (mut holder_actor, addr, peer_id) = default_listening_actor().await;

  let mut verifier_actor = default_sending_actor().await;

  let handler = DidCommHandler::new().await;

  holder_actor.add_state(handler).add_handler(
    "didcomm/presentation_request",
    DidCommHandler::presentation_holder_actor_handler,
  )?;

  log::debug!("verifier peer id: {}", verifier_actor.peer_id());
  log::debug!("holder peer id: {}", holder_actor.peer_id());

  verifier_actor.add_peer(peer_id, addr.clone()).await;

  let verifier_didcomm_actor = DidCommActor::new(verifier_actor.clone());

  verifier_actor
    .add_state(verifier_didcomm_actor.clone())
    .add_handler("didcomm/*", DidCommActor::catch_all_handler)?;

  presentation_verifier_handler(verifier_didcomm_actor, peer_id, None)
    .await
    .unwrap();

  holder_actor.stop_handling_requests().await.unwrap();

  Ok(())
}
