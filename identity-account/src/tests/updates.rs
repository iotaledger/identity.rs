// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use crate::account::Account;
use crate::account::AccountConfig;
use crate::account::AccountSetup;
use crate::error::Error;
use crate::error::Result;
use crate::identity::IdentitySetup;
use crate::updates::Update;
use crate::updates::UpdateError;

use crate::identity::IdentityState;
use crate::storage::MemStore;
use crate::types::Generation;
use crate::types::KeyLocation;
use crate::types::MethodSecret;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::crypto::KeyCollection;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_did::did::DID;
use identity_did::service::ServiceEndpoint;
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

  let expected_fragment = format!("{}{}", crate::updates::DEFAULT_UPDATE_METHOD_PREFIX, Generation::new());

  let state: &IdentityState = account.state();

  assert!(state.document().resolve_method(&expected_fragment).is_some());
  assert_eq!(state.document().as_document().verification_relationships().count(), 1);
  assert_eq!(state.document().as_document().methods().count(), 1);

  let location = state
    .method_location(MethodType::Ed25519VerificationKey2018, expected_fragment.clone())
    .unwrap();

  // Ensure we can retrieve the correct location for the key.
  assert_eq!(
    location,
    KeyLocation::new(
      MethodType::Ed25519VerificationKey2018,
      expected_fragment,
      Generation::new()
    )
  );

  // Ensure the key exists in storage.
  assert!(account.storage().key_exists(account.did(), &location).await.unwrap());

  // Enure the state was written to storage.
  assert!(account.load_state().await.is_ok());

  // Ensure timestamps were recently set.
  assert!(state.document().created() > Timestamp::from_unix(Timestamp::now_utc().to_unix() - 15));
  assert!(state.document().updated() > Timestamp::from_unix(Timestamp::now_utc().to_unix() - 15));

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

  account.process_update(update).await?;

  let state: &IdentityState = account.state();

  // Ensure existence and key type
  assert_eq!(
    state.document().resolve_method(&fragment).unwrap().key_type(),
    method_type
  );

  // Still only the default relationship.
  assert_eq!(state.document().as_document().verification_relationships().count(), 1);
  assert_eq!(state.document().as_document().methods().count(), 2);

  let location = state.method_location(method_type, fragment.clone()).unwrap();

  // Ensure we can retrieve the correct location for the key.
  assert_eq!(
    location,
    KeyLocation::new(
      method_type,
      fragment,
      // `create_identity` calls publish, which increments the generation.
      Generation::new().try_increment().unwrap(),
    )
  );

  // Ensure the key exists in storage.
  assert!(account.storage().key_exists(account.did(), &location).await.unwrap());

  // Ensure `created` wasn't updated.
  assert_eq!(initial_state.document().created(), state.document().created());
  // Ensure `updated` was recently set.
  assert!(state.document().updated() > Timestamp::from_unix(Timestamp::now_utc().to_unix() - 15));

  Ok(())
}

#[tokio::test]
async fn test_create_scoped_method() -> Result<()> {
  for scope in &[
    MethodScope::assertion_method(),
    MethodScope::authentication(),
    MethodScope::capability_delegation(),
    MethodScope::capability_invocation(),
    MethodScope::key_agreement(),
  ] {
    let mut account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

    let fragment = "#key-1".to_owned();

    let update: Update = Update::CreateMethod {
      scope: *scope,
      method_secret: None,
      type_: MethodType::Ed25519VerificationKey2018,
      fragment: fragment.clone(),
    };

    account.process_update(update).await?;

    let state: &IdentityState = account.state();

    assert_eq!(state.document().as_document().verification_relationships().count(), 2);

    assert_eq!(state.document().as_document().methods().count(), 2);

    let core_doc = state.document().as_document();

    let contains = match scope {
      MethodScope::VerificationRelationship(MethodRelationship::Authentication) => core_doc
        .try_resolve_method_with_scope(&fragment, MethodScope::authentication())
        .is_ok(),
      MethodScope::VerificationRelationship(MethodRelationship::AssertionMethod) => core_doc
        .try_resolve_method_with_scope(&fragment, MethodScope::assertion_method())
        .is_ok(),
      MethodScope::VerificationRelationship(MethodRelationship::KeyAgreement) => core_doc
        .try_resolve_method_with_scope(&fragment, MethodScope::key_agreement())
        .is_ok(),
      MethodScope::VerificationRelationship(MethodRelationship::CapabilityDelegation) => core_doc
        .try_resolve_method_with_scope(&fragment, MethodScope::capability_delegation())
        .is_ok(),
      MethodScope::VerificationRelationship(MethodRelationship::CapabilityInvocation) => core_doc
        .try_resolve_method_with_scope(&fragment, MethodScope::capability_invocation())
        .is_ok(),
      _ => unreachable!(),
    };

    assert!(contains);
  }

  Ok(())
}

#[tokio::test]
async fn test_create_method_duplicate_fragment() -> Result<()> {
  let mut account_setup = account_setup();
  account_setup.config = account_setup.config.testmode(true).autopublish(false);

  let mut account = Account::create_identity(account_setup, IdentitySetup::default())
    .await
    .unwrap();

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "key-1".to_owned(),
  };

  account.process_update(update.clone()).await.unwrap();

  let output = account.process_update(update.clone()).await;

  // Attempting to add a method with the same fragment in the same int and diff generation.
  assert!(matches!(
    output.unwrap_err(),
    Error::UpdateError(UpdateError::DuplicateKeyLocation(_)),
  ));

  // This increments the generation internally.
  account.publish_updates().await?;

  let output = account.process_update(update).await;

  // Now the location is different due to the incremented generation, but the fragment is the same.
  assert!(matches!(
    output.unwrap_err(),
    Error::DIDError(identity_did::Error::MethodAlreadyExists)
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

  account.process_update(update).await?;

  let state: &IdentityState = account.state();

  assert!(state.document().resolve_method(&fragment).is_some());

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

  let err = account.process_update(update).await.unwrap_err();

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

  account.process_update(update).await?;

  // One relationship by default.
  assert_eq!(
    account
      .state()
      .document()
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
  let update: Update = Update::AttachMethodRelationship {
    relationships: vec![MethodRelationship::AssertionMethod, MethodRelationship::KeyAgreement],
    fragment: default_method_fragment,
  };

  let err = account.process_update(update).await.unwrap_err();

  assert!(matches!(
    err,
    Error::IotaError(identity_iota::Error::InvalidDoc(
      identity_did::Error::InvalidMethodEmbedded
    ))
  ));

  // No relationships were created.
  assert_eq!(account.document().as_document().verification_relationships().count(), 1);

  assert_eq!(account.document().as_document().assertion_method().iter().count(), 0);
  assert_eq!(account.document().as_document().key_agreement().iter().count(), 0);

  let update: Update = Update::AttachMethodRelationship {
    relationships: vec![MethodRelationship::AssertionMethod, MethodRelationship::KeyAgreement],
    fragment: fragment.clone(),
  };

  account.process_update(update).await?;

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
    scope: MethodScope::authentication(),
    method_secret: None,
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: embedded_fragment.clone(),
  };

  account.process_update(update).await?;

  // Attempt detaching a relationship from an embedded method.
  let update: Update = Update::DetachMethodRelationship {
    relationships: vec![MethodRelationship::Authentication],
    fragment: embedded_fragment,
  };

  let err = account.process_update(update).await.unwrap_err();

  assert!(matches!(
    err,
    Error::IotaError(identity_iota::Error::InvalidDoc(
      identity_did::Error::InvalidMethodEmbedded
    ))
  ));

  // No relationships were removed.
  assert_eq!(account.document().as_document().verification_relationships().count(), 2);

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: generic_fragment.clone(),
  };

  account.process_update(update).await?;

  let update: Update = Update::AttachMethodRelationship {
    relationships: vec![MethodRelationship::AssertionMethod, MethodRelationship::KeyAgreement],
    fragment: generic_fragment.clone(),
  };

  account.process_update(update).await?;

  assert_eq!(account.document().as_document().assertion_method().len(), 1);
  assert_eq!(account.document().as_document().key_agreement().len(), 1);

  let update: Update = Update::DetachMethodRelationship {
    relationships: vec![MethodRelationship::AssertionMethod, MethodRelationship::KeyAgreement],
    fragment: generic_fragment.clone(),
  };

  account.process_update(update).await?;

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

  let err = account.process_update(update).await.unwrap_err();

  assert!(matches!(err, Error::UpdateError(UpdateError::InvalidMethodSecret(_))));

  let key_collection = KeyCollection::new_ed25519(4).unwrap();

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: Some(MethodSecret::MerkleKeyCollection(key_collection)),
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "key-2".to_owned(),
  };

  let err = account.process_update(update).await.unwrap_err();

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

  account.process_update(update).await?;

  // Ensure it was added.
  assert!(account.state().document().resolve_method(&fragment).is_some());

  let update: Update = Update::DeleteMethod {
    fragment: "key-1".to_owned(),
  };

  account.process_update(update.clone()).await?;

  let state: &IdentityState = account.state();

  // Ensure it no longer exists.
  assert!(state.document().resolve_method(&fragment).is_none());

  // Still only the default relationship.
  assert_eq!(state.document().as_document().verification_relationships().count(), 1);

  assert_eq!(state.document().as_document().methods().count(), 1);

  let location = state.method_location(method_type, fragment.clone()).unwrap();

  // Ensure the key still exists in storage - deletion in storage happens after successful publication.
  assert!(account.storage().key_exists(account.did(), &location).await.unwrap());

  // Ensure `created` wasn't updated.
  assert_eq!(initial_state.document().created(), state.document().created());
  // Ensure `updated` was recently set.
  assert!(state.document().updated() > Timestamp::from_unix(Timestamp::now_utc().to_unix() - 15));

  // Deleting a non-existing methods fails.
  let output = account.process_update(update).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::IotaError(identity_iota::Error::InvalidDoc(identity_did::Error::MethodNotFound))
  ));

  Ok(())
}

#[tokio::test]
async fn test_insert_service() -> Result<()> {
  let mut account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

  assert_eq!(account.document().service().len(), 0);

  let fragment = "#service-42".to_owned();

  let update: Update = Update::CreateService {
    fragment: fragment.clone(),
    type_: "LinkedDomains".to_owned(),
    endpoint: ServiceEndpoint::One(Url::parse("https://iota.org").unwrap()),
    properties: None,
  };

  account.process_update(update.clone()).await?;

  assert_eq!(account.document().service().len(), 1);

  // Ensure the service can be queried.
  let service_url = account.did().to_url().join(fragment).unwrap();
  assert!(account.document().service().query(service_url).is_some());

  let err = account.process_update(update.clone()).await.unwrap_err();

  assert!(matches!(
    err,
    Error::UpdateError(UpdateError::DuplicateServiceFragment(_))
  ));

  Ok(())
}

#[tokio::test]
async fn test_remove_service() -> Result<()> {
  let mut account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

  let fragment = "#service-42".to_owned();

  let update: Update = Update::CreateService {
    fragment: fragment.clone(),
    type_: "LinkedDomains".to_owned(),
    endpoint: ServiceEndpoint::One(Url::parse("https://iota.org").unwrap()),
    properties: None,
  };

  account.process_update(update).await.unwrap();

  assert_eq!(account.document().service().len(), 1);

  let update: Update = Update::DeleteService {
    fragment: fragment.clone(),
  };

  account.process_update(update.clone()).await.unwrap();

  assert_eq!(account.document().service().len(), 0);

  // Attempting to remove a non-existing service returns an error.
  let err = account.process_update(update).await.unwrap_err();

  assert!(matches!(err, Error::UpdateError(UpdateError::ServiceNotFound)));

  Ok(())
}
