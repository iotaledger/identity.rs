use std::str::FromStr;

use crate::common::get_client as get_test_client;
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
use identity_iota_core::IotaDID;
use identity_storage::JwkDocumentExt;
use identity_storage::JwsSignatureOptions;
use identity_sui_name_tbd::utils::MoveType as _;
use identity_sui_name_tbd::AuthenticatedAsset;
use identity_sui_name_tbd::PublicAvailableVC;
use identity_verification::VerificationMethod;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::TypeTag;
use itertools::Itertools as _;
use move_core_types::language_storage::StructTag;

#[tokio::test]
async fn creating_authenticated_asset_works() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let alice_client = test_client.new_user_client().await?;

  let asset = alice_client
    .create_authenticated_asset::<u64>(42)
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&alice_client)
    .await?;
  assert_eq!(asset.content(), &42);

  Ok(())
}

#[tokio::test]
async fn transfering_asset_works() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let alice_client = test_client.new_user_client().await?;
  let bob_client = test_client.new_user_client().await?;

  // Alice creates a new asset.
  let asset = alice_client
    .create_authenticated_asset::<u64>(42)
    .transferable(true)
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&alice_client)
    .await?;
  let asset_id = asset.id();

  // Alice propose to Bob the transfer of the asset.
  let mut proposal = asset
    .transfer(bob_client.sender_address(), TEST_GAS_BUDGET, &alice_client)
    .await?;
  // Bob accepts the transfer.
  proposal.accept(TEST_GAS_BUDGET, &bob_client).await?;
  let TypeTag::Struct(asset_type) = AuthenticatedAsset::<u64>::move_type(test_client.package_id()) else {
    unreachable!("asset is a struct");
  };
  let bob_owns_asset = bob_client
    .find_owned_ref(*asset_type, |obj| obj.object_id == asset_id)
    .await?
    .is_some();
  assert!(bob_owns_asset);

  // Alice concludes the transfer.
  assert!(proposal.is_concluded());
  proposal.conclude_or_cancel(TEST_GAS_BUDGET, &alice_client).await?;

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
  let test_client = get_test_client().await?;
  let alice_client = test_client.new_user_client().await?;
  let bob_client = test_client.new_user_client().await?;
  let caty_client = test_client.new_user_client().await?;

  // Alice creates a new asset.
  let asset = alice_client
    .create_authenticated_asset::<u64>(42)
    .transferable(true)
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&alice_client)
    .await?;

  // Alice propose to Bob the transfer of the asset.
  let mut proposal = asset
    .transfer(bob_client.sender_address(), TEST_GAS_BUDGET, &alice_client)
    .await?;

  // Caty attempts to accept the transfer instead of Bob but gets an error
  let error = proposal.accept(TEST_GAS_BUDGET, &caty_client).await.unwrap_err();
  assert!(matches!(error, identity_sui_name_tbd::Error::MissingPermission(_)));

  Ok(())
}

#[tokio::test]
async fn modifying_mutable_asset_works() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let alice_client = test_client.new_user_client().await?;

  let mut asset = alice_client
    .create_authenticated_asset::<u64>(42)
    .mutable(true)
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&alice_client)
    .await?;

  asset.set_content(420, TEST_GAS_BUDGET, &alice_client).await?;
  assert_eq!(asset.content(), &420);

  Ok(())
}

#[tokio::test]
async fn deleting_asset_works() -> anyhow::Result<()> {
  let test_client = get_test_client().await?;
  let alice_client = test_client.new_user_client().await?;

  let asset = alice_client
    .create_authenticated_asset::<u64>(42)
    .deletable(true)
    .gas_budget(TEST_GAS_BUDGET)
    .finish(&alice_client)
    .await?;
  let asset_id = asset.id();

  asset.delete(TEST_GAS_BUDGET, &alice_client).await?;
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
  let test_client = get_test_client().await?;
  let identity_client = test_client.new_user_client().await?;

  let did = {
    let object_id = ObjectID::random();
    IotaDID::parse(&format!("did:iota:{object_id}"))?
  };
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

  let vc = PublicAvailableVC::new(credential_jwt.clone(), TEST_GAS_BUDGET, &identity_client).await?;
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
