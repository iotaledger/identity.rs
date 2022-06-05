// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_core::document::IotaDocument;

use crate::actor::Result as ActorResult;
use crate::tests::default_listening_system;
use crate::tests::default_sending_system;
use crate::tests::remote_account::IdentityCreate;
use crate::tests::remote_account::IdentityGet;
use crate::tests::remote_account::IdentityList;
use crate::tests::remote_account::RemoteAccount;
use crate::tests::try_init_logger;

#[tokio::test]
async fn test_remote_account() -> ActorResult<()> {
  try_init_logger();

  let (receiver, receiver_addrs, receiver_peer_id) = default_listening_system(|mut builder| {
    let remote_account = RemoteAccount::new().unwrap();
    builder.attach::<IdentityCreate, _>(remote_account.clone());
    builder.attach::<IdentityList, _>(remote_account.clone());
    builder.attach::<IdentityGet, _>(remote_account);
    builder
  })
  .await;
  let mut sender = default_sending_system(|builder| builder).await;

  sender.add_addresses(receiver_peer_id, receiver_addrs).await.unwrap();

  let doc: IotaDocument = sender.send_request(receiver_peer_id, IdentityCreate).await?.unwrap();

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
