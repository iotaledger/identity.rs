mod common;

use common::get_client as get_test_client;
use common::get_key_data;
use common::request_funds;
use common::TEST_DOC;
use common::TEST_GAS_BUDGET;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_storage::JwkMemStore;
use identity_storage::KeyIdMemstore;
use identity_storage::Storage;
use identity_storage::StorageSigner;
use identity_sui_name_tbd::client::IdentityClient;
use identity_sui_name_tbd::migration::has_previous_version;
use identity_sui_name_tbd::utils::get_client as get_iota_client;
use identity_sui_name_tbd::utils::LOCAL_NETWORK;
use identity_sui_name_tbd::Error;
use identity_verification::MethodScope;
use identity_verification::VerificationMethod;
use iota_sdk::rpc_types::IotaObjectData;
use iota_sdk::types::base_types::SequenceNumber;

pub type MemStorage = Storage<JwkMemStore, KeyIdMemstore>;

#[tokio::test]
async fn updating_onchain_identity_did_doc_with_single_controller_works() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let iota_client = get_iota_client(LOCAL_NETWORK).await?;

  let (storage, key_id, public_key_jwk, public_key_bytes) = get_key_data().await?;
  let signer = StorageSigner::new(&storage, key_id.clone(), public_key_jwk.clone());

  let identity_client = IdentityClient::builder()
    .identity_iota_package_id(test_client.package_id())
    .sender_public_key(&public_key_bytes)
    .iota_client(iota_client)
    .build()?;

  request_funds(&identity_client.sender_address()?).await?;

  let mut newly_created_identity = identity_client
    .create_identity(TEST_DOC)
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&identity_client, &signer)
    .await?;

  let updated_did_doc = {
    let did = IotaDID::parse(format!("did:iota:{}", newly_created_identity.id.object_id()))?;
    let mut doc = IotaDocument::new_with_id(did.clone());
    doc.insert_method(
      VerificationMethod::new_from_jwk(did, public_key_jwk, Some(key_id.as_str()))?,
      MethodScope::VerificationMethod,
    )?;
    doc
  };

  newly_created_identity
    .update_did_document(updated_did_doc)
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&identity_client, &signer)
    .await?;

  Ok(())
}

#[tokio::test]
async fn can_get_historical_identity_data() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let iota_client = get_iota_client(LOCAL_NETWORK).await?;

  let (storage, key_id, public_key_jwk, public_key_bytes) = get_key_data().await?;
  let signer = StorageSigner::new(&storage, key_id.clone(), public_key_jwk.clone());

  let identity_client = IdentityClient::builder()
    .identity_iota_package_id(test_client.package_id())
    .sender_public_key(&public_key_bytes)
    .iota_client(iota_client)
    .build()?;

  request_funds(&identity_client.sender_address()?).await?;

  let mut newly_created_identity = identity_client
    .create_identity(TEST_DOC)
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&identity_client, &signer)
    .await?;

  let updated_did_doc = {
    let did = IotaDID::parse(format!("did:iota:{}", newly_created_identity.id.object_id()))?;
    let mut doc = IotaDocument::new_with_id(did.clone());
    doc.insert_method(
      VerificationMethod::new_from_jwk(did, public_key_jwk, Some(key_id.as_str()))?,
      MethodScope::VerificationMethod,
    )?;
    doc
  };

  newly_created_identity
    .update_did_document(updated_did_doc)
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&identity_client, &signer)
    .await?;

  let history = newly_created_identity.get_history(&identity_client, None, None).await?;

  // test check for previous version
  let has_previous_version_responses: Vec<bool> = history
    .iter()
    .map(has_previous_version)
    .collect::<Result<Vec<bool>, Error>>()?;
  assert_eq!(has_previous_version_responses, vec![true, true, false]);

  // test version numbers
  // 1 create version, shared with version 3, then 2 updates, sorted from new to old
  let expected_versions = vec![
    SequenceNumber::from_u64(5),
    SequenceNumber::from_u64(4),
    SequenceNumber::from_u64(3),
  ];
  let versions: Vec<SequenceNumber> = history.iter().map(|elem| elem.version).collect();
  assert_eq!(versions, expected_versions,);

  // paging:
  //   you can either loop until no result is returned
  let mut result_index = 0;
  let mut current_item: Option<&IotaObjectData> = None;
  let mut history: Vec<IotaObjectData>;
  loop {
    history = newly_created_identity
      .get_history(&identity_client, current_item, Some(1))
      .await?;
    if history.is_empty() {
      break;
    }
    current_item = history.first();
    assert_eq!(
      current_item.unwrap().version,
      *expected_versions.get(result_index).unwrap()
    );
    result_index += 1;
  }

  //   or check before fetching next page
  let mut result_index = 0;
  let mut current_item: Option<&IotaObjectData> = None;
  let mut history: Vec<IotaObjectData>;
  loop {
    history = newly_created_identity
      .get_history(&identity_client, current_item, Some(1))
      .await?;

    current_item = history.first();
    assert_eq!(
      current_item.unwrap().version,
      *expected_versions.get(result_index).unwrap()
    );
    result_index += 1;

    if !has_previous_version(current_item.unwrap())? {
      break;
    }
  }

  Ok(())
}
