// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libp2p::Multiaddr;
use libp2p::PeerId;
use tokio::sync::Notify;

use crate::actor::AsyncActorRequest;
use crate::actor::Endpoint;
use crate::actor::Error;
use crate::actor::RequestContext;
use crate::actor::Result as ActorResult;
use crate::actor::SyncActor;
use crate::actor::SyncActorRequest;
use crate::didcomm::presentation_holder_handler;
use crate::didcomm::presentation_verifier_handler;
use crate::didcomm::AsyncActor;
use crate::didcomm::AsyncSystemBuilder;
use crate::didcomm::DidCommActor;
use crate::didcomm::DidCommPlaintextMessage;
use crate::didcomm::DidCommSystem;
use crate::didcomm::DidCommTermination;
use crate::didcomm::Presentation;
use crate::didcomm::PresentationOffer;
use crate::didcomm::PresentationRequest;
use crate::didcomm::ThreadId;
use crate::remote_account::IdentityList;
use crate::tests::default_identity;
use crate::tests::try_init_logger;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use super::default_listening_didcomm_actor;
use super::default_sending_didcomm_actor;

#[tokio::test]
async fn test_didcomm_approach() -> ActorResult<()> {
  try_init_logger();

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  pub struct AsyncDummy(u32);

  impl AsyncActorRequest for AsyncDummy {
    fn endpoint() -> Endpoint {
      "dummy/request".try_into().unwrap()
    }
  }

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  pub struct AsyncDummy2(u32);

  impl AsyncActorRequest for AsyncDummy2 {
    fn endpoint() -> Endpoint {
      "dummy/request_alt".try_into().unwrap()
    }
  }

  #[derive(Clone)]
  struct MyActor {
    counter: Arc<AtomicU32>,
    notify: Arc<Notify>,
  }

  #[async_trait::async_trait]
  impl AsyncActor<DidCommPlaintextMessage<AsyncDummy>> for MyActor {
    async fn handle(&self, mut actor: DidCommSystem, request: RequestContext<DidCommPlaintextMessage<AsyncDummy>>) {
      self
        .counter
        .fetch_add(request.input.body.0, std::sync::atomic::Ordering::SeqCst);

      actor
        .send_message(request.peer, request.input.thread_id(), AsyncDummy2(21))
        .await
        .unwrap();

      let message: DidCommPlaintextMessage<AsyncDummy2> = actor.await_message(request.input.thread_id()).await.unwrap();

      assert_eq!(message.body.0, 1337);

      self.notify.notify_one();
    }
  }

  let mut builder = AsyncSystemBuilder::new().identity(default_identity());

  let notify = Arc::new(Notify::new());
  let actor = MyActor {
    counter: Arc::new(AtomicU32::new(0)),
    notify: notify.clone(),
  };
  builder.attach(actor.clone());

  let mut listening_system: DidCommSystem = builder.build().await.unwrap();

  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
  let _ = listening_system.start_listening(addr).await.unwrap();
  let addrs = listening_system.addresses().await.unwrap();

  let peer_id = listening_system.peer_id();

  let mut sending_system = AsyncSystemBuilder::new()
    .identity(default_identity())
    .build()
    .await
    .unwrap();

  sending_system.add_addresses(peer_id, addrs).await.unwrap();

  let thread_id = ThreadId::new();

  sending_system
    .send_message(peer_id, &thread_id, AsyncDummy(42))
    .await
    .unwrap();

  let message: DidCommPlaintextMessage<AsyncDummy2> = sending_system.await_message(&thread_id).await.unwrap();

  assert_eq!(message.body.0, 21);

  sending_system
    .send_message(peer_id, &thread_id, AsyncDummy2(1337))
    .await
    .unwrap();

  notify.notified().await;

  assert_eq!(actor.counter.load(std::sync::atomic::Ordering::SeqCst), 42);

  listening_system.shutdown().await.unwrap();
  sending_system.shutdown().await.unwrap();

  Ok(())
}

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

  struct TestActor;

  #[async_trait::async_trait]
  impl SyncActor<SyncDummy> for TestActor {
    async fn handle(&self, request: RequestContext<SyncDummy>) -> u16 {
      request.input.0
    }
  }

  let (listening_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder.attach_sync(TestActor);
    builder
  })
  .await;

  let mut sending_actor = default_sending_didcomm_actor(|builder| builder).await;
  sending_actor.add_addresses(peer_id, addrs).await.unwrap();

  let result = sending_actor.send_request(peer_id, SyncDummy(42)).await;

  assert_eq!(result.unwrap(), 42);

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
  let actor: DidCommActor = DidCommActor::new().await;

  let mut holder_system: DidCommSystem = default_sending_didcomm_actor(|builder| builder).await;

  let (verifier_system, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder.attach::<DidCommPlaintextMessage<PresentationOffer>, _>(actor.clone());
    builder
  })
  .await;

  holder_system.add_addresses(peer_id, addrs).await.unwrap();

  presentation_holder_handler(holder_system.clone(), peer_id, None)
    .await
    .unwrap();

  verifier_system.shutdown().await.unwrap();
  holder_system.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_didcomm_presentation_verifier_initiates() -> ActorResult<()> {
  try_init_logger();

  let actor = DidCommActor::new().await;

  let (holder_system, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder.attach::<DidCommPlaintextMessage<PresentationRequest>, _>(actor.clone());
    builder
  })
  .await;
  let mut verifier_system = default_sending_didcomm_actor(|builder| builder).await;

  verifier_system.add_addresses(peer_id, addrs).await.unwrap();

  presentation_verifier_handler(verifier_system.clone(), peer_id, None)
    .await
    .unwrap();

  holder_system.shutdown().await.unwrap();
  verifier_system.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_didcomm_presentation_verifier_initiates_with_send_message_hook() -> ActorResult<()> {
  try_init_logger();

  let actor = DidCommActor::new().await;

  let (holder_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder.attach::<DidCommPlaintextMessage<PresentationRequest>, _>(actor.clone());
    builder
  })
  .await;

  let function_state = TestFunctionState::new();

  async fn presentation_request_hook(
    state: TestFunctionState,
    _: DidCommSystem,
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

  let actor = DidCommActor::new().await;

  let function_state = TestFunctionState::new();

  async fn receive_presentation_hook(
    state: TestFunctionState,
    _: DidCommSystem,
    req: RequestContext<DidCommPlaintextMessage<Presentation>>,
  ) -> Result<DidCommPlaintextMessage<Presentation>, DidCommTermination> {
    state.was_called.store(true, Ordering::SeqCst);
    Ok(req.input)
  }

  let mut holder_actor = default_sending_didcomm_actor(|builder| builder).await;

  let (verifier_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder.attach::<DidCommPlaintextMessage<PresentationOffer>, _>(actor.clone());

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

  #[derive(Clone)]
  struct MyActor;

  #[async_trait::async_trait]
  impl AsyncActor<DidCommPlaintextMessage<PresentationOffer>> for MyActor {
    async fn handle(&self, _: DidCommSystem, _: RequestContext<DidCommPlaintextMessage<PresentationOffer>>) {}
  }

  let (listening_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder.attach(MyActor);
    builder
  })
  .await;

  let mut sending_actor: DidCommSystem =
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
  struct TestActor {
    was_called: Arc<AtomicBool>,
  }

  impl TestActor {
    fn new() -> Self {
      Self {
        was_called: Arc::new(AtomicBool::new(false)),
      }
    }
  }

  #[async_trait::async_trait]
  impl AsyncActor<DidCommPlaintextMessage<PresentationOffer>> for TestActor {
    async fn handle(&self, _: DidCommSystem, _: RequestContext<DidCommPlaintextMessage<PresentationOffer>>) {
      tokio::time::sleep(std::time::Duration::from_millis(25)).await;
      self.was_called.store(true, std::sync::atomic::Ordering::SeqCst);
    }
  }

  let test_actor = TestActor::new();

  let (listening_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    builder.attach(test_actor.clone());
    builder
  })
  .await;

  let mut sending_actor: DidCommSystem = default_sending_didcomm_actor(|builder| builder).await;
  sending_actor.add_addresses(peer_id, addrs).await.unwrap();

  sending_actor
    .send_message(peer_id, &ThreadId::new(), PresentationOffer::default())
    .await
    .unwrap();

  listening_actor.shutdown().await.unwrap();

  tokio::time::sleep(std::time::Duration::from_millis(50)).await;

  sending_actor.shutdown().await.unwrap();

  assert!(test_actor.was_called.load(std::sync::atomic::Ordering::SeqCst));

  Ok(())
}
