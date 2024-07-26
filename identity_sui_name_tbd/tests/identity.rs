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
use identity_sui_name_tbd::utils::get_client as get_iota_client;
use identity_sui_name_tbd::utils::LOCAL_NETWORK;
use identity_verification::MethodScope;
use identity_verification::VerificationMethod;

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
