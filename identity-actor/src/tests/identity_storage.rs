#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use async_std::task;
  use futures::future::join;
  use identity_iota::did::IotaDocument;
  use libp2p::{Multiaddr, PeerId};

  use crate::{
    communicator::IdentityCommunicator,
    types::{IdentityStorageRequest, IdentityStorageResponse},
    IdentityStorageHandler,
  };

  use identity_account::Result;

  #[async_std::test]
  async fn test_list_identities() -> Result<()> {
    let handler = IdentityStorageHandler::new().await?;
    let mut comm = IdentityCommunicator::new().await;

    comm.register_command("IdentityStorage", handler);

    // TODO: Handle error
    let addr = comm.start_listening(None).await.unwrap();
    let peer_id = comm.peer_id();

    let shared_comm = Arc::new(comm);
    let shared_clone = Arc::clone(&shared_comm);

    let _listener_handle = task::spawn(async move { shared_clone.handle_requests().await });

    let sender = task::spawn(async move {
      let other_comm = IdentityCommunicator::new().await;
      other_comm.add_peer(peer_id, addr);

      let res = other_comm
        .send_command::<IdentityStorageResponse, _>(peer_id, IdentityStorageRequest::List)
        .await;

      res.unwrap()
    });

    let sender_result = sender.await;

    assert!(matches!(sender_result, IdentityStorageResponse::List(vec) if vec.is_empty()));

    // shared_comm.stop_listening();

    // listener_handle.await;

    Ok(())
  }
}
