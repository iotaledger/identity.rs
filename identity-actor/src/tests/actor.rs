// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::task::Poll;

use futures::pin_mut;
use libp2p::request_response::OutboundFailure;
use libp2p::Multiaddr;

use crate::remote_account::IdentityGet;
use crate::remote_account::IdentityList;
use crate::tests::try_init_logger;
use crate::Actor;
use crate::ActorBuilder;
use crate::ActorRequest;
use crate::Error;
use crate::ErrorLocation;
use crate::RequestContext;
use crate::Synchronous;

use super::default_listening_actor;
use super::default_sending_actor;

#[tokio::test]
async fn test_unknown_request_returns_error() -> crate::Result<()> {
  try_init_logger();

  let (listening_actor, addrs, peer_id) = default_listening_actor(|_| {}).await;

  let mut sending_actor = default_sending_actor(|_| {}).await;
  sending_actor.add_addresses(peer_id, addrs).await.unwrap();

  let result = sending_actor
    .send_named_request(
      peer_id,
      "unknown/request",
      IdentityGet("did:iota:FFFAH6qct9KGQcSenG1iaw2Nj9jP7Zmug2zcmTpF4942".parse().unwrap()),
    )
    .await;

  assert!(matches!(result.unwrap_err(), Error::UnexpectedRequest(_)));

  listening_actor.shutdown().await.unwrap();
  sending_actor.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_actors_can_communicate_bidirectionally() -> crate::Result<()> {
  try_init_logger();

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  pub struct Dummy(u8);

  impl ActorRequest<Synchronous> for Dummy {
    type Response = ();

    fn endpoint() -> &'static str {
      "request/test"
    }
  }

  #[derive(Clone)]
  pub struct State(pub Arc<AtomicBool>);

  impl State {
    async fn handler(self, _actor: Actor, _req: RequestContext<Dummy>) {
      self.0.store(true, std::sync::atomic::Ordering::SeqCst);
    }
  }

  let actor1_state = State(Arc::new(AtomicBool::new(false)));
  let actor2_state = State(Arc::new(AtomicBool::new(false)));

  let mut actor1_builder = ActorBuilder::new();
  actor1_builder
    .add_state(actor1_state.clone())
    .add_sync_handler(State::handler)
    .unwrap();
  let mut actor1: Actor = actor1_builder.build().await.unwrap();

  let mut actor2_builder = ActorBuilder::new();
  actor2_builder
    .add_state(actor2_state.clone())
    .add_sync_handler(State::handler)
    .unwrap();
  let mut actor2: Actor = actor2_builder.build().await.unwrap();

  actor2
    .start_listening("/ip4/0.0.0.0/tcp/0".parse().unwrap())
    .await
    .unwrap();

  let addr: Multiaddr = actor2.addresses().await.unwrap().into_iter().next().unwrap();

  actor1.add_address(actor2.peer_id(), addr).await.unwrap();

  actor1.send_request(actor2.peer_id(), Dummy(42)).await.unwrap();

  actor2.send_request(actor1.peer_id(), Dummy(43)).await.unwrap();

  actor1.shutdown().await.unwrap();
  actor2.shutdown().await.unwrap();

  assert!(actor1_state.0.load(std::sync::atomic::Ordering::SeqCst));
  assert!(actor2_state.0.load(std::sync::atomic::Ordering::SeqCst));

  Ok(())
}

#[tokio::test]
async fn test_actor_handler_is_invoked() -> crate::Result<()> {
  try_init_logger();

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  pub struct Dummy(u8);

  impl ActorRequest<Synchronous> for Dummy {
    type Response = ();

    fn endpoint() -> &'static str {
      "request/test"
    }
  }

  #[derive(Clone)]
  pub struct State(pub Arc<AtomicBool>);

  impl State {
    async fn handler(self, _actor: Actor, req: RequestContext<Dummy>) {
      if let Dummy(42) = req.input {
        self.0.store(true, std::sync::atomic::Ordering::SeqCst);
      }
    }
  }

  let state = State(Arc::new(AtomicBool::new(false)));

  let (receiver, receiver_addrs, receiver_peer_id) = default_listening_actor(|builder| {
    builder
      .add_state(state.clone())
      .add_sync_handler(State::handler)
      .unwrap();
  })
  .await;
  let mut sender = default_sending_actor(|_| {}).await;

  sender.add_addresses(receiver_peer_id, receiver_addrs).await.unwrap();

  sender.send_request(receiver_peer_id, Dummy(42)).await.unwrap();

  sender.shutdown().await.unwrap();
  receiver.shutdown().await.unwrap();

  assert!(state.0.load(std::sync::atomic::Ordering::SeqCst));

  Ok(())
}

#[tokio::test]
async fn test_synchronous_handler_invocation() -> crate::Result<()> {
  try_init_logger();

  #[derive(Debug, serde::Serialize, serde::Deserialize)]
  pub struct MessageResponse(String);

  #[derive(Debug, serde::Serialize, serde::Deserialize)]
  pub struct MessageRequest(String);

  impl ActorRequest<Synchronous> for MessageRequest {
    type Response = MessageResponse;

    fn endpoint() -> &'static str {
      "test/message"
    }
  }

  let (listening_actor, addrs, peer_id) = default_listening_actor(|builder| {
    builder
      .add_state(())
      .add_sync_handler(|_: (), _: Actor, message: RequestContext<MessageRequest>| async move {
        MessageResponse(message.input.0)
      })
      .unwrap();
  })
  .await;

  let mut sending_actor = default_sending_actor(|_| {}).await;
  sending_actor.add_addresses(peer_id, addrs).await.unwrap();

  let result = sending_actor
    .send_request(peer_id, MessageRequest("test".to_owned()))
    .await;

  assert_eq!(result.unwrap().0, "test".to_owned());

  listening_actor.shutdown().await.unwrap();
  sending_actor.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_interacting_with_shutdown_actor_returns_error() {
  try_init_logger();

  let (listening_actor, _, _) = default_listening_actor(|_| {}).await;

  let mut actor_clone = listening_actor.clone();

  listening_actor.shutdown().await.unwrap();

  assert!(matches!(actor_clone.addresses().await.unwrap_err(), Error::Shutdown));
}

#[tokio::test]
async fn test_shutdown_returns_errors_through_open_channels() -> crate::Result<()> {
  try_init_logger();

  let (listening_actor, addrs, peer_id) = default_listening_actor(|builder| {
    builder
      .add_state(())
      .add_sync_handler(|_: (), _: Actor, _message: RequestContext<IdentityList>| async move {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        vec![]
      })
      .unwrap();
  })
  .await;

  let mut sending_actor: Actor = ActorBuilder::new().build().await.unwrap();
  sending_actor.add_addresses(peer_id, addrs).await.unwrap();

  let mut sender1 = sending_actor.clone();

  // Ensure that an actor shutdown returns errors through open channels,
  // such as `EventLoop::await_response`.
  // We do not test all `EventLoop::await*` fields, because some are
  // much harder to test than others.
  // We poll the futures once to ensure that the channels are created,
  // before shutting the actor down. If we would call these methods after shutdown,
  // they would immediately return a shutdown error (see test_interacting_with_shutdown_actor_returns_error),
  // hence the need for manual polling.
  // On the next poll after shutdown, we expect the errors.

  let send_request_future = sender1.send_request(peer_id, IdentityList);
  pin_mut!(send_request_future);
  let result = futures::poll!(&mut send_request_future);
  assert!(matches!(result, Poll::Pending));

  sending_actor.shutdown().await.unwrap();

  let result = send_request_future.await;
  assert!(matches!(
    result.unwrap_err(),
    Error::OutboundFailure(OutboundFailure::ConnectionClosed)
  ));

  listening_actor.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_endpoint_type_mismatch_result_in_serialization_errors() -> crate::Result<()> {
  try_init_logger();

  // Define two types with identical serialization results, but different `Response` types.
  // Sending `CustomRequest2` to an endpoint expecting `CustomRequest`, we expect a local deserialization error.

  #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
  pub struct CustomRequest(u8);

  #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
  pub struct CustomRequest2(u8);

  impl ActorRequest<Synchronous> for CustomRequest {
    type Response = String;

    fn endpoint() -> &'static str {
      "test/request"
    }
  }

  impl ActorRequest<Synchronous> for CustomRequest2 {
    type Response = u32;

    fn endpoint() -> &'static str {
      "test/request"
    }
  }

  let (listening_actor, addrs, peer_id) = default_listening_actor(|builder| {
    builder
      .add_state(())
      .add_sync_handler(|_: (), _: Actor, _: RequestContext<CustomRequest2>| async move { 42 })
      .unwrap();
  })
  .await;

  let mut sending_actor: Actor = ActorBuilder::new().build().await.unwrap();
  sending_actor.add_addresses(peer_id, addrs).await.unwrap();

  let result = sending_actor.send_request(peer_id, CustomRequest(13)).await;

  assert!(matches!(
    result.unwrap_err(),
    Error::DeserializationFailure {
      location: ErrorLocation::Local,
      ..
    }
  ));

  // Define a third type that has a different serialization result.
  // We expect a deserialization error on the peer.
  #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
  pub struct CustomRequest3(String);

  impl ActorRequest<Synchronous> for CustomRequest3 {
    type Response = String;

    fn endpoint() -> &'static str {
      "test/request"
    }
  }

  let result = sending_actor
    .send_request(peer_id, CustomRequest3("13".to_owned()))
    .await;

  assert!(matches!(
    result.unwrap_err(),
    Error::DeserializationFailure {
      location: ErrorLocation::Remote,
      ..
    }
  ));

  listening_actor.shutdown().await.unwrap();
  sending_actor.shutdown().await.unwrap();

  Ok(())
}
