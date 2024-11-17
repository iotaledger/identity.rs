// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use crate::common;
use crate::common::get_client as get_test_client;
use crate::common::get_key_data;
use crate::common::TEST_COIN_TYPE;
use crate::common::TEST_DOC;
use crate::common::TEST_GAS_BUDGET;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_iota_core::rebased::client::get_object_id_from_did;
use identity_iota_core::rebased::migration::has_previous_version;
use identity_iota_core::rebased::migration::Identity;
use identity_iota_core::rebased::proposals::ProposalResult;
use identity_iota_core::rebased::proposals::ProposalT as _;
use identity_iota_core::rebased::transaction::Transaction;
use identity_verification::MethodScope;
use identity_verification::VerificationMethod;
use iota_sdk::rpc_types::IotaObjectData;
use iota_sdk::types::base_types::SequenceNumber;
use iota_sdk::types::TypeTag;
use iota_sdk::types::IOTA_FRAMEWORK_PACKAGE_ID;
use move_core_types::ident_str;
use move_core_types::language_storage::StructTag;

#[tokio::test]
async fn updating_onchain_identity_did_doc_with_single_controller_works() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let identity_client = test_client.new_user_client().await?;

  let mut newly_created_identity = identity_client
    .create_identity(TEST_DOC)
    .finish()
    .execute_with_gas(TEST_GAS_BUDGET, &identity_client)
    .await?
    .output;

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
    .finish(&identity_client)
    .await?
    .execute(&identity_client)
    .await?
    .output;

  Ok(())
}

#[tokio::test]
async fn approving_proposal_works() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let alice_client = test_client.new_user_client().await?;
  let bob_client = test_client.new_user_client().await?;

  let mut identity = alice_client
    .create_identity(TEST_DOC)
    .controller(alice_client.sender_address(), 1)
    .controller(bob_client.sender_address(), 1)
    .threshold(2)
    .finish()
    .execute(&alice_client)
    .await?
    .output;

  let did_doc = {
    let did = IotaDID::parse(format!("did:iota:{}", identity.id()))?;
    let mut doc = IotaDocument::new_with_id(did.clone());
    doc.insert_method(
      VerificationMethod::new_from_jwk(
        did,
        alice_client.signer().public_key().clone(),
        Some(alice_client.signer().key_id().as_str()),
      )?,
      MethodScope::VerificationMethod,
    )?;
    doc
  };
  let ProposalResult::Pending(mut proposal) = identity
    .update_did_document(did_doc)
    .finish(&alice_client)
    .await?
    .execute(&alice_client)
    .await?
    .output
  else {
    anyhow::bail!("the proposal is executed");
  };

  proposal.approve(&identity).execute(&bob_client).await?;

  assert_eq!(proposal.votes(), 2);

  Ok(())
}

#[tokio::test]
async fn adding_controller_works() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let alice_client = test_client.new_user_client().await?;
  let bob_client = test_client.new_user_client().await?;

  let mut identity = alice_client
    .create_identity(TEST_DOC)
    .finish()
    .execute(&alice_client)
    .await?
    .output;

  // Alice proposes to add Bob as a controller. Since Alice has enough voting power the proposal
  // is executed directly after creation.
  identity
    .update_config()
    .add_controller(bob_client.sender_address(), 1)
    .finish(&alice_client)
    .await?
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

  let mut newly_created_identity = identity_client
    .create_identity(TEST_DOC)
    .finish()
    .execute_with_gas(TEST_GAS_BUDGET, &identity_client)
    .await?
    .output;

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
    .finish(&identity_client)
    .await?
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
    .collect::<Result<Vec<bool>, identity_iota_core::rebased::Error>>()?;
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

#[tokio::test]
async fn send_proposal_works() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let identity_client = test_client.new_user_client().await?;

  let mut identity = identity_client
    .create_identity(TEST_DOC)
    .finish()
    .execute_with_gas(TEST_GAS_BUDGET, &identity_client)
    .await?
    .output;
  let identity_address = identity.id().into();

  // Let's give the identity 2 coins in order to have something to move.
  let coin1 = common::get_test_coin(identity_address, &identity_client).await?;
  let coin2 = common::get_test_coin(identity_address, &identity_client).await?;

  // Let's propose the send of those two caps to the identity_client's address.
  let ProposalResult::Pending(send_proposal) = identity
    .send_assets()
    .object(coin1, identity_client.sender_address())
    .object(coin2, identity_client.sender_address())
    .finish(&identity_client)
    .await?
    .execute(&identity_client)
    .await?
    .output
  else {
    panic!("send proposal cannot be chain-executed!");
  };

  send_proposal
    .into_tx(&mut identity, &identity_client)
    .await?
    .execute(&identity_client)
    .await?;

  // Assert that identity_client's address now owns those coins.
  identity_client
    .find_owned_ref(TEST_COIN_TYPE.clone(), |obj| obj.object_id == coin1)
    .await?
    .expect("coin1 was transfered to this address");

  identity_client
    .find_owned_ref(TEST_COIN_TYPE.clone(), |obj| obj.object_id == coin2)
    .await?
    .expect("coin2 was transfered to this address");

  Ok(())
}

#[tokio::test]
async fn borrow_proposal_works() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let identity_client = test_client.new_user_client().await?;

  let mut identity = identity_client
    .create_identity(TEST_DOC)
    .finish()
    .execute(&identity_client)
    .await?
    .output;
  let identity_address = identity.id().into();

  let coin1 = common::get_test_coin(identity_address, &identity_client).await?;
  let coin2 = common::get_test_coin(identity_address, &identity_client).await?;

  // Let's propose the borrow of those two coins to the identity_client's address.
  let ProposalResult::Pending(borrow_proposal) = identity
    .borrow_assets()
    .borrow(coin1)
    .borrow(coin2)
    .finish(&identity_client)
    .await?
    .execute(&identity_client)
    .await?
    .output
  else {
    panic!("borrow proposal cannot be chain-executed!");
  };

  borrow_proposal
    .into_tx(&mut identity, &identity_client)
    .await?
    // this doesn't really do anything but if it doesn't fail it means coin1 was properly borrowed.
    .with_intent(move |ptb, objs| {
      ptb.programmable_move_call(
        IOTA_FRAMEWORK_PACKAGE_ID,
        ident_str!("coin").into(),
        ident_str!("value").into(),
        vec![TypeTag::Bool],
        vec![objs.get(&coin1).expect("coin1 data is borrowed").0],
      );
    })
    .execute(&identity_client)
    .await?;

  Ok(())
}
