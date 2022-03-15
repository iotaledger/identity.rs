// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_actor::remote_account::RemoteAccount;
use identity_actor::Actor;
use identity_actor::ActorRequest;
use identity_actor::Multiaddr;
use identity_actor::RequestContext;
use identity_actor::Synchronous;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestRequest(String);

impl ActorRequest<Synchronous> for TestRequest {
  type Response = String;

  fn endpoint<'cow>(&self) -> std::borrow::Cow<'cow, str> {
    std::borrow::Cow::Borrowed("test/request")
  }
}

#[tokio::main]
async fn main() -> identity_actor::Result<()> {
  // pretty_env_logger::init();

  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/12345/ws".parse().unwrap();

  let mut actor_builder = identity_actor::ActorBuilder::new();

  actor_builder
    .add_state(())
    .add_sync_handler(
      "test/request",
      |_: (), _: Actor, request: RequestContext<TestRequest>| async move { request.input.0 },
    )
    .unwrap();

  let handler = RemoteAccount::new().unwrap();

  actor_builder
    .add_state(handler)
    .add_sync_handler("remote_account/list", RemoteAccount::list)?;

  let mut actor = actor_builder.build().await?;

  actor.start_listening(addr).await?;
  let addrs = actor.addresses().await?;

  let peer_id = actor.peer_id();

  println!("Listening on {:#?} with PeerId: {}", addrs, peer_id.to_base58());

  // Blocks forever
  futures::future::pending::<()>().await;

  Ok(())
}
