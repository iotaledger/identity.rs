// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::Account;
use crate::account::Config;
use crate::error::Error;
use crate::error::Result;
use crate::events::Command;
use crate::events::UpdateError;
use crate::identity::IdentityCreate;
use crate::identity::IdentityId;
use crate::identity::IdentitySnapshot;
use crate::identity::IdentityState;
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

async fn new_account() -> Result<Account> {
  let store: MemStore = MemStore::new();
  let config: Config = Config::new().testmode(true);

  Account::with_config(store, config).await
}

#[tokio::test]
async fn test_create_identity() -> Result<()> {
  let account: Account = new_account().await?;
  let identity: IdentityId = IdentityId::from_u32(1);
  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  assert_eq!(snapshot.sequence(), Generation::new());
  assert_eq!(snapshot.id(), identity);
  assert!(snapshot.identity().did().is_none());
  assert_eq!(snapshot.identity().created(), UnixTimestamp::EPOCH);
  assert_eq!(snapshot.identity().updated(), UnixTimestamp::EPOCH);

  let command: Command = Command::CreateIdentity {
    network: None,
    method_secret: None,
    authentication: MethodType::Ed25519VerificationKey2018,
  };

  account.process(identity, command, false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  assert_eq!(snapshot.sequence(), Generation::from_u32(4));
  assert_eq!(snapshot.id(), identity);
  assert!(snapshot.identity().did().is_some());
  assert_ne!(snapshot.identity().created(), UnixTimestamp::EPOCH);
  assert_ne!(snapshot.identity().updated(), UnixTimestamp::EPOCH);

  Ok(())
}

#[tokio::test]
async fn test_create_identity_invalid_method() -> Result<()> {
  const TYPES: &[MethodType] = &[MethodType::MerkleKeyCollection2021];

  let account: Account = new_account().await?;
  let identity: IdentityId = IdentityId::from_u32(1);
  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  // initial snapshot version = 0
  assert_eq!(snapshot.sequence(), Generation::new());

  for type_ in TYPES.iter().copied() {
    let command: Command = Command::CreateIdentity {
      network: None,
      method_secret: None,
      authentication: type_,
    };

    let output: Result<()> = account.process(identity, command, false).await;

    assert!(matches!(
      output.unwrap_err(),
      Error::UpdateError(UpdateError::InvalidMethodType(_))
    ));

    let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

    // version is still 0, no events have been committed
    assert_eq!(snapshot.sequence(), Generation::new());
  }

  Ok(())
}

#[tokio::test]
async fn test_create_identity_network() -> Result<()> {
  let account: Account = new_account().await?;

  // Create an identity with a valid network string
  let create_identity: IdentityCreate = IdentityCreate::new().network("dev")?.key_type(KeyType::Ed25519);
  let identity: IdentityState = account.create_identity(create_identity).await?;

  // Ensure the identity creation was successful
  assert!(identity.did().is_some());
  assert!(identity.authentication().is_ok());

  Ok(())
}

#[tokio::test]
async fn test_create_identity_invalid_network() -> Result<()> {
  // Attempt to create an identity with an invalid network string
  let result: Result<IdentityCreate> = IdentityCreate::new().network("Invalid=Network!");

  // Ensure an `InvalidNetworkName` error is thrown
  assert!(matches!(
    result.unwrap_err(),
    Error::IotaError(identity_iota::Error::InvalidNetworkName),
  ));

  Ok(())
}

#[tokio::test]
async fn test_create_identity_already_exists() -> Result<()> {
  let account: Account = new_account().await?;
  let identity: IdentityId = IdentityId::from_u32(1);
  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  // initial snapshot version = 0
  assert_eq!(snapshot.sequence(), Generation::new());

  let command: Command = Command::CreateIdentity {
    network: None,
    method_secret: None,
    authentication: MethodType::Ed25519VerificationKey2018,
  };

  account.process(identity, command.clone(), false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  // version is now 4
  assert_eq!(snapshot.sequence(), Generation::from(4));

  let output: Result<()> = account.process(identity, command, false).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::UpdateError(UpdateError::DocumentAlreadyExists),
  ));

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  // version is still 4, no events have been committed
  assert_eq!(snapshot.sequence(), Generation::from(4));

  Ok(())
}

#[tokio::test]
async fn test_create_identity_from_private_key() -> Result<()> {
  let account: Account = new_account().await?;
  let account2: Account = new_account().await?;

  let identity: IdentityId = IdentityId::from_u32(1);

  let private_key = KeyPair::new_ed25519()?.private().clone();

  let id_create = IdentityCreate::new()
    .key_type(KeyType::Ed25519)
    .method_secret(MethodSecret::Ed25519(private_key));

  account.create_identity(id_create.clone()).await?;
  account2.create_identity(id_create).await?;

  let ident = account.find_identity(identity).await.unwrap().unwrap();
  let ident2 = account.find_identity(identity).await.unwrap().unwrap();

  // The same private key should result in the same did
  assert_eq!(ident.did(), ident2.did());
  assert_eq!(ident.authentication()?, ident2.authentication()?);

  Ok(())
}

#[tokio::test]
async fn test_create_identity_from_invalid_private_key() -> Result<()> {
  let account: Account = new_account().await?;

  let private_bytes: Box<[u8]> = Box::new([0; 33]);
  let private_key: PrivateKey = PrivateKey::from(private_bytes);

  let id_create = IdentityCreate::new()
    .key_type(KeyType::Ed25519)
    .method_secret(MethodSecret::Ed25519(private_key));

  let err = account.create_identity(id_create).await.unwrap_err();

  assert!(matches!(err, Error::UpdateError(UpdateError::InvalidMethodSecret(_))));

  Ok(())
}

#[tokio::test]
async fn test_create_method() -> Result<()> {
  let account: Account = new_account().await?;
  let identity: IdentityId = IdentityId::from_u32(1);

  let command: Command = Command::CreateIdentity {
    network: None,
    method_secret: None,
    authentication: MethodType::Ed25519VerificationKey2018,
  };

  account.process(identity, command, false).await?;

  let command: Command = Command::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "key-1".to_owned(),
  };

  account.process(identity, command, false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  assert_eq!(snapshot.sequence(), Generation::from_u32(6));
  assert_eq!(snapshot.id(), identity);
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
  let account: Account = new_account().await?;
  let identity: IdentityId = IdentityId::from_u32(1);

  let command: Command = Command::CreateIdentity {
    network: None,
    method_secret: None,
    authentication: MethodType::Ed25519VerificationKey2018,
  };

  account.process(identity, command, false).await?;

  let command: Command = Command::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "_sign-123".to_owned(),
  };

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  // version is now 4
  assert_eq!(snapshot.sequence(), Generation::from_u32(4));

  let output: _ = account.process(identity, command, false).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::UpdateError(UpdateError::InvalidMethodFragment(_)),
  ));

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  // version is still 4, no new events have been committed
  assert_eq!(snapshot.sequence(), Generation::from_u32(4));

  Ok(())
}

#[tokio::test]
async fn test_create_method_duplicate_fragment() -> Result<()> {
  let account: Account = new_account().await?;
  let identity: IdentityId = IdentityId::from_u32(1);

  let command: Command = Command::CreateIdentity {
    network: None,
    method_secret: None,
    authentication: MethodType::Ed25519VerificationKey2018,
  };

  account.process(identity, command, false).await?;

  let command: Command = Command::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "key-1".to_owned(),
  };

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;
  assert_eq!(snapshot.sequence(), Generation::from_u32(4));

  account.process(identity, command.clone(), false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;
  assert_eq!(snapshot.sequence(), Generation::from_u32(6));

  let output: _ = account.process(identity, command, false).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::UpdateError(UpdateError::DuplicateKeyFragment(_)),
  ));

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;
  assert_eq!(snapshot.sequence(), Generation::from_u32(6));

  Ok(())
}

#[tokio::test]
async fn test_create_method_from_private_key() -> Result<()> {
  let account: Account = new_account().await?;
  let identity: IdentityId = IdentityId::from_u32(1);

  let command: Command = Command::CreateIdentity {
    network: None,
    method_secret: None,
    authentication: MethodType::Ed25519VerificationKey2018,
  };

  account.process(identity, command, false).await?;

  let keypair = KeyPair::new_ed25519()?;

  let command: Command = Command::CreateMethod {
    scope: MethodScope::default(),
    method_secret: Some(MethodSecret::Ed25519(keypair.private().clone())),
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "key-1".to_owned(),
  };

  account.process(identity, command, false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  let method: &TinyMethod = snapshot.identity().methods().fetch("key-1")?;

  let public_key = account.store().key_get(identity, method.location()).await?;

  assert_eq!(public_key.as_ref(), keypair.public().as_ref());

  Ok(())
}

#[tokio::test]
async fn test_create_method_from_invalid_private_key() -> Result<()> {
  let account: Account = new_account().await?;
  let identity: IdentityId = IdentityId::from_u32(1);

  let command: Command = Command::CreateIdentity {
    network: None,
    method_secret: None,
    authentication: MethodType::Ed25519VerificationKey2018,
  };

  account.process(identity, command, false).await?;

  let private_bytes: Box<[u8]> = Box::new([0; 33]);
  let private_key = PrivateKey::from(private_bytes);

  let command: Command = Command::CreateMethod {
    scope: MethodScope::default(),
    method_secret: Some(MethodSecret::Ed25519(private_key)),
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "key-1".to_owned(),
  };

  let err = account.process(identity, command, false).await.unwrap_err();

  assert!(matches!(err, Error::UpdateError(UpdateError::InvalidMethodSecret(_))));

  Ok(())
}

#[tokio::test]
async fn test_create_method_with_type_secret_mismatch() -> Result<()> {
  let account: Account = new_account().await?;
  let identity: IdentityId = IdentityId::from_u32(1);

  let command: Command = Command::CreateIdentity {
    network: None,
    method_secret: None,
    authentication: MethodType::Ed25519VerificationKey2018,
  };

  account.process(identity, command, false).await?;

  let private_bytes: Box<[u8]> = Box::new([0; 32]);
  let private_key = PrivateKey::from(private_bytes);

  let command: Command = Command::CreateMethod {
    scope: MethodScope::default(),
    method_secret: Some(MethodSecret::Ed25519(private_key)),
    type_: MethodType::MerkleKeyCollection2021,
    fragment: "key-1".to_owned(),
  };

  let err = account.process(identity, command, false).await.unwrap_err();

  assert!(matches!(err, Error::UpdateError(UpdateError::InvalidMethodSecret(_))));

  let key_collection = KeyCollection::new_ed25519(4).unwrap();

  let command: Command = Command::CreateMethod {
    scope: MethodScope::default(),
    method_secret: Some(MethodSecret::MerkleKeyCollection(key_collection)),
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "key-2".to_owned(),
  };

  let err = account.process(identity, command, false).await.unwrap_err();

  assert!(matches!(err, Error::UpdateError(UpdateError::InvalidMethodSecret(_))));

  Ok(())
}

#[tokio::test]
async fn test_delete_method() -> Result<()> {
  let account: Account = new_account().await?;
  let identity: IdentityId = IdentityId::from_u32(1);

  let command: Command = Command::CreateIdentity {
    network: None,
    method_secret: None,
    authentication: MethodType::Ed25519VerificationKey2018,
  };

  account.process(identity, command, false).await?;

  let command: Command = Command::CreateMethod {
    scope: MethodScope::default(),
    method_secret: None,
    type_: MethodType::Ed25519VerificationKey2018,
    fragment: "key-1".to_owned(),
  };

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;
  assert_eq!(snapshot.sequence(), Generation::from_u32(4));

  account.process(identity, command, false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  assert_eq!(snapshot.sequence(), Generation::from_u32(6));
  assert_eq!(snapshot.identity().methods().len(), 2);
  assert!(snapshot.identity().methods().contains("key-1"));
  assert!(snapshot.identity().methods().get("key-1").is_some());
  assert!(snapshot.identity().methods().fetch("key-1").is_ok());

  let command: Command = Command::DeleteMethod {
    fragment: "key-1".to_owned(),
  };

  account.process(identity, command, false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  assert_eq!(snapshot.sequence(), Generation::from_u32(8));
  assert_eq!(snapshot.identity().methods().len(), 1);
  assert!(!snapshot.identity().methods().contains("key-1"));
  assert!(snapshot.identity().methods().get("key-1").is_none());
  assert!(snapshot.identity().methods().fetch("key-1").is_err());

  Ok(())
}
