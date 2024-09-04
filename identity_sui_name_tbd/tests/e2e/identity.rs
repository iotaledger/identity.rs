use std::str::FromStr;

use crate::common::get_client as get_test_client;
use crate::common::TEST_DOC;
use crate::common::TEST_GAS_BUDGET;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_sui_name_tbd::proposals::ProposalResult;
use identity_verification::MethodScope;
use identity_verification::VerificationMethod;
use move_core_types::language_storage::StructTag;

#[tokio::test]
async fn updating_onchain_identity_did_doc_with_single_controller_works() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let identity_client = test_client.new_user_client().await?;

  let mut newly_created_identity = identity_client
    .create_identity(TEST_DOC)
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&identity_client)
    .await?;

  let updated_did_doc = {
    let did = IotaDID::parse(format!("did:iota:{}", newly_created_identity.id.object_id()))?;
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
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&identity_client)
    .await?;

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
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&alice_client)
    .await?;

  let did_doc = {
    let did = IotaDID::parse(format!("did:iota:{}", identity.id.object_id().to_string()))?;
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
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&alice_client)
    .await?
  else {
    anyhow::bail!("the proposal is executed");
  };

  proposal.approve(TEST_GAS_BUDGET, &mut identity, &bob_client).await?;

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
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&alice_client)
    .await?;

  // Alice proposes to add Bob as a controller. Since Alice has enough voting power the proposal
  // is executed directly after creation.
  identity
    .update_config()
    .add_controller(bob_client.sender_address(), 1)
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&alice_client)
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
