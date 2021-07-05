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
    .await;

  let handler = IdentityStorageHandler::new().await?;
  comm.set_handler("IdentityStorage", handler);

  let addr = comm.start_listening(None).await?;
  let peer_id = comm.peer_id();

  let shared_comm = Arc::new(comm);
  let shared_clone = Arc::clone(&shared_comm);

  let listener_handle = task::spawn(async move { shared_clone.handle_requests().await });

  let sender = task::spawn(async move {
    let other_comm = ActorBuilder::new().build().await;
    other_comm.add_peer(peer_id, addr);

    let res = other_comm
      .send_command::<IdentityStorageResponse, _>(peer_id, IdentityStorageRequest::List)
      .await;

    res.unwrap()
  });

  let sender_result = sender.await.unwrap();

  assert!(matches!(sender_result, IdentityStorageResponse::List(vec) if vec.is_empty()));

  listener_handle.abort();
  let _ = listener_handle.await;

  Ok(())
}
