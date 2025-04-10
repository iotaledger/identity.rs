// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use crate::common::get_funded_test_client;
use crate::common::TEST_GAS_BUDGET;
use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_credential::credential::CredentialBuilder;
use identity_credential::validator::FailFast;
use identity_credential::validator::JwtCredentialValidationOptions;
use identity_credential::validator::JwtCredentialValidator;
use identity_document::document::CoreDocument;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota_core::rebased::AuthenticatedAsset;
use identity_iota_core::rebased::PublicAvailableVC;
use identity_iota_core::rebased::TransferProposal;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_iota_interaction::IotaClientTrait;
use identity_iota_interaction::MoveType as _;
use identity_storage::JwkDocumentExt;
use identity_storage::JwsSignatureOptions;
use identity_verification::VerificationMethod;
use iota_sdk::types::TypeTag;
use itertools::Itertools as _;
use move_core_types::language_storage::StructTag;

#[tokio::test]
async fn creating_authenticated_asset_works() -> anyhow::Result<()> {

  // Glossary:
  // * `ProductClient`:
  //   Manages product addresses, public_keys, id_documents and other
  //   product specific data, needed to create `ResourceTxBuilder`s for
  //   specific `ProductResource`s.
  //   Examples: `IdentityClient`, `NotarizationClient`
  // * `ProductResource`:
  //   References one or more existing object(s) on the IOTA ledger.
  //   Examples: ' AuthenticatedAsset<T>', `DynamicNotarization`, `LockedNotarization`
  // * `ResourceTxBuilder`:
  //   A transaction builder (current name `TransactionBuilder<Tx>`),
  //   providing a `Transaction` to perform actions (create, update, destroy, ...)
  //   on a `ProductResource`.
  // * `Transaction`:
  //   The currently used `Transaction` trait, but in a `ProductClient` agnostic way.
  //   Provides the transaction needed to perform an action on a `ProductResource` object
  //   on the ledger (fn `build_programmable_transaction()`) and to evaluate the
  //   transaction results (fn `apply()`).
  // * `ProgrammableTransactionBlockManager`:
  //   Manages the overall PTB construction and execution process.
  //   An extended `ProgrammableTransactionBuilder` (e.g. from IOTA Rust or TS SDK) providing
  //   additional functionality to facilitate the usage of `ProductResource`s.
  // * `TransactionHandle`:
  //   A reference to a `Transaction` for a specific action on a specific `ProductResource`.
  //   Can be used to receive the resulting `ProductResource` of an executed `Transaction`.
  //   For example, it can be seen as a promise to receive an `AuthenticatedAsset<u64>`
  //   (a.k.a. `ProductResource`) after a PTB, containing a `Transaction` of type
  //   `CreateAsset<u64>`, has been executed by a `ProgrammableTransactionBlockManager`.

  let test_client = get_funded_test_client().await?;

  // ******** Introduction ********
  // Alice is going to build and execute a PTB to create an AuthenticatedAsset<u64>,
  // using an IdentityClient that uses the same address & public_key as the
  // `ProgrammableTransactionBlockManager` that is used to manage the PTB.
  // Alternatively someone else (GasStation, Bob, ...) could manage the PTB construction
  // and execution process for her.

  // Create a `ProgrammableTransactionBlockManager` for Alice
  let alice_tx_manager = test_client.new_user_tx_manager().await?;

  // Create the `ProductClient` which is an `IdentityClient` here.
  // The IdentityClient uses the same address & public_key as alice_tx_manager.
  //
  //    @param tx_manager: If provided, provides address & public_key data for the alice identity,
  //                       if None, new address & public_key data are used.
  //    async fn new_user_client(&self, tx_manager: Option<&ProgrammableTransactionBlockManager>)
  //       -> anyhow::Result<IdentityClient<MemSigner>>
  let alice_client = test_client.new_user_client(Some(&alice_tx_manager)).await?;

  // ****************************************************************************************
  // *  Here we could add arbitrary transactions to the managed PTB using alice_tx_manager  *
  // ****************************************************************************************

  // Create a `ResourceTxBuilder` for an `AuthenticatedAsset<u64>`
  let alice_authenticated_asset_tx_builder = alice_client
    .create_authenticated_asset::<u64>(42)
    .finish();

  // Add the alice_authenticated_asset_tx_builder to the `ProgrammableTransactionBlockManager`.
  //
  // The returned `tx_handle_for_auth_asset` is a `TransactionHandle`, that can be used
  // later, to receive the `AuthenticatedAsset<u64>` after the PTB has been executed.
  //
  // The `ProgrammableTransactionBlockManager` internally manages an ordered list of
  // `TransactionBuilder<Tx>` instances that will be `build()` when
  // `ProgrammableTransactionBlockManager::build_and_execute()` is called (see below). The
  // `TransactionHandle` is just a reference or list index to the `TransactionBuilder<Tx>` instance
  // that finally provides the `Transaction::output`. 
  //
  // The function `append_tx()` needs to be product/resource agnostic and shall be usable for all
  // `ProductResource`s of all IOTA Products.
  //
  //    // The generic arguments C and T used here are just placeholders to symbolize
  //    // the currently unknown exact types.
  //    async fn append_tx(&self, resource_tx_builder: TransactionBuilder<C>)
  //      -> anyhow::Result<TransactionHandle<T>>
  let tx_handle_for_auth_asset = alice_tx_manager
    .append_tx(alice_authenticated_asset_tx_builder)
    .await?;

  // ****************************************************************************************
  // *  Here we could add arbitrary transactions to the managed PTB using alice_tx_manager
  // *
  // *  For example:
  // *  let another_tx_handle = alice_tx_manager
  // *    .append_tx(just_another_product_resource_tx_builder)
  // *    .await?;
  // ****************************************************************************************

  // Create and execute the final PTB.
  // The final PTB is constructed from all previously added `TransactionBuilder`s.
  // Then the final PTB is executed and the resulting `IotaTransactionBlockEffects` are returned for
  // quality assurance and logging purposes. The _tx_block_effects are not needed
  // to receive the created/updated `ProductResource`, which will be done using the
  // `TransactionHandle` tx_handle_for_auth_asset.
  //
  //    async fn build_and_execute(&self) -> anyhow::Result<IotaTransactionBlockEffects>
  //
  // Alternatively the final PTB is available:
  //
  //    async fn build_programmable_transaction(&self) -> anyhow::Result<ProgrammableTransaction>
  //
  //    let ptb = alice_tx_manager.build_programmable_transaction().await?;
  //    let tx_effects = ... // Execute the ptb for example using the GasStation
  //    alice_tx_manager.apply(tx_effects).await?;
  
  let _tx_block_effects = alice_tx_manager.build_and_execute().await?;

  // Receive the `ProductResource`, which is an `AuthenticatedAsset<u64>`, from
  // the tx_handle_for_auth_asset.
  //
  //     async fn get_resource(&self) -> anyhow::Result<TransactionOutput<T>>
  let asset = tx_handle_for_auth_asset.get_resource()
    .await?
    .output;

  assert_eq!(asset.content(), &42);

  Ok(())
}

/*
#[tokio::test]
async fn transferring_asset_works() -> anyhow::Result<()> {
  let test_client = get_funded_test_client().await?;
  let alice_client = test_client.new_user_client().await?;
  let bob_client = test_client.new_user_client().await?;

  // Alice creates a new asset.
  let asset = alice_client
    .create_authenticated_asset::<u64>(42)
    .transferable(true)
    .finish()
    .build_and_execute(&alice_client)
    .await?
    .output;
  let asset_id = asset.id();

  // Alice propose to Bob the transfer of the asset.
  let proposal = asset
    .transfer(bob_client.sender_address())?
    .build_and_execute(&alice_client)
    .await?
    .output;
  let proposal_id = proposal.id();
  // Bob accepts the transfer.
  proposal.accept().build_and_execute(&bob_client).await?;
  let TypeTag::Struct(asset_type) = AuthenticatedAsset::<u64>::move_type(test_client.package_id()) else {
    unreachable!("asset is a struct");
  };
  let bob_owns_asset = bob_client
    .find_owned_ref(*asset_type, |obj| obj.object_id == asset_id)
    .await?
    .is_some();
  assert!(bob_owns_asset);

  // Alice concludes the transfer.
  let proposal = TransferProposal::get_by_id(proposal_id, &alice_client).await?;
  assert!(proposal.is_concluded());
  proposal.conclude_or_cancel().build_and_execute(&alice_client).await?;

  // After the transfer is concluded all capabilities as well as the proposal bound to the transfer are deleted.
  let alice_has_sender_cap = alice_client
    .find_owned_ref(
      StructTag::from_str(&format!("{}::asset::SenderCap", test_client.package_id()))?,
      |_| true,
    )
    .await?
    .is_some();
  assert!(!alice_has_sender_cap);
  let bob_has_recipient_cap = bob_client
    .find_owned_ref(
      StructTag::from_str(&format!("{}::asset::RecipientCap", test_client.package_id()))?,
      |_| true,
    )
    .await?
    .is_some();
  assert!(!bob_has_recipient_cap);

  Ok(())
}

#[tokio::test]
async fn accepting_the_transfer_of_an_asset_requires_capability() -> anyhow::Result<()> {
  let test_client = get_funded_test_client().await?;
  let alice_client = test_client.new_user_client().await?;
  let bob_client = test_client.new_user_client().await?;
  let caty_client = test_client.new_user_client().await?;

  // Alice creates a new asset.
  let asset = alice_client
    .create_authenticated_asset::<u64>(42)
    .transferable(true)
    .finish()
    .build_and_execute(&alice_client)
    .await?
    .output;

  // Alice propose to Bob the transfer of the asset.
  let proposal = asset
    .transfer(bob_client.sender_address())?
    .build_and_execute(&alice_client)
    .await?
    .output;

  // Caty attempts to accept the transfer instead of Bob but gets an error
  let _error = proposal.accept().build_and_execute(&caty_client).await.unwrap_err();

  Ok(())
}

#[tokio::test]
async fn modifying_mutable_asset_works() -> anyhow::Result<()> {
  let test_client = get_funded_test_client().await?;
  let alice_client = test_client.new_user_client().await?;

  let mut asset = alice_client
    .create_authenticated_asset::<u64>(42)
    .mutable(true)
    .finish()
    .build_and_execute(&alice_client)
    .await?
    .output;

  asset.set_content(420)?.build_and_execute(&alice_client).await?;
  assert_eq!(asset.content(), &420);

  Ok(())
}

#[tokio::test]
async fn deleting_asset_works() -> anyhow::Result<()> {
  let test_client = get_funded_test_client().await?;
  let alice_client = test_client.new_user_client().await?;

  let asset = alice_client
    .create_authenticated_asset::<u64>(42)
    .deletable(true)
    .finish()
    .build_and_execute(&alice_client)
    .await?
    .output;
  let asset_id = asset.id();

  asset.delete()?.build_and_execute(&alice_client).await?;
  let alice_owns_asset = alice_client
    .read_api()
    .get_owned_objects(alice_client.sender_address(), None, None, None)
    .await?
    .data
    .into_iter()
    .map(|obj| obj.object_id().unwrap())
    .contains(&asset_id);
  assert!(!alice_owns_asset);

  Ok(())
}

#[tokio::test]
async fn hosting_vc_works() -> anyhow::Result<()> {
  let test_client = get_funded_test_client().await?;
  let identity_client = test_client.new_user_client().await?;

  let newly_created_identity = identity_client
    .create_identity(IotaDocument::new(identity_client.network()))
    .finish()
    .with_gas_budget(TEST_GAS_BUDGET)
    .build_and_execute(&identity_client)
    .await?
    .output;
  let object_id = newly_created_identity.id();
  let did = { IotaDID::parse(format!("did:iota:{object_id}"))? };

  test_client
    .store_key_id_for_verification_method(identity_client.clone(), did.clone())
    .await?;
  let did_doc = CoreDocument::builder(Object::default())
    .id(did.clone().into())
    .verification_method(VerificationMethod::new_from_jwk(
      did.clone(),
      identity_client.signer().public_key().clone(),
      Some(identity_client.signer().key_id().as_str()),
    )?)
    .build()?;
  let credential = CredentialBuilder::new(Object::default())
    .id(Url::parse("http://example.com/credentials/42")?)
    .issuance_date(Timestamp::now_utc())
    .issuer(Url::parse(did.to_string())?)
    .subject(serde_json::from_value(serde_json::json!({
      "id": did,
      "type": ["VerifiableCredential", "ExampleCredential"],
      "value": 3
    }))?)
    .build()?;
  let credential_jwt = did_doc
    .create_credential_jwt(
      &credential,
      identity_client.signer().storage(),
      identity_client.signer().key_id().as_str(),
      &JwsSignatureOptions::default(),
      None,
    )
    .await?;

  let vc = PublicAvailableVC::new(credential_jwt.clone(), Some(TEST_GAS_BUDGET), &identity_client).await?;
  assert_eq!(credential_jwt, vc.jwt());

  let validator = JwtCredentialValidator::with_signature_verifier(EdDSAJwsVerifier::default());
  validator.validate::<_, Object>(
    &credential_jwt,
    &did_doc,
    &JwtCredentialValidationOptions::default(),
    FailFast::FirstError,
  )?;

  Ok(())
}
*/