// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use crate::agent::Actor;
use crate::agent::ActorRequest;
use crate::agent::AgentId;
use crate::agent::Endpoint;
use crate::agent::Error;
use crate::agent::RequestContext;
use crate::agent::Result as AgentResult;
use crate::didcomm::DidCommActor;
use crate::didcomm::DidCommPlaintextMessage;
use crate::didcomm::DidCommRequest;
use crate::didcomm::DidCommSystem;
use crate::didcomm::ThreadId;
use crate::tests::default_listening_didcomm_system;
use crate::tests::default_sending_didcomm_system;
use crate::tests::presentation::presentation_holder_handler;
use crate::tests::presentation::presentation_verifier_handler;
use crate::tests::presentation::DidCommState;
use crate::tests::presentation::PresentationOffer;
use crate::tests::presentation::PresentationRequest;
use crate::tests::remote_account::IdentityList;
use crate::tests::try_init_logger;

/// Ensure the DidCommSystem supports actors working with `ActorRequest`s (rather than `DidCommRequest`s).
#[tokio::test]
async fn test_didcomm_system_supports_actor_requests() -> AgentResult<()> {
  try_init_logger();

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  struct SyncDummy(u16);

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

  let (listening_actor, addrs, agent_id) = default_listening_didcomm_system(|mut builder| {
    builder.attach(TestActor);
    builder
  })
  .await;

  let mut sending_system = default_sending_didcomm_system(|builder| builder).await;
  sending_system.add_agent_addresses(agent_id, addrs).await.unwrap();

  let result = sending_system.send_request(agent_id, SyncDummy(42)).await;

  assert_eq!(result.unwrap(), 42);

  listening_actor.shutdown().await.unwrap();
  sending_system.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_unknown_thread_returns_error() -> AgentResult<()> {
  try_init_logger();

  let (listening_actor, addrs, agent_id) = default_listening_didcomm_system(|builder| builder).await;

  let mut sending_system = default_sending_didcomm_system(|builder| builder).await;
  sending_system.add_agent_addresses(agent_id, addrs).await.unwrap();

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  struct DidCommTestRequest(u16);

  impl DidCommRequest for DidCommTestRequest {
    fn endpoint() -> Endpoint {
      "unknown/thread".try_into().unwrap()
    }
  }

  // Send a message that no handling actor on the remote agent exists for
  // which causes the remote agent to look for a potential thread that is waiting for this message,
  // but no such thread exists either, so an error is returned.
  let result = sending_system
    .send_message(agent_id, &ThreadId::new(), DidCommTestRequest(42))
    .await;

  assert!(matches!(result.unwrap_err(), Error::UnexpectedRequest(_)));

  listening_actor.shutdown().await.unwrap();
  sending_system.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_didcomm_presentation_holder_initiates() -> AgentResult<()> {
  try_init_logger();
  let actor: DidCommState = DidCommState::new();

  let mut holder_system: DidCommSystem = default_sending_didcomm_system(|builder| builder).await;

  // Attach the DidCommState actor to the listening system, so it can handle PresentationOffer requests.
  let (verifier_system, addrs, agent_id) = default_listening_didcomm_system(|mut builder| {
    builder.attach_didcomm::<DidCommPlaintextMessage<PresentationOffer>, _>(actor.clone());
    builder
  })
  .await;

  holder_system.add_agent_addresses(agent_id, addrs).await.unwrap();

  // Holder initiates the presentation protocol.
  presentation_holder_handler(holder_system.clone(), agent_id, None)
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
async fn test_didcomm_presentation_verifier_initiates() -> AgentResult<()> {
  try_init_logger();

  let actor = DidCommState::new();

  // Attach the DidCommState actor to the listening system, so it can handle PresentationRequest requests.
  let (holder_system, addrs, agent_id) = default_listening_didcomm_system(|mut builder| {
    builder.attach_didcomm::<DidCommPlaintextMessage<PresentationRequest>, _>(actor.clone());
    builder
  })
  .await;
  let mut verifier_system = default_sending_didcomm_system(|builder| builder).await;

  verifier_system.add_agent_addresses(agent_id, addrs).await.unwrap();

  // Verifier initiates the presentation protocol.
  presentation_verifier_handler(verifier_system.clone(), agent_id, None)
    .await
    .unwrap();

  holder_system.shutdown().await.unwrap();
  verifier_system.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_sending_to_unconnected_peer_returns_error() -> AgentResult<()> {
  try_init_logger();

  let mut sending_system = default_sending_didcomm_system(|builder| builder).await;

  // Send a request without adding an address first.
  let result = sending_system.send_request(AgentId::random(), IdentityList).await;

  assert!(matches!(result.unwrap_err(), Error::OutboundFailure(_)));

  let result = sending_system
    .send_message(AgentId::random(), &ThreadId::new(), PresentationOffer::default())
    .await;

  assert!(matches!(result.unwrap_err(), Error::OutboundFailure(_)));

  sending_system.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_await_message_returns_timeout_error() -> AgentResult<()> {
  try_init_logger();

  #[derive(Debug, Clone)]
  struct MyActor;

  #[async_trait::async_trait]
  impl DidCommActor<DidCommPlaintextMessage<PresentationOffer>> for MyActor {
    async fn handle(&self, _: DidCommSystem, _: RequestContext<DidCommPlaintextMessage<PresentationOffer>>) {}
  }

  let (listening_actor, addrs, agent_id) = default_listening_didcomm_system(|mut builder| {
    builder.attach_didcomm(MyActor);
    builder
  })
  .await;

  let mut sending_system: DidCommSystem =
    default_sending_didcomm_system(|builder| builder.timeout(Duration::from_millis(50))).await;

  sending_system.add_agent_addresses(agent_id, addrs).await.unwrap();

  let thread_id = ThreadId::new();
  sending_system
    .send_message(agent_id, &thread_id, PresentationOffer::default())
    .await
    .unwrap();

  // We attempt to await a message, but the remote agent never sends one, so we expect a timeout.
  let result = sending_system.await_message::<()>(&thread_id).await;

  assert!(matches!(result.unwrap_err(), Error::AwaitTimeout(_)));

  listening_actor.shutdown().await.unwrap();
  sending_system.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_handler_finishes_execution_after_shutdown() -> AgentResult<()> {
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
      tokio::time::sleep(Duration::from_millis(25)).await;
      self.was_called.store(true, Ordering::SeqCst);
    }
  }

  let test_actor = TestActor::new();

  let (listening_system, addrs, agent_id) = default_listening_didcomm_system(|mut builder| {
    builder.attach_didcomm(test_actor.clone());
    builder
  })
  .await;

  let mut sending_system: DidCommSystem = default_sending_didcomm_system(|builder| builder).await;
  sending_system.add_agent_addresses(agent_id, addrs).await.unwrap();

  sending_system
    .send_message(agent_id, &ThreadId::new(), PresentationOffer::default())
    .await
    .unwrap();

  // Shut down the system that executes the actor, and wait for some time to allow the handler to finish.
  // Even though we shut the system down, we expect the task that the actor is running in to finish.
  listening_system.shutdown().await.unwrap();

  tokio::time::sleep(Duration::from_millis(50)).await;

  sending_system.shutdown().await.unwrap();

  assert!(test_actor.was_called.load(Ordering::SeqCst));

  Ok(())
}
