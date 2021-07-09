use communication_refactored::{InitKeypair, Keypair};
use libp2p::{tcp::TcpConfig, Multiaddr};

use crate::{actor_builder::ActorBuilder, storage_handler::List, IdentityStorageHandler};

#[tokio::test]
async fn test_list_identities() -> anyhow::Result<()> {
  let id_keys = Keypair::generate_ed25519();
  let transport = TcpConfig::new().nodelay(true);

  let addr: Multiaddr = "/ip4/127.0.0.1/tcp/10000".parse().unwrap();

  let comm = ActorBuilder::new()
    .keys(InitKeypair::IdKeys(id_keys))
    .listen_on(addr.clone())
    .build_with_transport(transport)
    .await?;

  let handler = IdentityStorageHandler::new().await?;
  comm.set_handler_type(handler);

  comm.set_handler("storage/list", IdentityStorageHandler::list);
  comm.set_handler("storage/resolve", IdentityStorageHandler::resolve);

  // let addr = comm.addrs().await.pop().unwrap();
  let peer_id = comm.peer_id();

  let mut other_comm = ActorBuilder::new().build().await?;
  other_comm.add_peer(peer_id, addr).await;

  let result = other_comm.send_request(peer_id, List).await?;

  assert!(result.is_empty());

  comm.stop_handling_requests().await.unwrap();

  Ok(())
}
