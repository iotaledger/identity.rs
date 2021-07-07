use communication_refactored::{InitKeypair, Keypair};
use libp2p::tcp::TcpConfig;

use crate::{
  actor_builder::ActorBuilder,
  types::{StorageRequest, StorageResponse},
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
  comm.set_handler("Storage", handler);

  let addr = comm.addrs().pop().unwrap();
  let peer_id = comm.peer_id();

  let other_comm = ActorBuilder::new().build().await?;
  other_comm.add_peer(peer_id, addr);

  let result = other_comm.send_request(peer_id, StorageRequest::List).await?;

  assert!(matches!(result, StorageResponse::List(vec) if vec.is_empty()));

  comm.stop_handling_requests().await.unwrap();

  Ok(())
}
