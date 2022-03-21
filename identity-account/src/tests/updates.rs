// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use identity_account_storage::storage::MemStore;
use identity_account_storage::types::method_to_key_type;
use identity_account_storage::types::IotaVerificationMethodExt;
use identity_account_storage::types::KeyLocation;
use identity_core::common::OneOrSet;
use identity_core::common::OrderedSet;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_did::did::DID;
use identity_did::service::ServiceEndpoint;
use identity_did::utils::Queryable;
use identity_did::verification::MethodRelationship;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_iota::tangle::ClientBuilder;
use identity_iota_core::did::IotaDID;
use identity_iota_core::document::IotaDocument;
use identity_iota_core::document::IotaVerificationMethod;
use identity_iota_core::tangle::Network;

use crate::account::Account;
use crate::account::AccountConfig;
use crate::account::AccountSetup;
use crate::error::Error;
use crate::error::Result;
use crate::identity::IdentitySetup;
use crate::types::MethodSecret;
use crate::updates::Update;
use crate::updates::UpdateError;

async fn account_setup(network: Network) -> AccountSetup {
  AccountSetup::new(
    Arc::new(MemStore::new()),
    Arc::new(
      ClientBuilder::new()
        .network(network)
        .node_sync_disabled()
        .build()
        .await
        .unwrap(),
    ),
    AccountConfig::new().testmode(true),
  )
}

#[tokio::test]
async fn test_create_identity() -> Result<()> {
  let account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default())
    .await
    .unwrap();

  let document: &IotaDocument = account.document();

  let expected_fragment = IotaDocument::DEFAULT_METHOD_FRAGMENT;
  let method: &IotaVerificationMethod = document.resolve_method(expected_fragment).unwrap();

  assert_eq!(document.core_document().verification_relationships().count(), 1);
  assert_eq!(document.core_document().methods().count(), 1);

  let location: KeyLocation = method.key_location().unwrap();

  // Ensure we can retrieve the correct location for the key.
  assert_eq!(
    location,
    KeyLocation::new(KeyType::Ed25519, expected_fragment.to_owned(), method.key_data())
  );

  // Ensure the key exists in storage.
  assert!(account
    .storage()
    .key_exists(&account.account_id, &location)
    .await
    .unwrap());

  // Enure the state was written to storage.
  assert!(account.load_document().await.is_ok());

  // Ensure timestamps were recently set.
  assert!(document.metadata.created > Timestamp::from_unix(Timestamp::now_utc().to_unix() - 15).unwrap());
  assert!(document.metadata.updated > Timestamp::from_unix(Timestamp::now_utc().to_unix() - 15).unwrap());

  // Ensure the DID was added to the index.
  assert_eq!(
    account.storage().index_get(account.did()).await.unwrap().unwrap(),
    account.account_id
  );

  Ok(())
}

#[tokio::test]
async fn test_create_identity_network() -> Result<()> {
  // Ensure created identities match the client network.

  // Mainnet
  let account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default()).await?;
  assert_eq!(account.did().network_str(), Network::Mainnet.name_str());

  // Devnet
  let account = Account::create_identity(account_setup(Network::Devnet).await, IdentitySetup::default()).await?;
  assert_eq!(account.did().network_str(), Network::Devnet.name_str());

  // Private Tangle
  let account = Account::create_identity(
    AccountSetup::new(
      Arc::new(MemStore::new()),
      Arc::new(
        ClientBuilder::new()
          .network(Network::try_from_name("custom")?)
          .node("http://127.0.0.1:8082")?
          .node_sync_disabled()
          .build()
          .await
          .unwrap(),
      ),
      AccountConfig::new().testmode(true),
    ),
    IdentitySetup::default(),
  )
  .await?;
  assert_eq!(account.did().network_str(), "custom");

  Ok(())
}

#[tokio::test]
async fn test_create_identity_already_exists() -> Result<()> {
  let keypair = KeyPair::new_ed25519()?;
  let identity_create = IdentitySetup::default()
    .key_type(KeyType::Ed25519)
    .method_secret(MethodSecret::Ed25519(keypair.private().clone()));
  let account_setup = account_setup(Network::Mainnet).await;

  let account = Account::create_identity(account_setup.clone(), identity_create.clone())
    .await
    .unwrap();

  let initial_state = account_setup
    .storage
    .document(&account.account_id)
    .await
    .unwrap()
    .unwrap();

  let output = Account::create_identity(account_setup.clone(), identity_create).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::UpdateError(UpdateError::DocumentAlreadyExists),
  ));

  // Ensure nothing was overwritten in storage
  assert_eq!(
    initial_state,
    account_setup.storage.document(&account.account_id).await?.unwrap()
  );

  Ok(())
}

#[tokio::test]
async fn test_create_identity_from_invalid_private_key() -> Result<()> {
  let private_bytes: Box<[u8]> = Box::new([0; 33]);
  let private_key: PrivateKey = PrivateKey::from(private_bytes);

  let id_create = IdentitySetup::new()
    .key_type(KeyType::Ed25519)
    .method_secret(MethodSecret::Ed25519(private_key));

  let err = Account::create_identity(account_setup(Network::Mainnet).await, id_create)
    .await
    .unwrap_err();

  assert!(matches!(err, Error::UpdateError(UpdateError::InvalidMethodSecret(_))));

  Ok(())
}

#[tokio::test]
async fn test_create_method() -> Result<()> {
  let mut account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default())
    .await
    .unwrap();

  let initial_document: IotaDocument = account.document().to_owned();
  let method_type = MethodType::Ed25519VerificationKey2018;

  let fragment = "key-1".to_owned();
  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: method_type,
    fragment: fragment.clone(),
  };

  account.process_update(update).await.unwrap();

  let document: &IotaDocument = account.document();

  let method: &IotaVerificationMethod = document.resolve_method(&fragment).unwrap();

  // Ensure existence and key type
  assert_eq!(method.key_type(), method_type);

  // Still only the default relationship.
  assert_eq!(document.core_document().verification_relationships().count(), 1);
  assert_eq!(document.core_document().methods().count(), 2);

  let location = method.key_location().unwrap();

  // Ensure we can retrieve the correct location for the key.
  assert_eq!(
    location,
    KeyLocation::new(method_to_key_type(method_type), fragment, method.key_data())
  );

  // Ensure the key exists in storage.
  assert!(account
    .storage()
    .key_exists(&account.account_id, &location)
    .await
    .unwrap());

  // Ensure `created` wasn't updated.
  assert_eq!(initial_document.metadata.created, document.metadata.created);

  // Ensure `updated` was recently set.
  assert!(document.metadata.updated > Timestamp::from_unix(Timestamp::now_utc().to_unix() - 15).unwrap());

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
    let mut account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default()).await?;

    let fragment = "#key-1".to_owned();

    let update: Update = Update::CreateMethod {
      scope: *scope,
      method_secret: None,
      type_: MethodType::Ed25519VerificationKey2018,
      fragment: fragment.clone(),
    };

    account.process_update(update).await?;

    let document: &IotaDocument = account.document();

    assert_eq!(document.core_document().verification_relationships().count(), 2);

    assert_eq!(document.core_document().methods().count(), 2);

    let core_doc = document.core_document();

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
  let mut account_setup = account_setup(Network::Mainnet).await;
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

  // Attempting to add a method with the same fragment.
  assert!(matches!(
    output.unwrap_err(),
    Error::DIDError(identity_did::Error::MethodAlreadyExists)
  ));

  Ok(())
}

#[tokio::test]
async fn test_create_method_from_private_key() -> Result<()> {
  let mut account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default()).await?;

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

  let document: &IotaDocument = account.document();

  let method: &IotaVerificationMethod = document.resolve_method(&fragment).unwrap();

  let location = method.key_location().unwrap();
  let public_key = account.storage().key_public(&account.account_id, &location).await?;

  assert_eq!(public_key.as_ref(), keypair.public().as_ref());

  Ok(())
}

#[tokio::test]
async fn test_create_method_from_invalid_private_key() -> Result<()> {
  let mut account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default()).await?;

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
  let mut account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default()).await?;

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
    account.document().core_document().verification_relationships().count(),
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
    Error::IotaCoreError(identity_iota_core::Error::InvalidDoc(
      identity_did::Error::InvalidMethodEmbedded
    ))
  ));

  // No relationships were created.
  assert_eq!(
    account.document().core_document().verification_relationships().count(),
    1
  );

  assert_eq!(account.document().core_document().assertion_method().iter().count(), 0);
  assert_eq!(account.document().core_document().key_agreement().iter().count(), 0);

  let update: Update = Update::AttachMethodRelationship {
    relationships: vec![MethodRelationship::AssertionMethod, MethodRelationship::KeyAgreement],
    fragment: fragment.clone(),
  };

  account.process_update(update).await?;

  // Relationships were created.
  assert_eq!(
    account.document().core_document().verification_relationships().count(),
    3
  );

  assert_eq!(account.document().core_document().assertion_method().len(), 1);
  assert_eq!(
    account
      .document()
      .core_document()
      .assertion_method()
      .first()
      .unwrap()
      .id()
      .fragment()
      .unwrap(),
    fragment
  );
  assert_eq!(account.document().core_document().key_agreement().len(), 1);
  assert_eq!(
    account
      .document()
      .core_document()
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
  let mut account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default()).await?;

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
    Error::IotaCoreError(identity_iota_core::Error::InvalidDoc(
      identity_did::Error::InvalidMethodEmbedded
    ))
  ));

  // No relationships were removed.
  assert_eq!(
    account.document().core_document().verification_relationships().count(),
    2
  );

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

  assert_eq!(account.document().core_document().assertion_method().len(), 1);
  assert_eq!(account.document().core_document().key_agreement().len(), 1);

  let update: Update = Update::DetachMethodRelationship {
    relationships: vec![MethodRelationship::AssertionMethod, MethodRelationship::KeyAgreement],
    fragment: generic_fragment.clone(),
  };

  account.process_update(update).await?;

  assert_eq!(account.document().core_document().assertion_method().len(), 0);
  assert_eq!(account.document().core_document().key_agreement().len(), 0);

  Ok(())
}

// TODO: With the change from `MethodType` to `KeyType` in `KeyLocation`, this test is fully broken.
// #[tokio::test]
// async fn test_create_method_with_type_secret_mismatch() -> Result<()> {
//   let mut account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default()).await?;

//   let private_bytes: Box<[u8]> = Box::new([0; 32]);
//   let private_key = PrivateKey::from(private_bytes);

//   let update: Update = Update::CreateMethod {
//     scope: MethodScope::default(),
//     method_secret: Some(MethodSecret::Ed25519(private_key)),
//     type_: MethodType::MerkleKeyCollection2021,
//     fragment: "key-1".to_owned(),
//   };

//   let err = account.process_update(update).await.unwrap_err();

//   assert!(matches!(err, Error::UpdateError(UpdateError::InvalidMethodSecret(_))));

//   let key_collection = KeyCollection::new_ed25519(4).unwrap();

//   let update: Update = Update::CreateMethod {
//     scope: MethodScope::default(),
//     method_secret: Some(MethodSecret::MerkleKeyCollection(key_collection)),
//     type_: MethodType::Ed25519VerificationKey2018,
//     fragment: "key-2".to_owned(),
//   };

//   let err = account.process_update(update).await.unwrap_err();

//   assert!(matches!(err, Error::UpdateError(UpdateError::InvalidMethodSecret(_))));

//   Ok(())
// }

#[tokio::test]
async fn test_delete_method() -> Result<()> {
  let mut account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default()).await?;

  let fragment = "key-1".to_owned();
  let method_type = MethodType::Ed25519VerificationKey2018;
  let initial_document = account.document().to_owned();

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: method_type,
    fragment: fragment.clone(),
  };

  account.process_update(update).await?;

  // Ensure it was added.
  let method: &IotaVerificationMethod = account.document().resolve_method(&fragment).unwrap();
  let location = method.key_location().unwrap();

  let update: Update = Update::DeleteMethod {
    fragment: "key-1".to_owned(),
  };

  account.process_update(update.clone()).await?;

  let document: &IotaDocument = account.document();

  // Ensure it no longer exists.
  assert!(document.resolve_method(&fragment).is_none());

  // Still only the default relationship.
  assert_eq!(document.core_document().verification_relationships().count(), 1);

  assert_eq!(document.core_document().methods().count(), 1);

  // Ensure the key still exists in storage.
  assert!(account
    .storage()
    .key_exists(&account.account_id, &location)
    .await
    .unwrap());

  // Ensure `created` wasn't updated.
  assert_eq!(initial_document.metadata.created, document.metadata.created);
  // Ensure `updated` was recently set.
  assert!(document.metadata.updated > Timestamp::from_unix(Timestamp::now_utc().to_unix() - 15).unwrap());

  // Deleting a non-existing methods fails.
  let output = account.process_update(update).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::IotaCoreError(identity_iota_core::Error::InvalidDoc(
      identity_did::Error::MethodNotFound
    ))
  ));

  Ok(())
}

#[tokio::test]
async fn test_insert_service() -> Result<()> {
  let mut account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default()).await?;

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
  let mut account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default()).await?;

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

#[tokio::test]
async fn test_set_controller() -> Result<()> {
  let mut account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default()).await?;

  let keypair1: KeyPair = KeyPair::new_ed25519().unwrap();
  let iota_did1: IotaDID = IotaDID::new(keypair1.public().as_ref()).unwrap();

  let keypair2: KeyPair = KeyPair::new_ed25519().unwrap();
  let iota_did2: IotaDID = IotaDID::new(keypair2.public().as_ref()).unwrap();

  // Set one controller.
  let update: Update = Update::SetController {
    controllers: Some(OneOrSet::new_one(iota_did1.clone())),
  };
  account.process_update(update).await.unwrap();
  assert_eq!(account.document().controller().unwrap().len(), 1);

  // Set two controllers.
  let set: OrderedSet<IotaDID> = OrderedSet::from_iter(vec![iota_did1, iota_did2]);
  let update: Update = Update::SetController {
    controllers: Some(OneOrSet::new_set(set).unwrap()),
  };
  account.process_update(update).await.unwrap();
  assert_eq!(account.document().controller().unwrap().len(), 2);

  // Remove all controllers.
  let update: Update = Update::SetController { controllers: None };
  account.process_update(update).await.unwrap();
  assert_eq!(account.document().controller(), None);

  Ok(())
}

#[tokio::test]
async fn test_set_also_known_as() -> Result<()> {
  let mut account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default()).await?;

  // No elements by default.
  assert_eq!(account.document().also_known_as().len(), 0);

  // Set two Urls.
  let urls: OrderedSet<Url> = OrderedSet::from_iter(vec![
    Url::parse("did:iota:xyz").unwrap(),
    Url::parse("did:iota:abc").unwrap(),
  ]);
  let update: Update = Update::SetAlsoKnownAs { urls };
  account.process_update(update).await.unwrap();
  assert_eq!(account.document().also_known_as().len(), 2);

  // Remove all Urls.
  let update: Update = Update::SetAlsoKnownAs {
    urls: OrderedSet::new(),
  };
  account.process_update(update).await.unwrap();
  assert_eq!(account.document().also_known_as().len(), 0);

  Ok(())
}
