// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::didcomm::actor::DidCommActor;
use crate::didcomm::actor::DidCommMessages;
use crate::didcomm::actor::DidCommTermination;
use crate::didcomm::presentation::presentation_holder_handler;
use crate::didcomm::presentation::presentation_verifier_handler;
use crate::didcomm::presentation::DidCommHandler;
use crate::didcomm::presentation::PresentationOffer;
use crate::didcomm::presentation::PresentationRequest;
use crate::Actor;
use crate::RequestContext;
use crate::Result;
use std::result::Result as StdResult;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use super::default_listening_actor;
use super::default_sending_actor;

#[derive(Clone)]
struct TestFunctionState {
  was_called: Arc<AtomicBool>,
}

impl TestFunctionState {
  fn new() -> Self {
    Self {
      was_called: Arc::new(AtomicBool::new(false)),
    }
  }
}

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
    .add_state(holder_didcomm_actor.messages.clone())
    .add_handler("didcomm/*", DidCommMessages::catch_all_handler)
    .unwrap();

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
    .add_state(verifier_didcomm_actor.messages.clone())
    .add_handler("didcomm/*", DidCommMessages::catch_all_handler)
    .unwrap();

  presentation_verifier_handler(verifier_didcomm_actor, peer_id, None)
    .await
    .unwrap();

  holder_actor.stop_handling_requests().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_didcomm_presentation_verifier_initiates_with_implicit_hooks() -> Result<()> {
  pretty_env_logger::init();

  let (mut holder_actor, addr, peer_id) = default_listening_actor().await;

  let mut verifier_actor = default_sending_actor().await;

  let handler = DidCommHandler::new().await;

  holder_actor.add_state(handler).add_handler(
    "didcomm/presentation_request",
    DidCommHandler::presentation_holder_actor_handler,
  )?;

  let function_state = TestFunctionState::new();

  async fn presentation_request_hook(
    state: TestFunctionState,
    _: Actor,
    req: RequestContext<PresentationRequest>,
  ) -> StdResult<PresentationRequest, DidCommTermination> {
    state.was_called.store(true, Ordering::SeqCst);
    Ok(req.input)
  }

  verifier_actor
    .add_state(function_state.clone())
    .add_hook("didcomm/presentation_request/hook", presentation_request_hook)
    .unwrap();

  log::debug!("verifier peer id: {}", verifier_actor.peer_id());
  log::debug!("holder peer id: {}", holder_actor.peer_id());

  verifier_actor.add_peer(peer_id, addr.clone()).await;

  let verifier_didcomm_actor = DidCommActor::new(verifier_actor.clone());

  verifier_actor
    .add_state(verifier_didcomm_actor.messages.clone())
    .add_handler("didcomm/*", DidCommMessages::catch_all_handler)
    .unwrap();

  presentation_verifier_handler(verifier_didcomm_actor, peer_id, None)
    .await
    .unwrap();

  holder_actor.stop_handling_requests().await.unwrap();

  assert!(function_state.was_called.load(Ordering::SeqCst));

  Ok(())
}

#[tokio::test]
async fn test_didcomm_hook_invocation_with_incorrect_type_fails() -> Result<()> {
  pretty_env_logger::init();

  let mut verifier_actor = default_sending_actor().await;

  // a hook that has the wrong type: offer instead of request
  async fn presentation_request_hook(
    _: (),
    _: Actor,
    req: RequestContext<PresentationOffer>,
  ) -> StdResult<PresentationOffer, DidCommTermination> {
    Ok(req.input)
  }

  verifier_actor
    .add_state(())
    .add_hook("didcomm/presentation_request/hook", presentation_request_hook)
    .unwrap();

  let peer_id = verifier_actor.peer_id();
  let mut verifier_didcomm_actor = DidCommActor::new(verifier_actor);

  let result = verifier_didcomm_actor
    .send_request(peer_id, PresentationRequest::default())
    .await;

  assert!(matches!(result.unwrap_err(), crate::Error::HookInvocationError(_)));

  Ok(())
}
