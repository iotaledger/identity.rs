use std::str::FromStr;

use crate::common::get_client as get_test_client;
use crate::common::get_key_data;
use crate::common::TEST_DOC;
use crate::common::TEST_GAS_BUDGET;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_sui_name_tbd::client::get_object_id_from_did;
use identity_sui_name_tbd::migration::has_previous_version;
use identity_sui_name_tbd::migration::Identity;
use identity_sui_name_tbd::proposals::ProposalResult;
use identity_sui_name_tbd::transaction::Transaction;
use identity_sui_name_tbd::iota_sdk_abstraction::rpc_types::IotaObjectData;
use identity_sui_name_tbd::iota_sdk_abstraction::types::base_types::SequenceNumber;
use identity_sui_name_tbd::iota_sdk_abstraction::move_types::language_storage::StructTag;
use identity_verification::MethodScope;
use identity_verification::VerificationMethod;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn updating_onchain_identity_did_doc_with_single_controller_works() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let identity_client = test_client.new_user_client().await?;

  let newly_created_identity = identity_client
    .create_identity(TEST_DOC)
    .finish()
    .execute_with_gas(TEST_GAS_BUDGET, &identity_client)
    .await?;

  let updated_did_doc = {
    let did = IotaDID::parse(format!("did:iota:{}", newly_created_identity.id()))?;
    let mut doc = IotaDocument::new_with_id(did.clone());
    doc.insert_method(
      VerificationMethod::new_from_jwk(
        did,
        identity_client.signer().public_key().clone(),
        Some(identity_client.signer().key_id().as_str()),
      )?,
      MethodScope::VerificationMethod,
    )?;
    doc
  };

  newly_created_identity
    .update_did_document(updated_did_doc)
    .finish()
    .execute(&identity_client)
    .await?;

  Ok(())
}

#[tokio::test]
#[serial]
async fn approving_proposal_works() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let alice_client = test_client.new_user_client().await?;
  let bob_client = test_client.new_user_client().await?;

  let identity = alice_client
    .create_identity(TEST_DOC)
    .controller(alice_client.sender_address(), 1)
    .controller(bob_client.sender_address(), 1)
    .threshold(2)
    .finish()
    .execute(&alice_client)
    .await?;

  let did = IotaDID::parse(format!("did:iota:{}", identity.id().to_string()))?;
  let did_doc = {
    let mut doc = IotaDocument::new_with_id(did.clone());
    doc.insert_method(
      VerificationMethod::new_from_jwk(
        did.clone(),
        alice_client.signer().public_key().clone(),
        Some(alice_client.signer().key_id().as_str()),
      )?,
      MethodScope::VerificationMethod,
    )?;
    doc
  };
  let ProposalResult::Pending(mut proposal) = identity
    .update_did_document(did_doc)
    .finish()
    .execute(&alice_client)
    .await?
  else {
    anyhow::bail!("the proposal is executed");
  };

  let Identity::FullFledged(mut identity) = alice_client.get_identity(get_object_id_from_did(&did)?).await? else {
    anyhow::bail!("resolved identity should be an onchain identity");
  };

  proposal.approve(&mut identity).execute(&bob_client).await?;

  assert_eq!(proposal.votes(), 2);

  Ok(())
}

#[tokio::test]
#[serial]
async fn adding_controller_works() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let alice_client = test_client.new_user_client().await?;
  let bob_client = test_client.new_user_client().await?;

  let identity = alice_client
    .create_identity(TEST_DOC)
    .finish()
    .execute(&alice_client)
    .await?;

  // Alice proposes to add Bob as a controller. Since Alice has enough voting power the proposal
  // is executed directly after creation.
  identity
    .update_config()
    .add_controller(bob_client.sender_address(), 1)
    .finish()
    .execute(&alice_client)
    .await?;

  let cap = bob_client
    .find_owned_ref(
      StructTag::from_str(&format!("{}::multicontroller::ControllerCap", test_client.package_id())).unwrap(),
      |_| true,
    )
    .await?;

  assert!(cap.is_some());

  Ok(())
}

#[tokio::test]
async fn can_get_historical_identity_data() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let identity_client = test_client.new_user_client().await?;

  let newly_created_identity = identity_client
    .create_identity(TEST_DOC)
    .finish()
    .execute_with_gas(TEST_GAS_BUDGET, &identity_client)
    .await?;

  let did = IotaDID::parse(format!("did:iota:{}", newly_created_identity.id()))?;
  let updated_did_doc = {
    let mut doc = IotaDocument::new_with_id(did.clone());
    let (_, key_id, public_key_jwk, _) = get_key_data().await?;
    doc.insert_method(
      VerificationMethod::new_from_jwk(did.clone(), public_key_jwk, Some(key_id.as_str()))?,
      MethodScope::VerificationMethod,
    )?;
    doc
  };

  newly_created_identity
    .update_did_document(updated_did_doc)
    .finish()
    .execute_with_gas(TEST_GAS_BUDGET, &identity_client)
    .await?;

  let Identity::FullFledged(updated_identity) = identity_client.get_identity(get_object_id_from_did(&did)?).await?
  else {
    anyhow::bail!("resolved identity should be an onchain identity");
  };

  let history = updated_identity.get_history(&identity_client, None, None).await?;

  // test check for previous version
  let has_previous_version_responses: Vec<bool> = history
    .iter()
    .map(has_previous_version)
    .collect::<Result<Vec<bool>, identity_sui_name_tbd::Error>>()?;
  assert_eq!(has_previous_version_responses, vec![true, false]);

  let versions: Vec<SequenceNumber> = history.iter().map(|elem| elem.version).collect();
  let version_numbers: Vec<usize> = versions.iter().map(|v| (*v).into()).collect();
  let oldest_version: usize = *version_numbers.last().unwrap();
  let version_diffs: Vec<usize> = version_numbers.iter().map(|v| v - oldest_version).collect();
  assert_eq!(version_diffs, vec![1, 0],);

  // paging:
  //   you can either loop until no result is returned
  let mut result_index = 0;
  let mut current_item: Option<&IotaObjectData> = None;
  let mut history: Vec<IotaObjectData>;
  loop {
    history = updated_identity
      .get_history(&identity_client, current_item, Some(1))
      .await?;
    if history.is_empty() {
      break;
    }
    current_item = history.first();
    assert_eq!(current_item.unwrap().version, *versions.get(result_index).unwrap());
    result_index += 1;
  }

  //   or check before fetching next page
  let mut result_index = 0;
  let mut current_item: Option<&IotaObjectData> = None;
  let mut history: Vec<IotaObjectData>;
  loop {
    history = updated_identity
      .get_history(&identity_client, current_item, Some(1))
      .await?;

    current_item = history.first();
    assert_eq!(current_item.unwrap().version, *versions.get(result_index).unwrap());
    result_index += 1;

    if !has_previous_version(current_item.unwrap())? {
      break;
    }
  }

  Ok(())
}
