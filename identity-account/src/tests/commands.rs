// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use crate::account::Account;
use crate::account::AccountConfig;
use crate::account::AccountSetup;
use crate::error::Error;
use crate::error::Result;
use crate::events::Update;
use crate::events::UpdateError;
use crate::identity::IdentitySetup;

use crate::identity::IdentityState;
use crate::storage::MemStore;
use crate::types::Generation;
use crate::types::KeyLocation;
use crate::types::MethodSecret;
use identity_core::common::Timestamp;
use identity_core::crypto::KeyCollection;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_did::did::RelativeDIDUrl;
use identity_did::verification::MethodRelationship;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_iota::did::IotaDID;
use identity_iota::tangle::Network;

fn account_setup() -> AccountSetup {
  AccountSetup::new_with_options(
    Arc::new(MemStore::new()),
    Some(AccountConfig::new().testmode(true)),
    None,
  )
}

#[tokio::test]
async fn test_create_identity() -> Result<()> {
  let account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

  let expected_fragment = format!("{}{}", crate::events::DEFAULT_UPDATE_METHOD_PREFIX, Generation::new());

  let state: &IdentityState = account.state();

  assert!(state.as_document().resolve_method(&expected_fragment).is_some());
  assert_eq!(
    state.as_document().as_document().verification_relationships().count(),
    1
  );
  assert_eq!(state.as_document().as_document().methods().count(), 1);

  let location = state
    .method_location(MethodType::Ed25519VerificationKey2018, expected_fragment.clone())
    .unwrap();

  // Ensure we can retrieve the correct location for the key.
  assert_eq!(
    location,
    KeyLocation::new(
      MethodType::Ed25519VerificationKey2018,
      expected_fragment,
      Generation::new(),
      Generation::new()
    )
  );

  // Ensure the key exists in storage.
  assert!(account.storage().key_exists(account.did(), &location).await.unwrap());

  // Enure the state was written to storage.
  assert!(account.load_state().await.is_ok());

  // Ensure timestamps were recently set.
  assert!(state.as_document().created() > Timestamp::from_unix(Timestamp::now_utc().to_unix() - 15));
  assert!(state.as_document().updated() > Timestamp::from_unix(Timestamp::now_utc().to_unix() - 15));

  Ok(())
}

#[tokio::test]
async fn test_create_identity_network() -> Result<()> {
  // Create an identity with a valid network string
  let create_identity: IdentitySetup = IdentitySetup::new().network("dev")?.key_type(KeyType::Ed25519);
  let account = Account::create_identity(account_setup(), create_identity).await?;

  assert_eq!(
    account.did().network().unwrap().name(),
    Network::try_from_name("dev").unwrap().name()
  );

  Ok(())
}

#[tokio::test]
async fn test_create_identity_invalid_network() -> Result<()> {
  // Attempt to create an identity with an invalid network string
  let result: Result<IdentitySetup> = IdentitySetup::new().network("Invalid=Network!");

  // Ensure an `InvalidNetworkName` error is thrown
  assert!(matches!(
    result.unwrap_err(),
    Error::IotaError(identity_iota::Error::InvalidNetworkName),
  ));

  Ok(())
}

#[tokio::test]
async fn test_create_identity_already_exists() -> Result<()> {
  let keypair = KeyPair::new_ed25519()?;
  let identity_create = IdentitySetup::default()
    .key_type(KeyType::Ed25519)
    .method_secret(MethodSecret::Ed25519(keypair.private().clone()));
  let account_setup = account_setup();

  let account = Account::create_identity(account_setup.clone(), identity_create.clone()).await?;
  let did: IotaDID = account.did().to_owned();

  let initial_state = account_setup.storage.state(&did).await?.unwrap();

  let output = Account::create_identity(account_setup.clone(), identity_create).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::UpdateError(UpdateError::DocumentAlreadyExists),
  ));

  // Ensure nothing was overwritten in storage
  assert_eq!(initial_state, account_setup.storage.state(&did).await?.unwrap());

  Ok(())
}

#[tokio::test]
async fn test_create_identity_from_invalid_private_key() -> Result<()> {
  let private_bytes: Box<[u8]> = Box::new([0; 33]);
  let private_key: PrivateKey = PrivateKey::from(private_bytes);

  let id_create = IdentitySetup::new()
    .key_type(KeyType::Ed25519)
    .method_secret(MethodSecret::Ed25519(private_key));

  let err = Account::create_identity(account_setup(), id_create).await.unwrap_err();

  assert!(matches!(err, Error::UpdateError(UpdateError::InvalidMethodSecret(_))));

  Ok(())
}

#[tokio::test]
async fn test_create_method() -> Result<()> {
  let mut account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

  let initial_state: IdentityState = account.state().to_owned();
  let method_type = MethodType::Ed25519VerificationKey2018;

  let fragment = "key-1".to_owned();
  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: method_type,
    fragment: fragment.clone(),
  };

  account.process_update(update, false).await?;

  let state: &IdentityState = account.state();

  // Ensure existence and key type
  assert_eq!(
    state.as_document().resolve_method(&fragment).unwrap().key_type(),
    method_type
  );

  // Still only the default relationship.
  assert_eq!(
    state.as_document().as_document().verification_relationships().count(),
    1
  );
  assert_eq!(state.as_document().as_document().methods().count(), 2);

  let location = state.method_location(method_type, fragment.clone()).unwrap();

  // Ensure we can retrieve the correct location for the key.
  assert_eq!(
    location,
    KeyLocation::new(
      method_type,
      fragment,
      state.integration_generation(),
      state.diff_generation()
    )
  );

  // Ensure the key exists in storage.
  assert!(account.storage().key_exists(account.did(), &location).await.unwrap());

  // Ensure `created` wasn't updated.
  assert_eq!(initial_state.as_document().created(), state.as_document().created());
  // Ensure `updated` was recently set.
  assert!(state.as_document().updated() > Timestamp::from_unix(Timestamp::now_utc().to_unix() - 15));

  Ok(())
}

#[tokio::test]
async fn test_create_scoped_method() -> Result<()> {
  for scope in &[
    MethodScope::Authentication,
    MethodScope::AssertionMethod,
    MethodScope::KeyAgreement,
    MethodScope::CapabilityInvocation,
    MethodScope::CapabilityDelegation,
  ] {
    let mut account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

    let fragment = "key-1".to_owned();

    let update: Update = Update::CreateMethod {
      scope: *scope,
      method_secret: None,
      type_: MethodType::Ed25519VerificationKey2018,
      fragment: fragment.clone(),
    };

    account.process_update(update, false).await?;

    let state: &IdentityState = account.state();

    assert_eq!(
      state.as_document().as_document().verification_relationships().count(),
      2
    );

    assert_eq!(state.as_document().as_document().methods().count(), 2);

    let mut relative_url = RelativeDIDUrl::new();
    relative_url.set_fragment(Some(&fragment)).unwrap();

    let core_doc = state.as_document().as_document();

    let contains = match scope {
      MethodScope::Authentication => core_doc
        .authentication()
        .iter()
        .any(|method_ref| method_ref.id().url() == &relative_url),
      MethodScope::AssertionMethod => core_doc
        .assertion_method()
        .iter()
        .any(|method_ref| method_ref.id().url() == &relative_url),
      MethodScope::KeyAgreement => core_doc
        .key_agreement()
        .iter()
        .any(|method_ref| method_ref.id().url() == &relative_url),
      MethodScope::CapabilityDelegation => core_doc
        .capability_delegation()
        .iter()
        .any(|method_ref| method_ref.id().url() == &relative_url),
      MethodScope::CapabilityInvocation => core_doc
        .capability_invocation()
        .iter()
        .any(|method_ref| method_ref.id().url() == &relative_url),
      _ => unreachable!(),
    };

    assert!(contains);
  }

  Ok(())
}

#[tokio::test]
async fn test_create_method_duplicate_fragment() -> Result<()> {
  let mut account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "key-1".to_owned(),
  };

  account.process_update(update.clone(), false).await?;

  let output = account.process_update(update.clone(), false).await;

  // Attempting to add a method with the same fragment in the same int and diff generation.
  assert!(matches!(
    output.unwrap_err(),
    Error::UpdateError(UpdateError::DuplicateKeyLocation(_)),
  ));

  // Fake publishing by incrementing any generation.
  account.state_mut_unchecked().increment_diff_generation().unwrap();

  let output = account.process_update(update, false).await;

  // Now the location is different, but the fragment is the same.
  assert!(matches!(
    output.unwrap_err(),
    Error::UpdateError(UpdateError::DuplicateKeyFragment(_)),
  ));

  Ok(())
}

#[tokio::test]
async fn test_create_method_from_private_key() -> Result<()> {
  let mut account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

  let keypair = KeyPair::new_ed25519()?;
  let fragment = "key-1".to_owned();
  let method_type = MethodType::Ed25519VerificationKey2018;

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: Some(MethodSecret::Ed25519(keypair.private().clone())),
    type_: method_type,
    fragment: fragment.clone(),
  };

  account.process_update(update, false).await?;

  let state: &IdentityState = account.state();

  assert!(state.as_document().resolve_method(&fragment).is_some());

  let location = state.method_location(method_type, fragment).unwrap();
  let public_key = account.storage().key_get(account.did(), &location).await?;

  assert_eq!(public_key.as_ref(), keypair.public().as_ref());

  Ok(())
}

#[tokio::test]
async fn test_create_method_from_invalid_private_key() -> Result<()> {
  let mut account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

  let private_bytes: Box<[u8]> = Box::new([0; 33]);
  let private_key = PrivateKey::from(private_bytes);

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: Some(MethodSecret::Ed25519(private_key)),
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "key-1".to_owned(),
  };

  let err = account.process_update(update, false).await.unwrap_err();

  assert!(matches!(err, Error::UpdateError(UpdateError::InvalidMethodSecret(_))));

  Ok(())
}

#[tokio::test]
async fn test_attach_method_relationship() -> Result<()> {
  let mut account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

  let fragment = "key-1".to_owned();

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: fragment.clone(),
  };

  account.process_update(update, false).await?;

  // One relationship by default.
  assert_eq!(
    account
      .state()
      .as_document()
      .as_document()
      .verification_relationships()
      .count(),
    1
  );

  let default_method_fragment = account
    .document()
    .default_signing_method()
    .unwrap()
    .id()
    .fragment()
    .unwrap()
    .to_owned();

  // Attempt attaching a relationship to an embedded method.
  let update: Update = Update::AttachMethod {
    relationships: vec![MethodRelationship::AssertionMethod, MethodRelationship::KeyAgreement],
    fragment: default_method_fragment,
  };

  let err = account.process_update(update, false).await.unwrap_err();

  assert!(matches!(err, Error::UpdateError(UpdateError::InvalidMethodTarget)));

  // No relationships were created.
  assert_eq!(account.document().as_document().verification_relationships().count(), 1);

  assert_eq!(account.document().as_document().assertion_method().iter().count(), 0);
  assert_eq!(account.document().as_document().key_agreement().iter().count(), 0);

  let update: Update = Update::AttachMethod {
    relationships: vec![MethodRelationship::AssertionMethod, MethodRelationship::KeyAgreement],
    fragment: fragment.clone(),
  };

  account.process_update(update, false).await?;

  // Relationships were created.
  assert_eq!(account.document().as_document().verification_relationships().count(), 3);

  assert_eq!(account.document().as_document().assertion_method().len(), 1);
  assert_eq!(
    account
      .document()
      .as_document()
      .assertion_method()
      .first()
      .unwrap()
      .id()
      .fragment()
      .unwrap(),
    fragment
  );
  assert_eq!(account.document().as_document().key_agreement().len(), 1);
  assert_eq!(
    account
      .document()
      .as_document()
      .key_agreement()
      .first()
      .unwrap()
      .id()
      .fragment()
      .unwrap(),
    fragment
  );

  Ok(())
}

#[tokio::test]
async fn test_detach_method_relationship() -> Result<()> {
  let mut account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

  let generic_fragment = "key-1".to_owned();
  let embedded_fragment = "embedded-1".to_owned();

  // Add an embedded method.
  let update: Update = Update::CreateMethod {
    scope: MethodScope::Authentication,
    method_secret: None,
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: embedded_fragment.clone(),
  };

  account.process_update(update, false).await?;

  // Attempt detaching a relationship from an embedded method.
  let update: Update = Update::DetachMethod {
    relationships: vec![MethodRelationship::Authentication],
    fragment: embedded_fragment,
  };

  let err = account.process_update(update, false).await.unwrap_err();

  assert!(matches!(err, Error::UpdateError(UpdateError::InvalidMethodTarget)));

  // No relationships were removed.
  assert_eq!(account.document().as_document().verification_relationships().count(), 2);

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: generic_fragment.clone(),
  };

  account.process_update(update, false).await?;

  let update: Update = Update::AttachMethod {
    relationships: vec![MethodRelationship::AssertionMethod, MethodRelationship::KeyAgreement],
    fragment: generic_fragment.clone(),
  };

  account.process_update(update, false).await?;

  assert_eq!(account.document().as_document().assertion_method().len(), 1);
  assert_eq!(account.document().as_document().key_agreement().len(), 1);

  let update: Update = Update::DetachMethod {
    relationships: vec![MethodRelationship::AssertionMethod, MethodRelationship::KeyAgreement],
    fragment: generic_fragment.clone(),
  };

  account.process_update(update, false).await?;

  assert_eq!(account.document().as_document().assertion_method().len(), 0);
  assert_eq!(account.document().as_document().key_agreement().len(), 0);

  Ok(())
}

#[tokio::test]
async fn test_create_method_with_type_secret_mismatch() -> Result<()> {
  let mut account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

  let private_bytes: Box<[u8]> = Box::new([0; 32]);
  let private_key = PrivateKey::from(private_bytes);

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: Some(MethodSecret::Ed25519(private_key)),
    type_: MethodType::MerkleKeyCollection2021,
    fragment: "key-1".to_owned(),
  };

  let err = account.process_update(update, false).await.unwrap_err();

  assert!(matches!(err, Error::UpdateError(UpdateError::InvalidMethodSecret(_))));

  let key_collection = KeyCollection::new_ed25519(4).unwrap();

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: Some(MethodSecret::MerkleKeyCollection(key_collection)),
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "key-2".to_owned(),
  };

  let err = account.process_update(update, false).await.unwrap_err();

  assert!(matches!(err, Error::UpdateError(UpdateError::InvalidMethodSecret(_))));

  Ok(())
}

#[tokio::test]
async fn test_delete_method() -> Result<()> {
  let mut account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

  let fragment = "key-1".to_owned();
  let method_type = MethodType::Ed25519VerificationKey2018;
  let initial_state = account.state().to_owned();

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: method_type,
    fragment: fragment.clone(),
  };

  account.process_update(update, false).await?;

  // Ensure it was added.
  assert!(account.state().as_document().resolve_method(&fragment).is_some());

  let update: Update = Update::DeleteMethod {
    fragment: "key-1".to_owned(),
  };

  account.process_update(update, false).await?;

  let state: &IdentityState = account.state();

  // Ensure it no longer exists.
  assert!(state.as_document().resolve_method(&fragment).is_none());

  // Still only the default relationship.
  assert_eq!(
    state.as_document().as_document().verification_relationships().count(),
    1
  );

  assert_eq!(state.as_document().as_document().methods().count(), 1);

  let location = state.method_location(method_type, fragment.clone()).unwrap();

  // Ensure the key still exists in storage - deletion in storage happens after successful publication.
  assert!(account.storage().key_exists(account.did(), &location).await.unwrap());

  // Ensure `created` wasn't updated.
  assert_eq!(initial_state.as_document().created(), state.as_document().created());
  // Ensure `updated` was recently set.
  assert!(state.as_document().updated() > Timestamp::from_unix(Timestamp::now_utc().to_unix() - 15));

  Ok(())
}
