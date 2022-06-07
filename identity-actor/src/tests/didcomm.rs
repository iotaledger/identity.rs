// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use libp2p::Multiaddr;
use libp2p::PeerId;

use crate::actor::Actor;
use crate::actor::ActorRequest;
use crate::actor::Endpoint;
use crate::actor::Error;
use crate::actor::RequestContext;
use crate::actor::Result as ActorResult;
use crate::didcomm::DidCommActor;
use crate::didcomm::DidCommPlaintextMessage;
use crate::didcomm::DidCommRequest;
use crate::didcomm::DidCommSystem;
use crate::didcomm::DidCommSystemBuilder;
use crate::didcomm::ThreadId;
use crate::tests::default_identity;
use crate::tests::default_listening_didcomm_system;
use crate::tests::default_sending_didcomm_system;
use crate::tests::presentation::presentation_holder_handler;
use crate::tests::presentation::presentation_verifier_handler;
use crate::tests::presentation::DidCommState;
use crate::tests::presentation::PresentationOffer;
use crate::tests::presentation::PresentationRequest;
use crate::tests::remote_account::IdentityList;
use crate::tests::try_init_logger;

#[tokio::test]
async fn test_didcomm_end_to_end() -> ActorResult<()> {
  try_init_logger();

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  pub struct TestRequest(u32);

  impl DidCommRequest for TestRequest {
    fn endpoint() -> Endpoint {
      "test/request".try_into().unwrap()
    }
  }

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  pub struct TestRequestAlt(u32);

  impl DidCommRequest for TestRequestAlt {
    fn endpoint() -> Endpoint {
      "test/request_alt".try_into().unwrap()
    }
  }

  #[derive(Debug, Clone)]
  struct MyActor {
    counter: Arc<AtomicU32>,
  }

  #[async_trait::async_trait]
  impl DidCommActor<DidCommPlaintextMessage<TestRequest>> for MyActor {
    async fn handle(&self, mut system: DidCommSystem, request: RequestContext<DidCommPlaintextMessage<TestRequest>>) {
      self.counter.fetch_add(request.input.body.0, Ordering::SeqCst);

      system
        .send_message(request.peer_id, request.input.thread_id(), TestRequestAlt(21))
        .await
        .unwrap();

      let message: DidCommPlaintextMessage<TestRequestAlt> =
        system.await_message(request.input.thread_id()).await.unwrap();

      assert_eq!(message.body.0, 1337);

      system
        .send_message(request.peer_id, request.input.thread_id(), TestRequestAlt(7))
        .await
        .unwrap();
    }
  }

  let mut builder = DidCommSystemBuilder::new().identity(default_identity());

  let actor = MyActor {
    counter: Arc::new(AtomicU32::new(0)),
  };
  builder.attach_didcomm(actor.clone());

  let mut listening_system: DidCommSystem = builder.build().await.unwrap();

  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
  let _ = listening_system.start_listening(addr).await.unwrap();
  let addrs = listening_system.addresses().await.unwrap();

  let peer_id = listening_system.peer_id();

  let mut sending_system = DidCommSystemBuilder::new()
    .identity(default_identity())
    .build()
    .await
    .unwrap();

  sending_system.add_addresses(peer_id, addrs).await.unwrap();

  let thread_id = ThreadId::new();

  sending_system
    .send_message(peer_id, &thread_id, TestRequest(42))
    .await
    .unwrap();

  let message: DidCommPlaintextMessage<TestRequestAlt> = sending_system.await_message(&thread_id).await.unwrap();

  assert_eq!(message.body.0, 21);

  sending_system
    .send_message(peer_id, &thread_id, TestRequestAlt(1337))
    .await
    .unwrap();

  let message: DidCommPlaintextMessage<TestRequestAlt> = sending_system.await_message(&thread_id).await.unwrap();

  assert_eq!(message.body.0, 7);
  assert_eq!(actor.counter.load(std::sync::atomic::Ordering::SeqCst), 42);

  // Allow background tasks to finish.
  // The test also succeeds without this, but might cause the background tasks to panic or log an error.
  tokio::task::yield_now().await;

  listening_system.shutdown().await.unwrap();
  sending_system.shutdown().await.unwrap();

  Ok(())
}

/// Ensure the DidCommSystem supports actors working with `ActorRequest`s.
#[tokio::test]
async fn test_didcomm_system_supports_actor_requests() -> ActorResult<()> {
  try_init_logger();

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  pub struct SyncDummy(u16);

  impl ActorRequest for SyncDummy {
    type Response = u16;

    fn endpoint() -> Endpoint {
      "test/request".try_into().unwrap()
    }
  }

  #[derive(Debug)]
  struct TestActor;

  #[async_trait::async_trait]
  impl Actor<SyncDummy> for TestActor {
    async fn handle(&self, request: RequestContext<SyncDummy>) -> u16 {
      request.input.0
    }
  }

  let (listening_actor, addrs, peer_id) = default_listening_didcomm_system(|mut builder| {
    builder.attach(TestActor);
    builder
  })
  .await;

  let mut sending_system = default_sending_didcomm_system(|builder| builder).await;
  sending_system.add_addresses(peer_id, addrs).await.unwrap();

  let result = sending_system.send_request(peer_id, SyncDummy(42)).await;

  assert_eq!(result.unwrap(), 42);

  listening_actor.shutdown().await.unwrap();
  sending_system.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_unknown_thread_returns_error() -> ActorResult<()> {
  try_init_logger();

  let (listening_actor, addrs, peer_id) = default_listening_didcomm_system(|builder| builder).await;

  let mut sending_system = default_sending_didcomm_system(|builder| builder).await;
  sending_system.add_addresses(peer_id, addrs).await.unwrap();

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  pub struct AsyncDummy(u16);

  impl DidCommRequest for AsyncDummy {
    fn endpoint() -> Endpoint {
      "unknown/thread".try_into().unwrap()
    }
  }

  let result = sending_system
    .send_message(peer_id, &ThreadId::new(), AsyncDummy(42))
    .await;

  assert!(matches!(result.unwrap_err(), Error::UnexpectedRequest(_)));

  listening_actor.shutdown().await.unwrap();
  sending_system.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_didcomm_presentation_holder_initiates() -> ActorResult<()> {
  try_init_logger();
  let actor: DidCommState = DidCommState::new().await;

  let mut holder_system: DidCommSystem = default_sending_didcomm_system(|builder| builder).await;

  let (verifier_system, addrs, peer_id) = default_listening_didcomm_system(|mut builder| {
    builder.attach_didcomm::<DidCommPlaintextMessage<PresentationOffer>, _>(actor.clone());
    builder
  })
  .await;

  holder_system.add_addresses(peer_id, addrs).await.unwrap();

  presentation_holder_handler(holder_system.clone(), peer_id, None)
    .await
    .unwrap();

  // Allow background tasks to finish.
  // The test also succeeds without this, but might cause the background tasks to panic or log an error.
  tokio::task::yield_now().await;

  verifier_system.shutdown().await.unwrap();
  holder_system.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_didcomm_presentation_verifier_initiates() -> ActorResult<()> {
  try_init_logger();

  let actor = DidCommState::new().await;

  let (holder_system, addrs, peer_id) = default_listening_didcomm_system(|mut builder| {
    builder.attach_didcomm::<DidCommPlaintextMessage<PresentationRequest>, _>(actor.clone());
    builder
  })
  .await;
  let mut verifier_system = default_sending_didcomm_system(|builder| builder).await;

  verifier_system.add_addresses(peer_id, addrs).await.unwrap();

  presentation_verifier_handler(verifier_system.clone(), peer_id, None)
    .await
    .unwrap();

  holder_system.shutdown().await.unwrap();
  verifier_system.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_sending_to_unconnected_peer_returns_error() -> ActorResult<()> {
  try_init_logger();

  let mut sending_system = default_sending_didcomm_system(|builder| builder).await;

  let result = sending_system.send_request(PeerId::random(), IdentityList).await;

  assert!(matches!(result.unwrap_err(), Error::OutboundFailure(_)));

  let result = sending_system
    .send_message(PeerId::random(), &ThreadId::new(), PresentationOffer::default())
    .await;

  assert!(matches!(result.unwrap_err(), Error::OutboundFailure(_)));

  sending_system.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_await_message_returns_timeout_error() -> ActorResult<()> {
  try_init_logger();

  #[derive(Debug, Clone)]
  struct MyActor;

  #[async_trait::async_trait]
  impl DidCommActor<DidCommPlaintextMessage<PresentationOffer>> for MyActor {
    async fn handle(&self, _: DidCommSystem, _: RequestContext<DidCommPlaintextMessage<PresentationOffer>>) {}
  }

  let (listening_actor, addrs, peer_id) = default_listening_didcomm_system(|mut builder| {
    builder.attach_didcomm(MyActor);
    builder
  })
  .await;

  let mut sending_system: DidCommSystem =
    default_sending_didcomm_system(|builder| builder.timeout(std::time::Duration::from_millis(50))).await;

  sending_system.add_addresses(peer_id, addrs).await.unwrap();

  let thread_id = ThreadId::new();
  sending_system
    .send_message(peer_id, &thread_id, PresentationOffer::default())
    .await
    .unwrap();

  let result = sending_system.await_message::<()>(&thread_id).await;

  assert!(matches!(result.unwrap_err(), Error::AwaitTimeout(_)));

  listening_actor.shutdown().await.unwrap();
  sending_system.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_handler_finishes_execution_after_shutdown() -> ActorResult<()> {
  try_init_logger();

  #[derive(Debug, Clone)]
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
  impl DidCommActor<DidCommPlaintextMessage<PresentationOffer>> for TestActor {
    async fn handle(&self, _: DidCommSystem, _: RequestContext<DidCommPlaintextMessage<PresentationOffer>>) {
      tokio::time::sleep(std::time::Duration::from_millis(25)).await;
      self.was_called.store(true, Ordering::SeqCst);
    }
  }

  let test_actor = TestActor::new();

  let (listening_actor, addrs, peer_id) = default_listening_didcomm_system(|mut builder| {
    builder.attach_didcomm(test_actor.clone());
    builder
  })
  .await;

  let mut sending_system: DidCommSystem = default_sending_didcomm_system(|builder| builder).await;
  sending_system.add_addresses(peer_id, addrs).await.unwrap();

  sending_system
    .send_message(peer_id, &ThreadId::new(), PresentationOffer::default())
    .await
    .unwrap();

  listening_actor.shutdown().await.unwrap();

  tokio::time::sleep(std::time::Duration::from_millis(50)).await;

  sending_system.shutdown().await.unwrap();

  assert!(test_actor.was_called.load(std::sync::atomic::Ordering::SeqCst));

  Ok(())
}
