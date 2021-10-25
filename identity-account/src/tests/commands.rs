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
use crate::identity::IdentitySnapshot;
use crate::identity::TinyMethod;
use crate::storage::MemStore;
use crate::types::Generation;
use crate::types::MethodSecret;
use identity_core::common::UnixTimestamp;
use identity_core::crypto::KeyCollection;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;

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

  let snapshot: IdentitySnapshot = account.load_snapshot().await?;

  assert_eq!(snapshot.sequence(), Generation::from_u32(4));
  assert!(snapshot.identity().did().is_some());
  assert_ne!(snapshot.identity().created(), UnixTimestamp::EPOCH);
  assert_ne!(snapshot.identity().updated(), UnixTimestamp::EPOCH);

  Ok(())
}

#[tokio::test]
async fn test_create_identity_network() -> Result<()> {
  // Create an identity with a valid network string
  let create_identity: IdentitySetup = IdentitySetup::new().network("dev")?.key_type(KeyType::Ed25519);
  let account = Account::create_identity(account_setup(), create_identity).await?;

  // Ensure the identity creation was successful
  assert!(account.state().await?.authentication().is_ok());

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

  assert_eq!(account.load_snapshot().await?.sequence(), Generation::from(4));

  let output = Account::create_identity(account_setup, identity_create).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::UpdateError(UpdateError::DocumentAlreadyExists),
  ));

  // version is still 4, no events have been committed
  assert_eq!(account.load_snapshot().await?.sequence(), Generation::from(4));

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
  let account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "key-1".to_owned(),
  };

  account.process_update(update, false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot().await?;

  assert_eq!(snapshot.sequence(), Generation::from_u32(6));
  assert!(snapshot.identity().did().is_some());
  assert_ne!(snapshot.identity().created(), UnixTimestamp::EPOCH);
  assert_ne!(snapshot.identity().updated(), UnixTimestamp::EPOCH);
  assert_eq!(snapshot.identity().methods().len(), 2);

  let method: &TinyMethod = snapshot.identity().methods().fetch("key-1")?;

  assert_eq!(method.location().fragment(), "key-1");
  assert_eq!(method.location().method(), MethodType::Ed25519VerificationKey2018);

  Ok(())
}

#[tokio::test]
async fn test_create_method_reserved_fragment() -> Result<()> {
  let account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "_sign-123".to_owned(),
  };

  let snapshot: IdentitySnapshot = account.load_snapshot().await?;

  // version is now 4
  assert_eq!(snapshot.sequence(), Generation::from_u32(4));

  let output: _ = account.process_update(update, false).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::UpdateError(UpdateError::InvalidMethodFragment(_)),
  ));

  let snapshot: IdentitySnapshot = account.load_snapshot().await?;

  // version is still 4, no new events have been committed
  assert_eq!(snapshot.sequence(), Generation::from_u32(4));

  Ok(())
}

#[tokio::test]
async fn test_create_method_duplicate_fragment() -> Result<()> {
  let account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "key-1".to_owned(),
  };

  let snapshot: IdentitySnapshot = account.load_snapshot().await?;
  assert_eq!(snapshot.sequence(), Generation::from_u32(4));

  account.process_update(update.clone(), false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot().await?;
  assert_eq!(snapshot.sequence(), Generation::from_u32(6));

  let output: _ = account.process_update(update, false).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::UpdateError(UpdateError::DuplicateKeyFragment(_)),
  ));

  let snapshot: IdentitySnapshot = account.load_snapshot().await?;
  assert_eq!(snapshot.sequence(), Generation::from_u32(6));

  Ok(())
}

#[tokio::test]
async fn test_create_method_from_private_key() -> Result<()> {
  let account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

  let keypair = KeyPair::new_ed25519()?;

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: Some(MethodSecret::Ed25519(keypair.private().clone())),
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "key-1".to_owned(),
  };

  account.process_update(update, false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot().await?;

  let method: &TinyMethod = snapshot.identity().methods().fetch("key-1")?;

  let public_key = account.storage().key_get(account.did(), method.location()).await?;

  assert_eq!(public_key.as_ref(), keypair.public().as_ref());

  Ok(())
}

#[tokio::test]
async fn test_create_method_from_invalid_private_key() -> Result<()> {
  let account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

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
async fn test_create_method_with_type_secret_mismatch() -> Result<()> {
  let account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

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
  let account = Account::create_identity(account_setup(), IdentitySetup::default()).await?;

  let update: Update = Update::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "key-1".to_owned(),
  };

  let snapshot: IdentitySnapshot = account.load_snapshot().await?;
  assert_eq!(snapshot.sequence(), Generation::from_u32(4));

  account.process_update(update, false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot().await?;

  assert_eq!(snapshot.sequence(), Generation::from_u32(6));
  assert_eq!(snapshot.identity().methods().len(), 2);
  assert!(snapshot.identity().methods().contains("key-1"));
  assert!(snapshot.identity().methods().get("key-1").is_some());
  assert!(snapshot.identity().methods().fetch("key-1").is_ok());

  let update: Update = Update::DeleteMethod {
    fragment: "key-1".to_owned(),
  };

  account.process_update(update, false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot().await?;

  assert_eq!(snapshot.sequence(), Generation::from_u32(8));
  assert_eq!(snapshot.identity().methods().len(), 1);
  assert!(!snapshot.identity().methods().contains("key-1"));
  assert!(snapshot.identity().methods().get("key-1").is_none());
  assert!(snapshot.identity().methods().fetch("key-1").is_err());

  Ok(())
}
