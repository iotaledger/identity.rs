// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_did::verification::MethodScope;
use identity_iota_core::document::IotaDocument;
use identity_iota_core::document::IotaVerificationMethod;
use libp2p::PeerId;

use crate::actor::Actor;
use crate::actor::ActorRequest;
use crate::actor::Asynchronous;
use crate::actor::Endpoint;
use crate::actor::Error;
use crate::actor::RequestContext;
use crate::actor::Result as ActorResult;
use crate::actor::Synchronous;
use crate::didcomm::accept_invitation;
use crate::didcomm::presentation_holder_handler;
use crate::didcomm::presentation_verifier_handler;
use crate::didcomm::ActorIdentity;
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
use std::collections::HashMap;
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

  impl ActorRequest<Synchronous> for SyncDummy {
    type Response = u16;

    fn endpoint() -> Endpoint {
      "test/request".parse().unwrap()
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

  impl ActorRequest<Asynchronous> for AsyncDummy {
    type Response = ();

    fn endpoint() -> Endpoint {
      "unknown/thread".parse().unwrap()
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

static KEX_FRAGMENT: &str = "kex-0";

fn identity_2() -> ActorIdentity {
  let keypair = KeyPair::try_from_private_key_bytes(
    KeyType::Ed25519,
    &[
      154, 100, 136, 109, 201, 151, 157, 92, 21, 214, 52, 233, 88, 102, 117, 76, 144, 248, 128, 187, 112, 189, 87, 253,
      238, 13, 179, 193, 149, 96, 176, 209,
    ],
  )
  .unwrap();
  let x25519 = KeyPair::try_from_private_key_bytes(
    KeyType::X25519,
    &[
      160, 50, 193, 6, 97, 85, 10, 87, 116, 249, 147, 166, 142, 232, 128, 175, 38, 33, 142, 66, 13, 125, 64, 162, 241,
      86, 146, 138, 230, 254, 209, 118,
    ],
  )
  .unwrap();

  reconstruct_id(keypair, x25519)
}

fn identity_1() -> ActorIdentity {
  let keypair = KeyPair::try_from_private_key_bytes(
    KeyType::Ed25519,
    &[
      86, 200, 169, 226, 190, 157, 62, 209, 68, 151, 90, 36, 1, 194, 65, 184, 72, 20, 45, 23, 162, 40, 6, 84, 68, 239,
      69, 81, 242, 115, 95, 15,
    ],
  )
  .unwrap();
  let x25519 = KeyPair::try_from_private_key_bytes(
    KeyType::X25519,
    &[
      112, 69, 87, 124, 143, 165, 183, 132, 17, 94, 151, 94, 33, 206, 19, 38, 93, 146, 103, 164, 170, 230, 18, 24, 67,
      248, 142, 19, 128, 124, 37, 68,
    ],
  )
  .unwrap();

  reconstruct_id(keypair, x25519)
}

// fn example_identity() -> ActorIdentity {
//   let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
//   let keypairx: KeyPair = KeyPair::new(KeyType::X25519).unwrap();
//   reconstruct_id(keypair, keypairx)
// }

fn reconstruct_id(keypair: KeyPair, keypairx: KeyPair) -> ActorIdentity {
  let mut document: IotaDocument = IotaDocument::new(&keypair).unwrap();

  let method =
    IotaVerificationMethod::new(document.id().clone(), keypairx.type_(), keypairx.public(), KEX_FRAGMENT).unwrap();
  let url = method.id().to_owned();
  document.insert_method(method, MethodScope::key_agreement()).unwrap();

  let mut keypairs = HashMap::new();
  keypairs.insert(document.default_signing_method().unwrap().id().to_owned(), keypair);

  keypairs.insert(url, keypairx);

  ActorIdentity::from((document, keypairs))
}

#[tokio::test]
async fn test_didcomm_connection() -> ActorResult<()> {
  try_init_logger();
  let handler = DidCommState::new();

  let mut sender_actor = default_sending_didcomm_actor(|builder| {
    let id = identity_1();
    println!("sender has did {}", id.doc.id());
    builder.identity(id)
  })
  .await;

  let test_state = TestFunctionState::new();

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  pub struct TestMessage(String);

  impl ActorRequest<Asynchronous> for TestMessage {
    type Response = ();

    fn endpoint() -> Endpoint {
      "didcomm/test_message".parse().unwrap()
    }
  }

  let (recv_actor, addrs, peer_id) = default_listening_didcomm_actor(|mut builder| {
    let id = identity_2();
    println!("recv has did {}", id.doc.id());
    builder = builder.identity(id);

    builder.add_state(test_state).add_async_handler(
      |state: TestFunctionState, _: DidCommActor, _: RequestContext<DidCommPlaintextMessage<TestMessage>>| async move {
        state.was_called.store(true, Ordering::SeqCst);
      },
    );

    builder.add_state(handler).add_async_handler(DidCommState::connection);

    builder
  })
  .await;

  sender_actor.add_addresses(peer_id, addrs).await.unwrap();

  let own_key_url = sender_actor
    .state
    .identity
    .doc
    .resolve_method(KEX_FRAGMENT, Some(MethodScope::key_agreement()))
    .unwrap()
    .id()
    .to_owned();

  let peer_key_url = recv_actor
    .state
    .identity
    .doc
    .resolve_method(KEX_FRAGMENT, Some(MethodScope::key_agreement()))
    .unwrap()
    .id()
    .to_owned();

  accept_invitation(&mut sender_actor, peer_id, own_key_url, peer_key_url)
    .await
    .unwrap();

  let thread_id = ThreadId::new();
  sender_actor
    .send_message(peer_id, &thread_id, TestMessage("hello".to_owned()))
    .await
    .unwrap();

  recv_actor.shutdown().await.unwrap();
  sender_actor.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_didcomm_presentation_holder_initiates() -> ActorResult<()> {
  try_init_logger();
  let handler = DidCommState::new();

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

  let handler = DidCommState::new();

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

  let handler = DidCommState::new();

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

  let handler = DidCommState::new();

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
