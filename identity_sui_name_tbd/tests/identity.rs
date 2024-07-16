mod common;

use common::get_client as get_test_client;
use common::request_funds;
use common::TEST_DOC;
use common::TEST_GAS_BUDGET;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_storage::JwkMemStore;
use identity_storage::JwkStorage;
use identity_storage::KeyIdMemstore;
use identity_storage::KeyType;
use identity_storage::Storage;
use identity_sui_name_tbd::client::IdentityClient;
use identity_sui_name_tbd::utils::get_client as get_iota_client;
use identity_sui_name_tbd::utils::LOCAL_NETWORK;
use identity_verification::jws::JwsAlgorithm;
use identity_verification::MethodScope;
use identity_verification::VerificationMethod;

pub type MemStorage = Storage<JwkMemStore, KeyIdMemstore>;

#[tokio::test]
async fn updating_onchain_identity_did_doc_with_single_controller_works() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let iota_client = get_iota_client(LOCAL_NETWORK).await?;
  let storage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());

  // generate new key
  let generate = storage
    .key_storage()
    .generate(KeyType::new("Ed25519"), JwsAlgorithm::EdDSA)
    .await?;
  let public_key = generate.jwk.to_public().expect("public components should be derivable");

  let identity_client = IdentityClient::builder()
    .identity_iota_package_id(test_client.package_id())
    .sender_key_id(generate.key_id.clone())
    .sender_public_jwk(public_key.clone())
    .storage(storage)
    .iota_client(iota_client)
    .build()?;

  request_funds(&identity_client.sender_address()?).await?;

  let mut newly_created_identity = identity_client
    .create_identity(TEST_DOC)
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&identity_client)
    .await?;

  let updated_did_doc = {
    let did = IotaDID::parse(format!(
      "did:iota:{}",
      newly_created_identity.id.object_id().to_string()
    ))?;
    let mut doc = IotaDocument::new_with_id(did.clone());
    doc.insert_method(
      VerificationMethod::new_from_jwk(did, public_key, Some(generate.key_id.as_str()))?,
      MethodScope::VerificationMethod,
    )?;
    doc
  };

  newly_created_identity
    .update_did_document(updated_did_doc)
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&identity_client)
    .await?;

  Ok(())
}
