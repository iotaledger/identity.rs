// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::task::Poll;

use futures::pin_mut;
use identity_iota_core::did::IotaDID;
use libp2p::request_response::OutboundFailure;
use libp2p::Multiaddr;

use crate::actor::Endpoint;
use crate::actor::Error;
use crate::actor::ErrorLocation;
use crate::actor::RequestContext;
use crate::actor::Result as ActorResult;
use crate::actor::SyncActor;
use crate::actor::SyncActorRequest;
use crate::actor::System;
use crate::actor::SystemBuilder;
use crate::remote_account::IdentityGet;
use crate::remote_account::IdentityList;
use crate::tests::try_init_logger;

use super::default_listening_actor;
use super::default_sending_actor;

#[tokio::test]
async fn test_new_approach() -> ActorResult<()> {
  try_init_logger();

  #[derive(Clone)]
  struct MyActor {
    counter: Arc<AtomicU32>,
  }

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  struct Increment(u32);

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  struct Decrement(u32);

  impl SyncActorRequest for Increment {
    type Response = ();

    fn endpoint() -> Endpoint {
      "counter/increment".try_into().unwrap()
    }
  }

  impl SyncActorRequest for Decrement {
    type Response = ();

    fn endpoint() -> Endpoint {
      "counter/decrement".try_into().unwrap()
    }
  }

  // States that MyActor can handle messages of type `Increment`.
  #[async_trait::async_trait]
  impl SyncActor<Increment> for MyActor {
    async fn handle(&self, request: RequestContext<Increment>) {
      self.counter.fetch_add(request.input.0, Ordering::SeqCst);
    }
  }

  #[async_trait::async_trait]
  impl SyncActor<Decrement> for MyActor {
    async fn handle(&self, request: RequestContext<Decrement>) {
      self.counter.fetch_sub(request.input.0, Ordering::SeqCst);
    }
  }

  let actor = MyActor {
    counter: Arc::new(AtomicU32::new(0)),
  };

  let mut builder = SystemBuilder::new();
  // Let's this system "manage" this actor, by sending incoming requests to it when they match.
  builder.attach::<Increment, _>(actor.clone());
  builder.attach::<Decrement, _>(actor.clone());

  let mut listening_system: System = builder.build().await.unwrap();

  let _ = listening_system
    .start_listening("/ip4/0.0.0.0/tcp/0".parse().unwrap())
    .await
    .unwrap();
  let addrs = listening_system.addresses().await.unwrap();

  let peer_id = listening_system.peer_id();

  let mut sender_system: System = SystemBuilder::new().build().await.unwrap();

  sender_system.add_addresses(peer_id, addrs).await.unwrap();

  sender_system.send_request(peer_id, Increment(3)).await.unwrap();

  assert_eq!(actor.counter.load(Ordering::SeqCst), 3);

  sender_system.send_request(peer_id, Decrement(2)).await.unwrap();

  assert_eq!(actor.counter.load(Ordering::SeqCst), 1);

  listening_system.shutdown().await.unwrap();
  sender_system.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_unknown_request_returns_error() -> ActorResult<()> {
  try_init_logger();

  let (listening_actor, addrs, peer_id) = default_listening_actor(|builder| builder).await;

  let mut sending_actor = default_sending_actor(|builder| builder).await;
  sending_actor.add_addresses(peer_id, addrs).await.unwrap();

  let result = sending_actor
    .send_request(
      peer_id,
      IdentityGet(
        "did:iota:FFFAH6qct9KGQcSenG1iaw2Nj9jP7Zmug2zcmTpF4942"
          .try_into()
          .unwrap(),
      ),
    )
    .await;

  assert!(matches!(result.unwrap_err(), Error::UnexpectedRequest(_)));

  listening_actor.shutdown().await.unwrap();
  sending_actor.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_actors_can_communicate_bidirectionally() -> ActorResult<()> {
  try_init_logger();

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  pub struct Dummy(u8);

  impl SyncActorRequest for Dummy {
    type Response = ();

    fn endpoint() -> Endpoint {
      "request/test".try_into().unwrap()
    }
  }

  #[derive(Clone)]
  pub struct Actor(pub Arc<AtomicBool>);

  #[async_trait::async_trait]
  impl SyncActor<Dummy> for Actor {
    async fn handle(&self, _req: RequestContext<Dummy>) {
      self.0.store(true, std::sync::atomic::Ordering::SeqCst);
    }
  }

  let actor1 = Actor(Arc::new(AtomicBool::new(false)));
  let actor2 = Actor(Arc::new(AtomicBool::new(false)));

  let mut system1_builder = SystemBuilder::new();
  system1_builder.attach(actor1.clone());
  let mut system1: System = system1_builder.build().await.unwrap();

  let mut system2_builder = SystemBuilder::new();
  system2_builder.attach(actor2.clone());
  let mut system2: System = system2_builder.build().await.unwrap();

  system2
    .start_listening("/ip4/0.0.0.0/tcp/0".try_into().unwrap())
    .await
    .unwrap();

  let addr: Multiaddr = system2.addresses().await.unwrap().into_iter().next().unwrap();

  system1.add_address(system2.peer_id(), addr).await.unwrap();

  system1.send_request(system2.peer_id(), Dummy(42)).await.unwrap();

  system2.send_request(system1.peer_id(), Dummy(43)).await.unwrap();

  system1.shutdown().await.unwrap();
  system2.shutdown().await.unwrap();

  assert!(actor1.0.load(std::sync::atomic::Ordering::SeqCst));
  assert!(actor2.0.load(std::sync::atomic::Ordering::SeqCst));

  Ok(())
}

#[tokio::test]
async fn test_actor_handler_is_invoked() -> ActorResult<()> {
  try_init_logger();

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  pub struct Dummy(u8);

  impl SyncActorRequest for Dummy {
    type Response = ();

    fn endpoint() -> Endpoint {
      "request/test".try_into().unwrap()
    }
  }

  #[derive(Clone)]
  pub struct Actor(pub Arc<AtomicBool>);

  #[async_trait::async_trait]
  impl SyncActor<Dummy> for Actor {
    async fn handle(&self, request: RequestContext<Dummy>) {
      if let Dummy(42) = request.input {
        self.0.store(true, std::sync::atomic::Ordering::SeqCst);
      }
    }
  }

  let actor = Actor(Arc::new(AtomicBool::new(false)));

  let (receiver, receiver_addrs, receiver_peer_id) = default_listening_actor(|mut builder| {
    builder.attach(actor.clone());
    // builder.add_state(state.clone()).add_sync_handler(Actor::handler);
    builder
  })
  .await;
  let mut sender = default_sending_actor(|builder| builder).await;

  sender.add_addresses(receiver_peer_id, receiver_addrs).await.unwrap();

  sender.send_request(receiver_peer_id, Dummy(42)).await.unwrap();

  sender.shutdown().await.unwrap();
  receiver.shutdown().await.unwrap();

  assert!(actor.0.load(std::sync::atomic::Ordering::SeqCst));

  Ok(())
}

#[tokio::test]
async fn test_synchronous_handler_invocation() -> ActorResult<()> {
  try_init_logger();

  #[derive(Debug, serde::Serialize, serde::Deserialize)]
  pub struct MessageResponse(String);

  #[derive(Debug, serde::Serialize, serde::Deserialize)]
  pub struct MessageRequest(String);

  impl SyncActorRequest for MessageRequest {
    type Response = MessageResponse;

    fn endpoint() -> Endpoint {
      "test/message".try_into().unwrap()
    }
  }

  struct Actor;

  #[async_trait::async_trait]
  impl SyncActor<MessageRequest> for Actor {
    async fn handle(&self, message: RequestContext<MessageRequest>) -> MessageResponse {
      MessageResponse(message.input.0)
    }
  }

  let (listening_actor, addrs, peer_id) = default_listening_actor(|mut builder| {
    builder.attach(Actor);
    builder
  })
  .await;

  let mut sending_actor = default_sending_actor(|builder| builder).await;
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

  let (listening_actor, _, _) = default_listening_actor(|builder| builder).await;

  let mut actor_clone = listening_actor.clone();

  listening_actor.shutdown().await.unwrap();

  assert!(matches!(actor_clone.addresses().await.unwrap_err(), Error::Shutdown));
}

#[tokio::test]
async fn test_shutdown_returns_errors_through_open_channels() -> ActorResult<()> {
  try_init_logger();

  struct Actor;

  #[async_trait::async_trait]
  impl SyncActor<IdentityList> for Actor {
    async fn handle(&self, _: RequestContext<IdentityList>) -> Vec<IotaDID> {
      tokio::time::sleep(std::time::Duration::from_millis(50)).await;
      vec![]
    }
  }

  let (listening_actor, addrs, peer_id) = default_listening_actor(|mut builder| {
    builder.attach(Actor);
    builder
  })
  .await;

  let mut sending_actor: System = SystemBuilder::new().build().await.unwrap();
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
async fn test_endpoint_type_mismatch_result_in_serialization_errors() -> ActorResult<()> {
  try_init_logger();

  // Define two types with identical serialization results, but different `Response` types.
  // Sending `CustomRequest2` to an endpoint expecting `CustomRequest`, we expect a local deserialization error.

  #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
  pub struct CustomRequest(u8);

  #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
  pub struct CustomRequest2(u8);

  impl SyncActorRequest for CustomRequest {
    type Response = String;

    fn endpoint() -> Endpoint {
      "test/request".try_into().unwrap()
    }
  }

  impl SyncActorRequest for CustomRequest2 {
    type Response = u32;

    fn endpoint() -> Endpoint {
      "test/request".try_into().unwrap()
    }
  }

  struct Actor;

  #[async_trait::async_trait]
  impl SyncActor<CustomRequest2> for Actor {
    async fn handle(&self, _: RequestContext<CustomRequest2>) -> u32 {
      42
    }
  }

  let (listening_actor, addrs, peer_id) = default_listening_actor(|mut builder| {
    builder.attach(Actor);
    builder
  })
  .await;

  let mut sending_actor: System = SystemBuilder::new().build().await.unwrap();
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

  impl SyncActorRequest for CustomRequest3 {
    type Response = String;

    fn endpoint() -> Endpoint {
      "test/request".try_into().unwrap()
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
