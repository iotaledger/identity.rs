// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_iota_core::NetworkName;

use crate::agent::Result as AgentResult;
use crate::tests::default_listening_agent;
use crate::tests::default_sending_agent;
use crate::tests::remote_account::IdentityCreate;
use crate::tests::remote_account::IdentityGet;
use crate::tests::remote_account::IdentityList;
use crate::tests::remote_account::RemoteAccount;
use crate::tests::try_init_logger;

#[tokio::test]
async fn test_remote_account() -> AgentResult<()> {
  try_init_logger();

  let (receiver, receiver_addrs, receiver_agent_id) = default_listening_agent(|mut builder| {
    let remote_account = RemoteAccount::new();
    builder.attach::<IdentityCreate, _>(remote_account.clone());
    builder.attach::<IdentityList, _>(remote_account.clone());
    builder.attach::<IdentityGet, _>(remote_account);
    builder
  })
  .await;
  let mut sender = default_sending_agent(|builder| builder).await;

  sender
    .add_agent_addresses(receiver_agent_id, receiver_addrs)
    .await
    .unwrap();

  let doc = fake_document();

  sender
    .send_request(receiver_agent_id, IdentityCreate(doc.clone()))
    .await?
    .unwrap();

  assert_eq!(sender.send_request(receiver_agent_id, IdentityList).await?.len(), 1);

  let doc2: IotaDocument = sender
    .send_request(receiver_agent_id, IdentityGet(doc.id().clone()))
    .await?
    .unwrap();

  assert_eq!(doc, doc2);

  sender.shutdown().await.unwrap();
  receiver.shutdown().await.unwrap();

  Ok(())
}

fn fake_document() -> IotaDocument {
  let rand_bytes: [u8; 32] = rand::random();
  let network_name = NetworkName::try_from("iota").unwrap();
  let mut did = IotaDID::new(&rand_bytes, &network_name);
  let mut doc = IotaDocument::new(&network_name);
  // Let's act as if this was a published IotaDocument for testing purposes.
  std::mem::swap(doc.core_document_mut().id_mut_unchecked(), &mut did);
  doc
}
