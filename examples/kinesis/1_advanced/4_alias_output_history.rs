// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use examples_kinesis::create_kinesis_did_document;
use examples_kinesis::get_client_and_create_account;
use examples_kinesis::get_memstorage;
use examples_kinesis::TEST_GAS_BUDGET;
use identity_iota::core::json;
use identity_iota::core::FromJson;
use identity_iota::core::Timestamp;
use identity_iota::did::DID;
use identity_iota::document::Service;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::verification::MethodRelationship;
use identity_storage::StorageSigner;
use identity_sui_name_tbd::client::get_object_id_from_did;
use identity_sui_name_tbd::migration::Identity;

/// Demonstrates how to obtain the alias output history.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new client to interact with the IOTA ledger.
  // NOTE: a permanode is required to fetch older output histories.
  let storage = get_memstorage()?;
  let (identity_client, key_id, public_key_jwk) = get_client_and_create_account(&storage).await?;
  // create new signer that will be used to sign tx with
  let signer = StorageSigner::new(&storage, key_id, public_key_jwk);
  // create new DID document and publish it
  let (document, vm_fragment_1) = create_kinesis_did_document(&identity_client, &storage, &signer).await?;
  let did: IotaDID = document.id().clone();

  // Resolve the latest state of the document.
  let mut document: IotaDocument = identity_client.resolve_did(&did).await?;

  // Attach a new method relationship to the existing method.
  document.attach_method_relationship(
    &document.id().to_url().join(format!("#{vm_fragment_1}"))?,
    MethodRelationship::Authentication,
  )?;

  // Adding multiple services.
  let services = [
    json!({"id": document.id().to_url().join("#my-service-0")?, "type": "MyService", "serviceEndpoint": "https://iota.org/"}),
  ];
  for service in services {
    let service: Service = Service::from_json_value(service)?;
    assert!(document.insert_service(service).is_ok());
    document.metadata.updated = Some(Timestamp::now_utc());

    identity_client
      .publish_did_document_update(document.clone(), TEST_GAS_BUDGET, &signer)
      .await?;
  }

  // ====================================
  // Retrieving the Alias Output History
  // ====================================

  // Step 1 - Get the latest identity
  let identity = identity_client.get_identity(get_object_id_from_did(&did)?).await?;
  let onchain_identity = if let Identity::FullFledged(value) = identity {
    value
  } else {
    anyhow::bail!("history only available for onchain identities");
  };

  // Step 2 - Get history
  let history = onchain_identity.get_history(&identity_client, None, None).await?;
  println!("Alias History: {history:?}");

  Ok(())
}
