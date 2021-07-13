use communication_refactored::{InitKeypair, Keypair};
use libp2p::{tcp::TcpConfig, Multiaddr};

use crate::{actor_builder::ActorBuilder, asyncfn::AsyncFn, storage_handler::IdentityList, IdentityStorageHandler};

#[tokio::test]
async fn test_list_identities() -> anyhow::Result<()> {
  let id_keys = Keypair::generate_ed25519();
  let transport = TcpConfig::new().nodelay(true);

  let addr: Multiaddr = "/ip4/127.0.0.1/tcp/1337".parse().unwrap();

  let mut comm = ActorBuilder::new()
    .keys(InitKeypair::IdKeys(id_keys))
    .listen_on(addr.clone())
    .build_with_transport(transport)
    .await?;

  let handler = IdentityStorageHandler::new().await?;
  comm.add_handler_object(handler);

  comm.add_handler_method("storage/list", AsyncFn::new(IdentityStorageHandler::list));
  comm.add_handler_method("storage/resolve", AsyncFn::new(IdentityStorageHandler::resolve));

  let peer_id = comm.peer_id();

  let mut other_comm = ActorBuilder::new().build().await?;
  other_comm.add_peer(peer_id, addr).await;

  let result = other_comm.send_request(peer_id, IdentityList).await?;

  assert!(result.is_empty());

  comm.stop_handling_requests().await.unwrap();

  Ok(())
}
