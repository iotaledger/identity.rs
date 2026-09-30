// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use crate::common;
use crate::common::get_funded_test_client;
use crate::common::TestClient;
use identity_iota_core::rebased::migration::get_identity;
use identity_iota_core::rebased::migration::ControllerToken;
use identity_iota_core::rebased::proposals::ProposalResult;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_jose::jwk::ToJwk as _;
use identity_verification::MethodScope;
use identity_verification::VerificationMethod;
use iota_sdk::transaction_builder::TransactionBuilder;
use iota_sdk::types::ObjectId;
use iota_sdk::types::TypeTag;
use product_core::operation::Operation;
use product_core::product_client::ProductClient as _;
use secret_storage::iota::TransactionSigner as _;

#[tokio::test]
async fn identity_deactivation_works() -> anyhow::Result<()> {
  let test_client = get_funded_test_client().await?;
  let signer = test_client.new_signer().await?;

  let mut identity = test_client
    .create_identity(IotaDocument::new(test_client.network()))
    .finish()
    .execute(&signer, &test_client)
    .await?
    .output;

  let controller_token = identity
    .get_controller_token(signer.address(), &test_client)
    .await?
    .expect("this address is a controller");

  identity
    .deactivate_did(&controller_token)
    .finish(&test_client)
    .await?
    .execute(&signer, &test_client)
    .await?;

  assert!(identity.did_document().metadata.deactivated == Some(true));

  Ok(())
}

#[tokio::test]
async fn updating_onchain_identity_did_doc_with_single_controller_works() -> anyhow::Result<()> {
  let test_client = get_funded_test_client().await?;
  let signer = test_client.new_signer().await?;

  let mut newly_created_identity = test_client
    .create_identity(IotaDocument::new(test_client.network()))
    .finish()
    .execute(&signer, &test_client)
    .await?
    .output;

  let updated_did_doc = {
    let did = IotaDID::parse(format!("did:iota:{}", newly_created_identity.id()))?;
    let mut doc = IotaDocument::new_with_id(did.clone());
    doc.insert_method(
      VerificationMethod::new_from_jwk(did, signer.public_key().to_jwk()?, Some(signer.key_id().as_str()))?,
      MethodScope::VerificationMethod,
    )?;
    doc
  };

  let controller_token = newly_created_identity
    .get_controller_token(signer.address(), &test_client)
    .await?
    .expect("this address is a controller");

  newly_created_identity
    .update_did_document(updated_did_doc, &controller_token)
    .finish(&test_client)
    .await?
    .execute(&signer, &test_client)
    .await?;

  Ok(())
}

#[tokio::test]
async fn approving_proposal_works() -> anyhow::Result<()> {
  let test_client = get_funded_test_client().await?;
  let alice_signer = test_client.new_signer().await?;
  let bob_signer = test_client.new_signer().await?;

  let mut identity = test_client
    .create_identity(IotaDocument::new(test_client.network()))
    .controller(alice_signer.address(), 1)
    .controller(bob_signer.address(), 1)
    .threshold(2)
    .finish()
    .execute(&alice_signer, &test_client)
    .await?
    .output;
  let did_doc = {
    let did = IotaDID::parse(format!("did:iota:{}", identity.id()))?;
    let mut doc = IotaDocument::new_with_id(did.clone());
    doc.insert_method(
      VerificationMethod::new_from_jwk(
        did,
        alice_signer.public_key().to_jwk()?,
        Some(alice_signer.key_id().as_str()),
      )?,
      MethodScope::VerificationMethod,
    )?;
    doc
  };

  let alice_token = identity
    .get_controller_token(alice_signer.address(), &test_client)
    .await?
    .expect("alice is a controller");
  let ProposalResult::Pending(mut proposal) = identity
    .update_did_document(did_doc, &alice_token)
    .finish(&test_client)
    .await?
    .execute(&alice_signer, &test_client)
    .await?
    .output
  else {
    anyhow::bail!("the proposal is executed");
  };
  let bob_token = identity
    .get_controller_token(bob_signer.address(), &test_client)
    .await?
    .expect("bob is a controller");
  proposal
    .approve(&identity, &bob_token)?
    .execute(&bob_signer, &test_client)
    .await?;

  assert_eq!(proposal.votes(), 2);

  Ok(())
}

#[tokio::test]
async fn adding_controller_works() -> anyhow::Result<()> {
  let test_client = get_funded_test_client().await?;
  let alice_signer = test_client.new_signer().await?;
  let bob_signer = test_client.new_signer().await?;

  let mut identity = test_client
    .create_identity(IotaDocument::new(test_client.network()))
    .finish()
    .execute(&alice_signer, &test_client)
    .await?
    .output;

  let alice_token = identity
    .get_controller_token(alice_signer.address(), &test_client)
    .await?
    .expect("alice is a controller");
  // Alice proposes to add Bob as a controller. Since Alice has enough voting power the proposal
  // is executed directly after creation.
  identity
    .update_config(&alice_token)
    .add_controller(bob_signer.address(), 1)
    .finish(&test_client)
    .await?
    .execute(&alice_signer, &test_client)
    .await?;

  let _bob_token = identity
    .get_controller_token(bob_signer.address(), &test_client)
    .await?
    .expect("bob is a controller");

  Ok(())
}

#[tokio::test]
async fn send_proposal_works() -> anyhow::Result<()> {
  let test_client = get_funded_test_client().await?;
  let signer = test_client.new_signer().await?;

  let mut identity = test_client
    .create_identity(IotaDocument::new(test_client.network()))
    .finish()
    .execute(&signer, &test_client)
    .await?
    .output;
  let identity_address = identity.id().into();
  let token = identity
    .get_controller_token(signer.address(), &test_client)
    .await?
    .expect("is a controller");

  // Let's give the identity 2 coins in order to have something to move.
  let coin1 = common::get_test_coin(identity_address, &signer, &test_client).await?;
  let coin2 = common::get_test_coin(identity_address, &signer, &test_client).await?;

  // Let's propose the send of those two coins to the signer's address.
  let ProposalResult::Executed(_) = identity
    .send_assets(&token)
    .object(coin1, signer.address())
    .object(coin2, signer.address())
    .finish(&test_client)
    .await?
    .execute(&signer, &test_client)
    .await?
    .output
  else {
    panic!("the controller has enough voting power and the proposal should have been executed");
  };

  // Assert that signer's address now owns those coins.
  assert!(
    test_client
      .object(coin1, None)
      .await?
      .expect("coin1 was transfered to this address")
      .owner
      .as_address()
      == &signer.address()
  );
  assert!(
    test_client
      .object(coin2, None)
      .await?
      .expect("coin2 was transfered to this address")
      .owner
      .as_address()
      == &signer.address()
  );

  Ok(())
}

#[tokio::test]
async fn borrow_proposal_works() -> anyhow::Result<()> {
  let test_client = get_funded_test_client().await?;
  let signer = test_client.new_signer().await?;

  let mut identity = test_client
    .create_identity(IotaDocument::new(test_client.network()))
    .finish()
    .execute(&signer, &test_client)
    .await?
    .output;
  let identity_address = identity.id().into();

  let token = identity
    .get_controller_token(signer.address(), &test_client)
    .await?
    .expect("is a controller");

  let coin1 = common::get_test_coin(identity_address, &signer, &test_client).await?;
  let coin2 = common::get_test_coin(identity_address, &signer, &test_client).await?;

  // Let's propose the borrow of those two coins to the signer's address.
  let ProposalResult::Executed(_) = identity
    .borrow_assets(&token)
    .borrow(coin1)
    .borrow(coin2)
    .with_intent(move |ptb, objs| {
      ptb
        .move_call(ObjectId::FRAMEWORK, "coin", "value")
        .type_tags([TypeTag::Bool])
        .arguments([objs.get(&coin1).unwrap().0]);
    })
    .finish(&test_client)
    .await?
    .execute(&signer, &test_client)
    .await?
    .output
  else {
    panic!("controller has enough voting power and proposal should have been executed");
  };

  Ok(())
}

#[tokio::test]
async fn identity_delete_did_works() -> anyhow::Result<()> {
  let client = get_funded_test_client().await?;
  let signer = client.new_signer().await?;
  let mut identity = client
    .create_identity(IotaDocument::new(client.network()))
    .finish()
    .execute(&signer, &client)
    .await?
    .output;
  let did = identity.did_document().id().clone();
  let controller_token = identity
    .get_controller_token(signer.address(), &client)
    .await?
    .expect("is a controller");

  let ProposalResult::Executed(_) = identity
    .delete_did(&controller_token)
    .finish(&client)
    .await?
    .execute(&signer, &client)
    .await?
    .output
  else {
    anyhow::bail!("proposal should have been executed right away!");
  };

  assert!(identity.has_deleted_did());
  assert_eq!(identity.did_document().metadata.deactivated, Some(true));

  // Trying to update a deleted DID Document must fail.
  let err = identity
    .update_did_document(IotaDocument::new(client.network()), &controller_token)
    .finish(&client)
    .await;
  assert!(matches!(err, Err(identity_iota_core::rebased::Error::Identity(_))));

  // Resolution of the DID document through its DID must fail.
  let err = client.resolve_did(&did).await.unwrap_err();
  assert!(matches!(err, identity_iota_core::rebased::Error::DIDResolutionError(_)));

  Ok(())
}

#[tokio::test]
async fn controller_delegation_works() -> anyhow::Result<()> {
  let test_client = TestClient::new().await?;
  let alice_signer = test_client.new_signer().await?;
  let bob_signer = test_client.new_signer().await?;

  // We create an identity with two controllers, one can delegate the other cannot.
  let mut identity = test_client
    .create_identity(IotaDocument::new(test_client.network()))
    .controller(alice_signer.address(), 1)
    .controller_with_delegation(bob_signer.address(), 1)
    .threshold(2)
    .finish()
    .execute(&alice_signer, &test_client)
    .await?
    .output;

  let alice_token = identity
    .get_controller_token(alice_signer.address(), &test_client)
    .await?
    .expect("alice is a controller");
  assert!(!alice_token.as_controller().unwrap().can_delegate());
  let bob_token = identity
    .get_controller_token(bob_signer.address(), &test_client)
    .await?
    .expect("bob is a controller");
  assert!(bob_token.as_controller().unwrap().can_delegate());

  // Bob delegates its token with full-permissions to Alice.
  let bobs_delegation_token = bob_token
    .as_controller()
    .expect("bob's token is a controller cap")
    .delegate(alice_signer.address(), None)
    .expect("bob can delegate its token")
    .execute(&bob_signer, &test_client)
    .await?
    .output;
  assert!(
    bobs_delegation_token.controller() == bob_token.id() && bobs_delegation_token.controller_of() == identity.id()
  );
  // Ensure Alice is the owner of bob's delegation token.
  let delegation_token_owner = test_client
    .object(bobs_delegation_token.id(), None)
    .await?
    .expect("delegation token exists")
    .owner;
  assert_eq!(*delegation_token_owner.as_address(), alice_signer.address());

  // Alice can interact with the Identity in Bob's stead.
  let bobs_delegation_token = ControllerToken::Delegate(bobs_delegation_token);
  let effects = identity
    .update_did_document(IotaDocument::new(test_client.network()), &bobs_delegation_token)
    .finish(&test_client)
    .await?
    .execute(&alice_signer, &test_client)
    .await?
    .effects;
  assert!(effects.as_v1().status.is_success());

  // Bob can revoke its delegation token anytime.
  identity
    .revoke_delegation_token(
      bob_token.as_controller().expect("bob is a controller"),
      bobs_delegation_token.as_delegate().expect("is a delegation token"),
    )?
    .execute(&bob_signer, &test_client)
    .await?;

  // Once revoked whoever is holding it won't be able to act in bob's stead.
  let res = identity
    .update_did_document(IotaDocument::new(test_client.network()), &bobs_delegation_token)
    .finish(&test_client)
    .await?
    .execute(&alice_signer, &test_client)
    .await;
  assert!(res.is_err());

  // A revoked token can be unrevoked too.
  identity
    .unrevoke_delegation_token(
      bob_token.as_controller().expect("bob is a controller"),
      bobs_delegation_token.as_delegate().expect("is a delegation token"),
    )?
    .execute(&bob_signer, &test_client)
    .await?;

  // Making the token valid again.
  let res = identity
    .update_did_document(IotaDocument::new(test_client.network()), &bobs_delegation_token)
    .finish(&test_client)
    .await?
    .execute(&alice_signer, &test_client)
    .await?
    .output;
  assert!(matches!(res, ProposalResult::Pending(_)));

  // The owner of the token can delete it whenever.
  let bobs_delegation_token_id = bobs_delegation_token.id();
  identity
    .delete_delegation_token(bobs_delegation_token.try_delegate().unwrap())?
    .execute(&alice_signer, &test_client)
    .await?;

  let maybe_obj = test_client.object(bobs_delegation_token_id, None).await?;
  assert!(maybe_obj.is_none());

  Ok(())
}

#[tokio::test]
async fn access_sub_identity_works() -> anyhow::Result<()> {
  let client = get_funded_test_client().await?;
  let signer = client.new_signer().await?;
  let network = client.network();
  let mut identity = client
    .create_identity(IotaDocument::new(network))
    .finish()
    .execute(&signer, &client)
    .await?
    .output;
  let mut sub_identity = client
    .create_identity(IotaDocument::new(network))
    .controller(identity.id().into(), 1)
    .finish()
    .execute(&signer, &client)
    .await?
    .output;
  let controller_token = identity
    .get_controller_token(signer.address(), &client)
    .await?
    .expect("is a controller");
  let client_ref = &client;

  identity
    .access_sub_identity(&mut sub_identity, &controller_token)
    .to_perform(|sub_identity, token| async move { sub_identity.deactivate_did(&token).finish(client_ref).await })
    .finish(&client)
    .await?
    .execute(&signer, &client)
    .await?;

  assert!(sub_identity.did_document().metadata.deactivated == Some(true));
  // Local `sub_identity`` reflects its on-chain Identity object.
  let synced_sub_identity = get_identity(&client, sub_identity.id()).await?.unwrap();
  assert_eq!(sub_identity.did_document(), synced_sub_identity.did_document());

  Ok(())
}
