// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libp2p::PeerId;

use crate::actor::Actor;
use crate::actor::AsyncActorRequest;
use crate::actor::Endpoint;
use crate::actor::Error;
use crate::actor::RequestContext;
use crate::actor::Result as ActorResult;
use crate::actor::SyncActorRequest;
use crate::didcomm::presentation_holder_handler;
use crate::didcomm::presentation_verifier_handler;
use crate::didcomm::DidCommActor;
use crate::didcomm::DidCommPlaintextMessage;
use crate::didcomm::DidCommState;
use crate::didcomm::DidCommTermination;
use crate::didcomm::Presentation;
use crate::didcomm::PresentationOffer;
use crate::didcomm::PresentationRequest;
use crate::didcomm::ThreadId;
use crate::remote_account::IdentityList;
use crate::tests::try_init_logger;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use super::default_listening_didcomm_actor;
use super::default_sending_didcomm_actor;

#[tokio::test]
async fn test_didcomm_actor_supports_sync_requests() -> ActorResult<()> {
  try_init_logger();

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  pub struct SyncDummy(u16);

  impl SyncActorRequest for SyncDummy {
    type Response = u16;

    fn endpoint() -> Endpoint {
      "test/request".try_into().unwrap()
    }
  }

  let (listening_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder
      .add_state(())
      .add_sync_handler(|_: (), _: Actor, request: RequestContext<SyncDummy>| async move { request.input.0 });

    builder
  })
  .await;

  let mut sending_actor = default_sending_didcomm_actor(|builder| builder).await;
  sending_actor.add_addresses(peer_id, addrs).await.unwrap();

  let result = sending_actor.send_request(peer_id, SyncDummy(42)).await;

  assert!(result.is_ok());

  listening_actor.shutdown().await.unwrap();
  sending_actor.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_unknown_thread_returns_error() -> ActorResult<()> {
  try_init_logger();

  let (listening_actor, addrs, peer_id) = default_listening_didcomm_actor(|builder| builder).await;

  let mut sending_actor = default_sending_didcomm_actor(|builder| builder).await;
  sending_actor.add_addresses(peer_id, addrs).await.unwrap();

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  pub struct AsyncDummy(u16);

  impl AsyncActorRequest for AsyncDummy {
    fn endpoint() -> Endpoint {
      "unknown/thread".try_into().unwrap()
    }
  }

  let result = sending_actor
    .send_message(peer_id, &ThreadId::new(), AsyncDummy(42))
    .await;

  assert!(matches!(result.unwrap_err(), Error::UnexpectedRequest(_)));

  listening_actor.shutdown().await.unwrap();
  sending_actor.shutdown().await.unwrap();

  Ok(())
}

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
async fn test_didcomm_presentation_holder_initiates() -> ActorResult<()> {
  try_init_logger();
  let handler = DidCommState::new().await;

  let mut holder_actor = default_sending_didcomm_actor(|builder| builder).await;

  let (verifier_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder
      .add_state(handler)
      .add_async_handler(DidCommState::presentation_verifier_actor_handler);
    builder
  })
  .await;

  holder_actor.add_addresses(peer_id, addrs).await.unwrap();

  presentation_holder_handler(holder_actor.clone(), peer_id, None)
    .await
    .unwrap();

  verifier_actor.shutdown().await.unwrap();
  holder_actor.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_didcomm_presentation_verifier_initiates() -> ActorResult<()> {
  try_init_logger();

  let handler = DidCommState::new().await;

  let (holder_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder
      .add_state(handler)
      .add_async_handler(DidCommState::presentation_holder_actor_handler);
    builder
  })
  .await;
  let mut verifier_actor = default_sending_didcomm_actor(|builder| builder).await;

  verifier_actor.add_addresses(peer_id, addrs).await.unwrap();

  presentation_verifier_handler(verifier_actor.clone(), peer_id, None)
    .await
    .unwrap();

  holder_actor.shutdown().await.unwrap();
  verifier_actor.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_didcomm_presentation_verifier_initiates_with_send_message_hook() -> ActorResult<()> {
  try_init_logger();

  let handler = DidCommState::new().await;

  let (holder_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder
      .add_state(handler)
      .add_async_handler(DidCommState::presentation_holder_actor_handler);
    builder
  })
  .await;

  let function_state = TestFunctionState::new();

  async fn presentation_request_hook(
    state: TestFunctionState,
    _: DidCommActor,
    request: RequestContext<DidCommPlaintextMessage<PresentationRequest>>,
  ) -> Result<DidCommPlaintextMessage<PresentationRequest>, DidCommTermination> {
    state.was_called.store(true, Ordering::SeqCst);
    Ok(request.input)
  }

  let mut verifier_actor = default_sending_didcomm_actor(|mut builder| {
    builder
      .add_state(function_state.clone())
      .add_hook(presentation_request_hook);
    builder
  })
  .await;

  verifier_actor.add_addresses(peer_id, addrs).await.unwrap();

  presentation_verifier_handler(verifier_actor.clone(), peer_id, None)
    .await
    .unwrap();

  verifier_actor.shutdown().await.unwrap();
  holder_actor.shutdown().await.unwrap();

  assert!(function_state.was_called.load(Ordering::SeqCst));

  Ok(())
}

#[tokio::test]
async fn test_didcomm_presentation_holder_initiates_with_await_message_hook() -> ActorResult<()> {
  try_init_logger();

  let handler = DidCommState::new().await;

  let function_state = TestFunctionState::new();

  async fn receive_presentation_hook(
    state: TestFunctionState,
    _: DidCommActor,
    req: RequestContext<DidCommPlaintextMessage<Presentation>>,
  ) -> Result<DidCommPlaintextMessage<Presentation>, DidCommTermination> {
    state.was_called.store(true, Ordering::SeqCst);
    Ok(req.input)
  }

  let mut holder_actor = default_sending_didcomm_actor(|builder| builder).await;

  let (verifier_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder
      .add_state(handler)
      .add_async_handler(DidCommState::presentation_verifier_actor_handler);

    builder
      .add_state(function_state.clone())
      .add_hook(receive_presentation_hook);

    builder
  })
  .await;

  holder_actor.add_addresses(peer_id, addrs).await.unwrap();

  presentation_holder_handler(holder_actor.clone(), peer_id, None)
    .await
    .unwrap();

  verifier_actor.shutdown().await.unwrap();
  holder_actor.shutdown().await.unwrap();

  assert!(function_state.was_called.load(Ordering::SeqCst));

  Ok(())
}

#[tokio::test]
async fn test_sending_to_unconnected_peer_returns_error() -> ActorResult<()> {
  try_init_logger();

  let mut sending_actor = default_sending_didcomm_actor(|builder| builder).await;

  let result = sending_actor.send_request(PeerId::random(), IdentityList).await;

  assert!(matches!(result.unwrap_err(), Error::OutboundFailure(_)));

  let result = sending_actor
    .send_message(PeerId::random(), &ThreadId::new(), PresentationOffer::default())
    .await;

  assert!(matches!(result.unwrap_err(), Error::OutboundFailure(_)));

  sending_actor.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_await_message_returns_timeout_error() -> ActorResult<()> {
  try_init_logger();

  let (listening_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder.add_state(()).add_async_handler(
      |_: (), _: DidCommActor, _: RequestContext<DidCommPlaintextMessage<PresentationOffer>>| async move {},
    );

    builder
  })
  .await;

  let mut sending_actor: DidCommActor =
    default_sending_didcomm_actor(|builder| builder.timeout(std::time::Duration::from_millis(50))).await;

  sending_actor.add_addresses(peer_id, addrs).await.unwrap();

  let thread_id = ThreadId::new();
  sending_actor
    .send_message(peer_id, &thread_id, PresentationOffer::default())
    .await
    .unwrap();

  let result = sending_actor.await_message::<()>(&thread_id).await;

  assert!(matches!(result.unwrap_err(), Error::AwaitTimeout(_)));

  listening_actor.shutdown().await.unwrap();
  sending_actor.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_handler_finishes_execution_after_shutdown() -> ActorResult<()> {
  try_init_logger();

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

  let state = TestFunctionState::new();

  let (listening_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder.add_state(state.clone()).add_async_handler(
      |state: TestFunctionState,
       _: DidCommActor,
       _message: RequestContext<DidCommPlaintextMessage<PresentationOffer>>| async move {
        tokio::time::sleep(std::time::Duration::from_millis(25)).await;
        state.was_called.store(true, std::sync::atomic::Ordering::SeqCst);
      },
    );

    builder
  })
  .await;

  let mut sending_actor: DidCommActor = default_sending_didcomm_actor(|builder| builder).await;
  sending_actor.add_addresses(peer_id, addrs).await.unwrap();

  sending_actor
    .send_message(peer_id, &ThreadId::new(), PresentationOffer::default())
    .await
    .unwrap();

  listening_actor.shutdown().await.unwrap();

  tokio::time::sleep(std::time::Duration::from_millis(50)).await;

  sending_actor.shutdown().await.unwrap();

  assert!(state.was_called.load(std::sync::atomic::Ordering::SeqCst));

  Ok(())
}
