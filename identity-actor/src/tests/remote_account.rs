// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account::types::IdentitySetup;
use identity_iota_core::document::IotaDocument;

use crate::actor::Result as ActorResult;
use crate::remote_account::IdentityCreate;
use crate::remote_account::IdentityGet;
use crate::remote_account::IdentityList;
use crate::remote_account::RemoteAccount;
use crate::tests::default_listening_actor;
use crate::tests::default_sending_actor;
use crate::tests::try_init_logger;

#[tokio::test]
async fn test_remote_account() -> ActorResult<()> {
  try_init_logger();

  let (receiver, receiver_addrs, receiver_peer_id) = default_listening_actor(|mut builder| {
    builder
      .add_state(RemoteAccount::new().unwrap())
      .add_sync_handler(RemoteAccount::create)
      .add_sync_handler(RemoteAccount::list)
      .add_sync_handler(RemoteAccount::get);
    builder
  })
  .await;
  let mut sender = default_sending_actor(|builder| builder).await;

  sender.add_addresses(receiver_peer_id, receiver_addrs).await.unwrap();

  let doc: IotaDocument = sender
    .send_request(receiver_peer_id, IdentityCreate(IdentitySetup::new()))
    .await?
    .unwrap();

  assert_eq!(sender.send_request(receiver_peer_id, IdentityList).await?.len(), 1);

  let doc2: IotaDocument = sender
    .send_request(receiver_peer_id, IdentityGet(doc.id().clone()))
    .await?
    .unwrap();

  assert_eq!(doc, doc2);

  sender.shutdown().await.unwrap();
  receiver.shutdown().await.unwrap();

  Ok(())
}
