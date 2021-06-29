#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use async_std::task;
  use futures::future::join;
  use identity_iota::did::IotaDocument;
  use libp2p::{Multiaddr, PeerId};

  use crate::{
    communicator::DefaultIdentityCommunicator,
    types::{IdentityStorageRequest, IdentityStorageResponse},
    DefaultIdentityHandler,
  };

  use identity_account::Result;

  #[async_std::test]
  async fn test_list_identities() -> Result<()> {
    let handler = DefaultIdentityHandler::new().await;
    let mut comm = DefaultIdentityCommunicator::new(handler).await;
    // TODO: Handle error
    let addr = comm.start_listening(None).await.unwrap();
    let peer_id = comm.peer_id();

    let shared_comm = Arc::new(comm);
    let shared_clone = Arc::clone(&shared_comm);

    let _listener_handle = task::spawn(async move { shared_clone.handle_requests().await });

    let sender = task::spawn(async move {
      let handler = DefaultIdentityHandler::new().await;
      let comm = DefaultIdentityCommunicator::new(handler).await;

      comm
        .send_command::<IdentityStorageResponse, _>(addr, peer_id, IdentityStorageRequest::List)
        .await
        .unwrap()
    });

    let sender_result = sender.await;

    assert!(matches!(sender_result, IdentityStorageResponse::List(vec) if vec.is_empty()));

    // shared_comm.stop_listening();

    // listener_handle.await;

    Ok(())
  }
}
