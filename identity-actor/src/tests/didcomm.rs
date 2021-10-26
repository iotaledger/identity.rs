// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::didcomm::actor::DidCommActor;
use crate::didcomm::actor::DidCommMessages;
use crate::didcomm::actor::DidCommTermination;
use crate::didcomm::presentation::presentation_holder_handler;
use crate::didcomm::presentation::presentation_verifier_handler;
use crate::didcomm::presentation::DidCommHandler;
use crate::didcomm::presentation::Presentation;
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
  let mut holder_actor = default_sending_actor().await;

  let (mut verifier_actor, addr, peer_id) = default_listening_actor().await;

  let handler = DidCommHandler::new().await;

  verifier_actor.add_state(handler).add_handler(
    "didcomm/presentation_offer",
    DidCommHandler::presentation_verifier_actor_handler,
  )?;

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
  holder_actor.stop_handling_requests().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_didcomm_presentation_verifier_initiates() -> Result<()> {
  let (mut holder_actor, addr, peer_id) = default_listening_actor().await;

  let mut verifier_actor = default_sending_actor().await;

  let handler = DidCommHandler::new().await;

  holder_actor.add_state(handler).add_handler(
    "didcomm/presentation_request",
    DidCommHandler::presentation_holder_actor_handler,
  )?;

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
  verifier_actor.stop_handling_requests().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_didcomm_presentation_verifier_initiates_with_implicit_hooks() -> Result<()> {
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

  verifier_actor.add_peer(peer_id, addr.clone()).await;

  let verifier_didcomm_actor = DidCommActor::new(verifier_actor.clone());

  verifier_actor
    .add_state(verifier_didcomm_actor.messages.clone())
    .add_handler("didcomm/*", DidCommMessages::catch_all_handler)
    .unwrap();

  presentation_verifier_handler(verifier_didcomm_actor, peer_id, None)
    .await
    .unwrap();

  verifier_actor.stop_handling_requests().await.unwrap();
  holder_actor.stop_handling_requests().await.unwrap();

  assert!(function_state.was_called.load(Ordering::SeqCst));

  Ok(())
}

#[tokio::test]
async fn test_didcomm_presentation_holder_initiates_with_implicit_hooks() -> Result<()> {
  let mut holder_actor = default_sending_actor().await;

  let (mut verifier_actor, addr, peer_id) = default_listening_actor().await;

  let handler = DidCommHandler::new().await;

  verifier_actor.add_state(handler).add_handler(
    "didcomm/presentation_offer",
    DidCommHandler::presentation_verifier_actor_handler,
  )?;

  let function_state = TestFunctionState::new();

  async fn receive_presentation_hook(
    state: TestFunctionState,
    _: Actor,
    req: RequestContext<Presentation>,
  ) -> StdResult<Presentation, DidCommTermination> {
    state.was_called.store(true, Ordering::SeqCst);
    Ok(req.input)
  }

  verifier_actor
    .add_state(function_state.clone())
    .add_hook("didcomm/presentation/hook", receive_presentation_hook)
    .unwrap();

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
  holder_actor.stop_handling_requests().await.unwrap();

  assert!(function_state.was_called.load(Ordering::SeqCst));

  Ok(())
}

#[tokio::test]
async fn test_didcomm_send_hook_invocation_with_incorrect_type_fails() -> Result<()> {
  let mut verifier_actor = default_sending_actor().await;

  async fn presentation_request_hook(
    _: (),
    _: Actor,
    req: RequestContext<PresentationOffer>,
  ) -> StdResult<PresentationOffer, DidCommTermination> {
    Ok(req.input)
  }

  // Register a hook that has the wrong type: PresentationOffer instead of PresentationRequest
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

#[tokio::test]
async fn test_didcomm_await_hook_invocation_with_incorrect_type_fails() -> Result<()> {
  let mut holder_actor = default_sending_actor().await;

  let (mut verifier_actor, addr, peer_id) = default_listening_actor().await;

  let mut holder_didcomm_actor = DidCommActor::new(holder_actor.clone());
  let verifier_didcomm_actor = DidCommActor::new(verifier_actor.clone());

  verifier_actor
    .add_state(verifier_didcomm_actor.messages.clone())
    .add_handler("didcomm/*", DidCommMessages::catch_all_handler)
    .unwrap();

  async fn presentation_request_hook(
    _: (),
    _: Actor,
    req: RequestContext<PresentationRequest>,
  ) -> StdResult<PresentationRequest, DidCommTermination> {
    Ok(req.input)
  }

  // Register a hook that has the wrong type: PresentationRequest instead of PresentationOffer
  verifier_actor
    .add_state(())
    .add_hook("didcomm/presentation_offer/hook", presentation_request_hook)
    .unwrap();

  let verifier_peer_id = verifier_actor.peer_id();
  let holder_peer_id = holder_actor.peer_id();

  holder_actor.add_peer(verifier_peer_id, addr.clone()).await;

  let task = tokio::spawn(async move {
    let message: crate::Result<PresentationOffer> = verifier_didcomm_actor.await_message(holder_peer_id).await;

    assert!(matches!(message.unwrap_err(), crate::Error::HookInvocationError(_)));
  });

  holder_didcomm_actor
    .send_request(peer_id, PresentationOffer::default())
    .await?;

  task.await.unwrap();

  verifier_actor.stop_handling_requests().await.unwrap();
  holder_actor.stop_handling_requests().await.unwrap();

  Ok(())
}
