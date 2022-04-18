// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libp2p::PeerId;

use crate::didcomm::didcomm_actor::DidCommActor;
use crate::didcomm::message::DidCommPlaintextMessage;
use crate::didcomm::presentation::presentation_holder_handler;
use crate::didcomm::presentation::presentation_verifier_handler;
use crate::didcomm::presentation::DidCommState;
use crate::didcomm::presentation::Presentation;
use crate::didcomm::presentation::PresentationOffer;
use crate::didcomm::presentation::PresentationRequest;
use crate::didcomm::termination::DidCommTermination;
use crate::didcomm::thread_id::ThreadId;
use crate::remote_account::IdentityList;
use crate::tests::try_init_logger;
use crate::ActorRequest;
use crate::Asynchronous;
use crate::Error;
use crate::RequestContext;
use crate::Result;
use std::result::Result as StdResult;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use super::default_listening_didcomm_actor;
use super::default_sending_didcomm_actor;

#[tokio::test]
async fn test_unknown_thread_returns_error() -> crate::Result<()> {
  try_init_logger();

  let (listening_actor, addrs, peer_id) = default_listening_didcomm_actor(|builder| builder).await;

  let mut sending_actor = default_sending_didcomm_actor(|builder| builder).await;
  sending_actor.add_addresses(peer_id, addrs).await.unwrap();

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  pub struct AsyncDummy(u16);

  impl ActorRequest<Asynchronous> for AsyncDummy {
    type Response = ();

    fn endpoint() -> &'static str {
      "unknown/thread"
    }
  }

  let result = sending_actor
    .send_named_message(peer_id, "unknown/thread", &ThreadId::new(), AsyncDummy(42))
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
async fn test_didcomm_presentation_holder_initiates() -> Result<()> {
  try_init_logger();
  let handler = DidCommState::new().await;

  let mut holder_actor = default_sending_didcomm_actor(|builder| builder).await;

  let (verifier_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder
      .add_state(handler)
      .add_async_handler(DidCommState::presentation_verifier_actor_handler)
      .unwrap();
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
async fn test_didcomm_presentation_verifier_initiates() -> Result<()> {
  try_init_logger();

  let handler = DidCommState::new().await;

  let (holder_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder
      .add_state(handler)
      .add_async_handler(DidCommState::presentation_holder_actor_handler)
      .unwrap();
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
async fn test_didcomm_presentation_verifier_initiates_with_send_message_hook() -> Result<()> {
  try_init_logger();

  let handler = DidCommState::new().await;

  let (holder_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder
      .add_state(handler)
      .add_async_handler(DidCommState::presentation_holder_actor_handler)
      .unwrap();
    builder
  })
  .await;

  let function_state = TestFunctionState::new();

  async fn presentation_request_hook(
    state: TestFunctionState,
    _: DidCommActor,
    request: RequestContext<DidCommPlaintextMessage<PresentationRequest>>,
  ) -> StdResult<DidCommPlaintextMessage<PresentationRequest>, DidCommTermination> {
    state.was_called.store(true, Ordering::SeqCst);
    Ok(request.input)
  }

  let mut verifier_actor = default_sending_didcomm_actor(|mut builder| {
    builder
      .add_state(function_state.clone())
      .add_hook(presentation_request_hook)
      .unwrap();
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
async fn test_didcomm_presentation_holder_initiates_with_await_message_hook() -> Result<()> {
  try_init_logger();

  let handler = DidCommState::new().await;

  let function_state = TestFunctionState::new();

  async fn receive_presentation_hook(
    state: TestFunctionState,
    _: DidCommActor,
    req: RequestContext<DidCommPlaintextMessage<Presentation>>,
  ) -> StdResult<DidCommPlaintextMessage<Presentation>, DidCommTermination> {
    state.was_called.store(true, Ordering::SeqCst);
    Ok(req.input)
  }

  let mut holder_actor = default_sending_didcomm_actor(|builder| builder).await;

  let (verifier_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder
      .add_state(handler)
      .add_async_handler(DidCommState::presentation_verifier_actor_handler)
      .unwrap();

    builder
      .add_state(function_state.clone())
      .add_hook(receive_presentation_hook)
      .unwrap();

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
async fn test_sending_to_unconnected_peer_returns_error() -> crate::Result<()> {
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
async fn test_await_message_returns_timeout_error() -> crate::Result<()> {
  try_init_logger();

  let (listening_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder
      .add_state(())
      .add_async_handler(
        |_: (), _: DidCommActor, _: RequestContext<DidCommPlaintextMessage<PresentationOffer>>| async move {},
      )
      .unwrap();

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
async fn test_handler_finishes_execution_after_shutdown() -> crate::Result<()> {
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
    builder
      .add_state(state.clone())
      .add_async_handler(
        |state: TestFunctionState,
         _: DidCommActor,
         _message: RequestContext<DidCommPlaintextMessage<PresentationOffer>>| async move {
          tokio::time::sleep(std::time::Duration::from_millis(25)).await;
          state.was_called.store(true, std::sync::atomic::Ordering::SeqCst);
        },
      )
      .unwrap();

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
