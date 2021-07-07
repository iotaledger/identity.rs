use std::sync::Arc;

use communication_refactored::{InitKeypair, Keypair};
use libp2p::tcp::TcpConfig;
use tokio::task;

use crate::{
  actor_builder::ActorBuilder,
  types::{IdentityStorageRequest, IdentityStorageResponse},
  IdentityStorageHandler,
};

#[tokio::test]
async fn test_list_identities() -> anyhow::Result<()> {
  let id_keys = Keypair::generate_ed25519();
  let transport = TcpConfig::new().nodelay(true);

  let comm = ActorBuilder::new()
    .keys(InitKeypair::IdKeys(id_keys))
    .build_with_transport(transport)
    .await?;

  let handler = IdentityStorageHandler::new().await?;
  comm.set_handler("IdentityStorage", handler);

  let addr = comm.addrs().pop().unwrap();
  let peer_id = comm.peer_id();

  let other_comm = ActorBuilder::new().build().await?;
  other_comm.add_peer(peer_id, addr);

  let sender = task::spawn(async move {
    // TODO: Let each request implement a trait that specifies the return type via an asssociated type
    let res = other_comm
      .send_command::<IdentityStorageResponse, _>(peer_id, IdentityStorageRequest::List)
      .await;

    res.unwrap()
  });

  let sender_result = sender.await.unwrap();

  assert!(matches!(sender_result, IdentityStorageResponse::List(vec) if vec.is_empty()));

  comm.stop_handling_requests().await.unwrap();

  Ok(())
}
