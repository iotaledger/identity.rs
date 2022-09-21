// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use identity_account_storage::storage::MemStore;
use identity_account_storage::types::KeyLocation;
use identity_core::common::OneOrSet;
use identity_core::common::OrderedSet;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FromJson;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_did::did::DID;
use identity_did::service::ServiceEndpoint;
use identity_did::utils::Queryable;
use identity_did::verification::MethodRelationship;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_iota_client_legacy::tangle::ClientBuilder;
use identity_iota_core_legacy::did::IotaDID;
use identity_iota_core_legacy::document::IotaDocument;
use identity_iota_core_legacy::document::IotaVerificationMethod;
use identity_iota_core_legacy::tangle::Network;

use crate::account::Account;
use crate::account::AccountConfig;
use crate::account::AccountSetup;
use crate::error::Error;
use crate::error::Result;
use crate::types::IdentitySetup;
use crate::types::IdentityState;
use crate::types::MethodContent;
use crate::updates::Update;
use crate::updates::UpdateError;

use super::util::*;

#[tokio::test]
async fn test_create_identity() -> Result<()> {
  for storage in storages().await {
    let account = Account::create_identity(
      account_setup_storage(storage, Network::Mainnet).await,
      IdentitySetup::default(),
    )
    .await
    .unwrap();

    let document: &IotaDocument = account.document();

    let expected_fragment = IotaDocument::DEFAULT_METHOD_FRAGMENT;
    let method: &IotaVerificationMethod = document.resolve_method(expected_fragment, None).unwrap();

    assert_eq!(document.core_document().verification_relationships().count(), 1);
    assert_eq!(document.core_document().methods(None).len(), 1);

    let location: KeyLocation = KeyLocation::from_verification_method(method).unwrap();

    // Ensure we can retrieve the correct location for the key.
    assert_eq!(
      location,
      KeyLocation::new(
        KeyType::Ed25519,
        expected_fragment.to_owned(),
        method.data().try_decode().unwrap().as_ref()
      )
    );

    // Ensure the key exists in storage.
    assert!(account
      .storage()
      .key_exists(account.did().as_ref(), &location)
      .await
      .unwrap());

    // Ensure the state was written to storage.
    assert!(account.load_document().await.is_ok());

    // Ensure timestamps were recently set.
    assert!(document.metadata.created.unwrap() > Timestamp::from_unix(Timestamp::now_utc().to_unix() - 15).unwrap());
    assert!(document.metadata.updated.unwrap() > Timestamp::from_unix(Timestamp::now_utc().to_unix() - 15).unwrap());

    // Ensure the DID was added to the index.
    assert!(account.storage().did_exists(account.did().as_ref()).await.unwrap());
  }

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
  for storage in storages().await {
    let keypair = KeyPair::new(KeyType::Ed25519)?;
    let identity_create = IdentitySetup::default().private_key(keypair.private().clone());

    let account_setup = account_setup_storage(storage, Network::Mainnet).await;

    let account = Account::create_identity(account_setup.clone(), identity_create.clone())
      .await
      .unwrap();

    let initial_state: Vec<u8> = account_setup.storage.blob_get(account.did().as_ref()).await?.unwrap();
    let initial_state: IdentityState = IdentityState::from_json_slice(&initial_state).unwrap();

    let output = Account::create_identity(account_setup.clone(), identity_create).await;

    assert!(matches!(
      output.unwrap_err(),
      Error::AccountCoreError(identity_account_storage::Error::IdentityAlreadyExists)
    ));

    // Ensure nothing was overwritten in storage
    let account_state: Vec<u8> = account_setup.storage.blob_get(account.did().as_ref()).await?.unwrap();
    let account_state: IdentityState = IdentityState::from_json_slice(&account_state).unwrap();
    assert_eq!(initial_state.document()?, account_state.document()?);
  }
  Ok(())
}

#[tokio::test]
async fn test_create_identity_from_invalid_private_key() -> Result<()> {
  let private_bytes: Box<[u8]> = Box::new([0; 33]);
  let private_key: PrivateKey = PrivateKey::from(private_bytes);

  let id_create = IdentitySetup::new().private_key(private_key);

  let err = Account::create_identity(account_setup(Network::Mainnet).await, id_create)
    .await
    .unwrap_err();

  assert!(matches!(err, Error::UpdateError(UpdateError::InvalidMethodContent(_))));

  Ok(())
}

#[tokio::test]
async fn test_create_method_content_generate() -> Result<()> {
  for storage in storages().await {
    for method_content in [MethodContent::GenerateEd25519, MethodContent::GenerateX25519] {
      let mut account: Account = Account::create_identity(
        account_setup_storage(Arc::clone(&storage), Network::Mainnet).await,
        IdentitySetup::default(),
      )
      .await?;

      let initial_document: IotaDocument = account.document().to_owned();

      let method_type: MethodType = method_content.method_type();
      let key_type: KeyType = method_content.key_type();
      let fragment = "key-1".to_owned();
      let update: Update = Update::CreateMethod {
        scope: MethodScope::default(),
        content: method_content,
        fragment: fragment.clone(),
      };

      account.process_update(update).await.unwrap();

      let document: &IotaDocument = account.document();

      // Ensure existence.
      let method: &IotaVerificationMethod = document.resolve_method(&fragment, None).unwrap();

      // Ensure key type.
      assert_eq!(method.type_(), method_type);

      // Still only the default relationship.
      assert_eq!(document.core_document().verification_relationships().count(), 1);
      assert_eq!(document.core_document().methods(None).len(), 2);

      let location: KeyLocation = KeyLocation::from_verification_method(method).unwrap();

      // Ensure we can retrieve the correct location for the key.
      assert_eq!(
        location,
        KeyLocation::new(key_type, fragment, method.data().try_decode().unwrap().as_ref())
      );

      // Ensure the key exists in storage.
      assert!(account
        .storage()
        .key_exists(account.did().as_ref(), &location)
        .await
        .unwrap());

      // Ensure `created` wasn't updated.
      assert_eq!(initial_document.metadata.created, document.metadata.created);

      // Ensure `updated` was recently set.
      assert!(document.metadata.updated.unwrap() > Timestamp::from_unix(Timestamp::now_utc().to_unix() - 15).unwrap());
    }
  }
  Ok(())
}

#[tokio::test]
async fn test_create_method_content_public() -> Result<()> {
  let bytes: [u8; 32] = [1_u8; 32];
  for method_content in [
    MethodContent::PublicX25519(PublicKey::from(bytes.to_vec())),
    MethodContent::PublicEd25519(PublicKey::from(bytes.to_vec())),
  ] {
    let mut account: Account =
      Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default()).await?;

    let method_type: MethodType = method_content.method_type();
    let fragment = "key-1".to_owned();
    let update: Update = Update::CreateMethod {
      scope: MethodScope::default(),
      content: method_content,
      fragment: fragment.clone(),
    };
    account.process_update(update).await?;

    // Ensure existence and type.
    let method: &IotaVerificationMethod = account.document().resolve_method(&fragment, None).unwrap();
    assert_eq!(method.type_(), method_type);
    assert_eq!(method.data().try_decode().unwrap().as_slice(), bytes);

    // Ensure no key exists in storage.
    let location: KeyLocation = KeyLocation::from_verification_method(method).unwrap();
    assert!(!account
      .storage()
      .key_exists(account.did().as_ref(), &location)
      .await
      .unwrap());
  }
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
      content: MethodContent::GenerateEd25519,
      fragment: fragment.clone(),
    };

    account.process_update(update).await?;

    let document: &IotaDocument = account.document();

    assert_eq!(document.core_document().verification_relationships().count(), 2);

    assert_eq!(document.core_document().methods(None).len(), 2);

    let core_doc = document.core_document();

    let contains = match scope {
      MethodScope::VerificationRelationship(MethodRelationship::Authentication) => core_doc
        .resolve_method(&fragment, Some(MethodScope::authentication()))
        .is_some(),
      MethodScope::VerificationRelationship(MethodRelationship::AssertionMethod) => core_doc
        .resolve_method(&fragment, Some(MethodScope::assertion_method()))
        .is_some(),
      MethodScope::VerificationRelationship(MethodRelationship::KeyAgreement) => core_doc
        .resolve_method(&fragment, Some(MethodScope::key_agreement()))
        .is_some(),
      MethodScope::VerificationRelationship(MethodRelationship::CapabilityDelegation) => core_doc
        .resolve_method(&fragment, Some(MethodScope::capability_delegation()))
        .is_some(),
      MethodScope::VerificationRelationship(MethodRelationship::CapabilityInvocation) => core_doc
        .resolve_method(&fragment, Some(MethodScope::capability_invocation()))
        .is_some(),
      _ => unreachable!(),
    };

    assert!(contains);
  }

  Ok(())
}

#[tokio::test]
async fn test_create_method_duplicate_fragment() {
  let mut account_setup = account_setup(Network::Mainnet).await;
  account_setup.config = account_setup.config.testmode(true).autopublish(false);

  let mut account = Account::create_identity(account_setup, IdentitySetup::default())
    .await
    .unwrap();

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    content: MethodContent::GenerateEd25519,
    fragment: "key-1".to_owned(),
  };

  account.process_update(update.clone()).await.unwrap();

  let output = account.process_update(update.clone()).await;

  // Attempting to add a method with the same fragment.
  assert!(matches!(
    output.unwrap_err(),
    Error::DIDError(identity_did::Error::MethodAlreadyExists)
  ));
}

#[tokio::test]
async fn test_create_method_from_private_key() {
  for storage in storages().await {
    let mut account = Account::create_identity(
      account_setup_storage(storage, Network::Mainnet).await,
      IdentitySetup::default(),
    )
    .await
    .unwrap();

    let keypair = KeyPair::new(KeyType::Ed25519).unwrap();
    let fragment = "key-1".to_owned();

    let update: Update = Update::CreateMethod {
      scope: MethodScope::default(),
      content: MethodContent::PrivateEd25519(keypair.private().clone()),
      fragment: fragment.clone(),
    };

    account.process_update(update).await.unwrap();

    let document: &IotaDocument = account.document();

    let method: &IotaVerificationMethod = document.resolve_method(&fragment, None).unwrap();

    let location: KeyLocation = KeyLocation::from_verification_method(method).unwrap();

    let public_key = account
      .storage()
      .key_public(account.did().as_ref(), &location)
      .await
      .unwrap();

    assert_eq!(public_key.as_ref(), keypair.public().as_ref());
  }
}

#[tokio::test]
async fn test_create_method_from_invalid_private_key() -> Result<()> {
  let mut account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default()).await?;

  let private_bytes: Box<[u8]> = Box::new([0; 33]);
  let private_key = PrivateKey::from(private_bytes);

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    content: MethodContent::PrivateEd25519(private_key),
    fragment: "key-1".to_owned(),
  };

  let err = account.process_update(update).await.unwrap_err();

  assert!(matches!(err, Error::UpdateError(UpdateError::InvalidMethodContent(_))));

  Ok(())
}

#[tokio::test]
async fn test_attach_method_relationship() -> Result<()> {
  let mut account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default()).await?;

  let fragment = "key-1".to_owned();

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    content: MethodContent::GenerateEd25519,
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
    Error::IotaCoreError(identity_iota_core_legacy::Error::InvalidDoc(
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
    content: MethodContent::GenerateEd25519,
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
    Error::IotaCoreError(identity_iota_core_legacy::Error::InvalidDoc(
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
    content: MethodContent::GenerateEd25519,
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

#[tokio::test]
async fn test_delete_method() -> Result<()> {
  let mut account = Account::create_identity(account_setup(Network::Mainnet).await, IdentitySetup::default()).await?;

  let fragment = "key-1".to_owned();
  let initial_document = account.document().to_owned();
  let content: MethodContent = MethodContent::GenerateEd25519;

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    content: content.clone(),
    fragment: fragment.clone(),
  };

  account.process_update(update).await?;

  // Ensure it was added.
  let method: &IotaVerificationMethod = account.document().resolve_method(&fragment, None).unwrap();
  let location: KeyLocation = KeyLocation::from_verification_method(method).unwrap();

  let update: Update = Update::DeleteMethod {
    fragment: "key-1".to_owned(),
  };

  account.process_update(update.clone()).await?;

  let document: &IotaDocument = account.document();

  // Ensure it no longer exists.
  assert!(document.resolve_method(&fragment, None).is_none());

  // Still only the default relationship.
  assert_eq!(document.core_document().verification_relationships().count(), 1);

  assert_eq!(document.core_document().methods(None).len(), 1);

  // Ensure the key still exists in storage.
  assert!(account
    .storage()
    .key_exists(account.did().as_ref(), &location)
    .await
    .unwrap());

  // Ensure `created` wasn't updated.
  assert_eq!(initial_document.metadata.created, document.metadata.created);
  // Ensure `updated` was recently set.
  assert!(document.metadata.updated.unwrap() > Timestamp::from_unix(Timestamp::now_utc().to_unix() - 15).unwrap());

  // Deleting a non-existing methods fails.
  let output = account.process_update(update).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::IotaCoreError(identity_iota_core_legacy::Error::InvalidDoc(
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
    types: vec!["LinkedDomains".to_owned()],
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
    types: vec!["LinkedDomains".to_owned()],
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

  let keypair1: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
  let iota_did1: IotaDID = IotaDID::new(keypair1.public().as_ref()).unwrap();

  let keypair2: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
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
