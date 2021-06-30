use std::sync::Arc;

use tokio::task;

use crate::{
  communicator::IdentityCommunicator,
  types::{IdentityStorageRequest, IdentityStorageResponse},
  IdentityStorageHandler,
};


#[tokio::test(flavor = "multi_thread")]
async fn test_list_identities() -> anyhow::Result<()> {
  let comm = IdentityCommunicator::new().await;

  let handler = IdentityStorageHandler::new().await?;
  comm.register_command("IdentityStorage", handler);

  let addr = comm.start_listening(None).await?;
  let peer_id = comm.peer_id();

  let shared_comm = Arc::new(comm);
  let shared_clone = Arc::clone(&shared_comm);

  let listener_handle = task::spawn(async move { shared_clone.handle_requests().await });

  let sender = task::spawn(async move {
    let other_comm = IdentityCommunicator::new().await;
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
