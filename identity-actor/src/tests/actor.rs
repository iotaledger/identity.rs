// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libp2p::tcp::TcpConfig;

use crate::{
  actor_builder::ActorBuilder, errors::Error, traits::ActorRequest, types::RequestContext, Actor, IdentityResolve,
};

use super::{default_listening_actor, default_sending_actor};

#[tokio::test]
async fn test_unknown_request() -> anyhow::Result<()> {
  pretty_env_logger::init();

  let (listening_actor, addr, peer_id) = default_listening_actor().await;

  let mut sending_actor = default_sending_actor().await;
  sending_actor.add_peer(peer_id, addr).await;

  let request_name = "unknown/request";

  let result = sending_actor
    .send_named_request(
      peer_id,
      request_name,
      IdentityResolve::new("did:iota:FFFAH6qct9KGQcSenG1iaw2Nj9jP7Zmug2zcmTpF4942".parse().unwrap()),
    )
    .await;

  assert!(matches!(result.unwrap_err(), Error::UnknownRequest(_)));

  listening_actor.stop_handling_requests().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_actors_can_communicate_bidirectionally() -> crate::errors::Result<()> {
  let transport1 = TcpConfig::new().nodelay(true);
  let transport2 = TcpConfig::new().nodelay(true);

  let mut actor1 = ActorBuilder::new().build_with_transport(transport1).await.unwrap();
  let mut actor2 = ActorBuilder::new().build_with_transport(transport2).await.unwrap();

  let addr = actor2
    .start_listening("/ip4/0.0.0.0/tcp/0".parse().unwrap())
    .await
    .unwrap();

  #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
  pub struct Dummy;

  impl ActorRequest for Dummy {
    type Response = ();

    fn request_name<'cow>(&self) -> std::borrow::Cow<'cow, str> {
      std::borrow::Cow::Borrowed("request/test")
    }
  }

  #[derive(Clone)]
  pub struct State;

  impl State {
    async fn handler(self, _actor: Actor, _req: RequestContext<Dummy>) {}
  }

  actor1
    .add_state(State)
    .add_handler("request/test", State::handler)
    .unwrap();

  actor2
    .add_state(State)
    .add_handler("request/test", State::handler)
    .unwrap();

  actor1.add_peer(actor2.peer_id(), addr).await;

  actor1.send_request(actor2.peer_id(), Dummy).await.unwrap();

  actor2.send_request(actor1.peer_id(), Dummy).await.unwrap();

  actor1.stop_handling_requests().await.unwrap();
  actor2.stop_handling_requests().await.unwrap();

  Ok(())
}
