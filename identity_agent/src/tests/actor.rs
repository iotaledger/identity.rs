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

use crate::agent::Actor;
use crate::agent::ActorRequest;
use crate::agent::Agent;
use crate::agent::AgentBuilder;
use crate::agent::Endpoint;
use crate::agent::Error;
use crate::agent::ErrorLocation;
use crate::agent::RequestContext;
use crate::agent::Result as AgentResult;
use crate::tests::default_listening_agent;
use crate::tests::default_sending_agent;
use crate::tests::remote_account::IdentityGet;
use crate::tests::remote_account::IdentityList;
use crate::tests::try_init_logger;

#[tokio::test]
async fn test_actor_end_to_end() -> AgentResult<()> {
  try_init_logger();

  #[derive(Debug, Clone)]
  struct MyActor {
    counter: Arc<AtomicU32>,
  }

  // Define our request types and implement ActorRequest for them.
  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  struct Increment(u32);

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  struct Decrement(u32);

  impl ActorRequest for Increment {
    type Response = u32;

    fn endpoint() -> Endpoint {
      "counter/increment".try_into().unwrap()
    }
  }

  impl ActorRequest for Decrement {
    type Response = u32;

    fn endpoint() -> Endpoint {
      "counter/decrement".try_into().unwrap()
    }
  }

  // States that MyActor can handle messages of type `Increment`.
  #[async_trait::async_trait]
  impl Actor<Increment> for MyActor {
    async fn handle(&self, request: RequestContext<Increment>) -> u32 {
      self.counter.fetch_add(request.input.0, Ordering::SeqCst);
      self.counter.load(Ordering::SeqCst)
    }
  }

  // States that MyActor can handle messages of type `Decrement`.
  #[async_trait::async_trait]
  impl Actor<Decrement> for MyActor {
    async fn handle(&self, request: RequestContext<Decrement>) -> u32 {
      self.counter.fetch_sub(request.input.0, Ordering::SeqCst);
      self.counter.load(Ordering::SeqCst)
    }
  }

  let actor = MyActor {
    counter: Arc::new(AtomicU32::new(0)),
  };

  // Create a new agent and attach the actor.
  // Each attachment is for one request type, so we have to do it twice.
  let mut builder = AgentBuilder::new();
  builder.attach::<Increment, _>(actor.clone());
  builder.attach::<Decrement, _>(actor.clone());

  // Build the listening agent and let it listen on a default address.
  let mut listening_agent: Agent = builder.build().await.unwrap();

  let _ = listening_agent
    .start_listening("/ip4/0.0.0.0/tcp/0".parse().unwrap())
    .await
    .unwrap();
  let addresses = listening_agent.addresses().await.unwrap();
  let agent_id = listening_agent.agent_id();

  let mut sender_agent: Agent = AgentBuilder::new().build().await.unwrap();
  // Add on which which addresses sender_agent can reach agent_id.
  sender_agent.add_agent_addresses(agent_id, addresses).await.unwrap();

  assert_eq!(sender_agent.send_request(agent_id, Increment(3)).await.unwrap(), 3);
  assert_eq!(sender_agent.send_request(agent_id, Decrement(2)).await.unwrap(), 1);

  listening_agent.shutdown().await.unwrap();
  sender_agent.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_unknown_request_returns_error() -> AgentResult<()> {
  try_init_logger();

  let (listening_actor, addrs, agent_id) = default_listening_agent(|builder| builder).await;

  let mut sending_actor = default_sending_agent(|builder| builder).await;
  sending_actor.add_agent_addresses(agent_id, addrs).await.unwrap();

  let result = sending_actor
    .send_request(
      agent_id,
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

/// Test that agent2 can send a request to agent1 if it was previously sent a request from agent1.
#[tokio::test]
async fn test_actors_can_communicate_bidirectionally() -> AgentResult<()> {
  try_init_logger();

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  struct Dummy(u8);

  impl ActorRequest for Dummy {
    type Response = ();

    fn endpoint() -> Endpoint {
      "request/test".try_into().unwrap()
    }
  }

  #[derive(Debug, Clone)]
  struct TestActor(Arc<AtomicBool>);

  #[async_trait::async_trait]
  impl Actor<Dummy> for TestActor {
    async fn handle(&self, _req: RequestContext<Dummy>) {
      self.0.store(true, std::sync::atomic::Ordering::SeqCst);
    }
  }

  let actor1 = TestActor(Arc::new(AtomicBool::new(false)));
  let actor2 = TestActor(Arc::new(AtomicBool::new(false)));

  let mut agent1_builder = AgentBuilder::new();
  agent1_builder.attach(actor1.clone());
  let mut agent1: Agent = agent1_builder.build().await.unwrap();

  let mut agent2_builder = AgentBuilder::new();
  agent2_builder.attach(actor2.clone());
  let mut agent2: Agent = agent2_builder.build().await.unwrap();

  agent2
    .start_listening("/ip4/0.0.0.0/tcp/0".try_into().unwrap())
    .await
    .unwrap();

  let addr: Multiaddr = agent2.addresses().await.unwrap().into_iter().next().unwrap();

  agent1.add_agent_address(agent2.agent_id(), addr).await.unwrap();

  agent1.send_request(agent2.agent_id(), Dummy(42)).await.unwrap();

  agent2.send_request(agent1.agent_id(), Dummy(43)).await.unwrap();

  agent1.shutdown().await.unwrap();
  agent2.shutdown().await.unwrap();

  assert!(actor1.0.load(std::sync::atomic::Ordering::SeqCst));
  assert!(actor2.0.load(std::sync::atomic::Ordering::SeqCst));

  Ok(())
}

#[tokio::test]
async fn test_interacting_with_shutdown_actor_returns_error() {
  try_init_logger();

  let (listening_actor, _, _) = default_listening_agent(|builder| builder).await;

  let mut actor_clone = listening_actor.clone();

  listening_actor.shutdown().await.unwrap();

  assert!(matches!(actor_clone.addresses().await.unwrap_err(), Error::Shutdown));
}

#[tokio::test]
async fn test_shutdown_returns_errors_through_open_channels() -> AgentResult<()> {
  try_init_logger();

  #[derive(Debug)]
  struct TestActor;

  #[async_trait::async_trait]
  impl Actor<IdentityList> for TestActor {
    async fn handle(&self, _: RequestContext<IdentityList>) -> Vec<IotaDID> {
      tokio::time::sleep(std::time::Duration::from_millis(50)).await;
      vec![]
    }
  }

  let (listening_agent, addrs, agent_id) = default_listening_agent(|mut builder| {
    builder.attach(TestActor);
    builder
  })
  .await;

  let mut sending_agent: Agent = AgentBuilder::new().build().await.unwrap();
  sending_agent.add_agent_addresses(agent_id, addrs).await.unwrap();

  let mut sender1 = sending_agent.clone();

  // Ensure that an actor shutdown returns errors through open channels,
  // such as `EventLoop::await_response`.
  // We do not test all `EventLoop::await*` fields, because some are
  // much harder to test than others.
  // We poll the futures once to ensure that the channels are created,
  // before shutting the actor down. If we would call these methods after shutdown,
  // they would immediately return a shutdown error (see test_interacting_with_shutdown_actor_returns_error),
  // hence the need for manual polling.
  // On the next poll after shutdown, we expect the errors.

  let send_request_future = sender1.send_request(agent_id, IdentityList);
  pin_mut!(send_request_future);
  let result = futures::poll!(&mut send_request_future);
  assert!(matches!(result, Poll::Pending));

  sending_agent.shutdown().await.unwrap();

  let result = send_request_future.await;
  assert!(matches!(
    result.unwrap_err(),
    Error::OutboundFailure(OutboundFailure::ConnectionClosed)
  ));

  listening_agent.shutdown().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_endpoint_type_mismatch_results_in_serialization_errors() -> AgentResult<()> {
  try_init_logger();

  // Define two types with identical serialization results, but different `Response` types.
  // Sending `CustomRequest2` to an endpoint expecting `CustomRequest`, we expect a local deserialization error.

  #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
  struct CustomRequest(u8);

  #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
  struct CustomRequest2(u8);

  impl ActorRequest for CustomRequest {
    type Response = String;

    fn endpoint() -> Endpoint {
      "test/request".try_into().unwrap()
    }
  }

  impl ActorRequest for CustomRequest2 {
    type Response = u32;

    fn endpoint() -> Endpoint {
      "test/request".try_into().unwrap()
    }
  }

  #[derive(Debug)]
  struct TestActor;

  #[async_trait::async_trait]
  impl Actor<CustomRequest2> for TestActor {
    async fn handle(&self, _: RequestContext<CustomRequest2>) -> u32 {
      42
    }
  }

  let (listening_actor, addrs, agent_id) = default_listening_agent(|mut builder| {
    builder.attach(TestActor);
    builder
  })
  .await;

  let mut sending_actor: Agent = AgentBuilder::new().build().await.unwrap();
  sending_actor.add_agent_addresses(agent_id, addrs).await.unwrap();

  let result = sending_actor.send_request(agent_id, CustomRequest(13)).await;

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
  struct CustomRequest3(String);

  impl ActorRequest for CustomRequest3 {
    type Response = String;

    fn endpoint() -> Endpoint {
      "test/request".try_into().unwrap()
    }
  }

  let result = sending_actor
    .send_request(agent_id, CustomRequest3("13".to_owned()))
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
