// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account::account::Account;
use identity_account::account::Config;
use identity_account::error::Error;
use identity_account::error::Result;
use identity_account::events::Command;
use identity_account::events::CommandError;
use identity_account::events::MethodSecret;
use identity_account::identity::IdentityCreate;
use identity_account::identity::IdentityId;
use identity_account::identity::IdentitySnapshot;
use identity_account::identity::TinyMethod;
use identity_account::storage::MemStore;
use identity_account::types::Generation;
use identity_core::common::UnixTimestamp;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::SecretKey;
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

  account
    .process(identity, Command::create_identity().finish().unwrap(), false)
    .await?;

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  assert_eq!(snapshot.sequence(), Generation::from_u32(3));
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
    let command: Command = Command::create_identity().authentication(type_).finish().unwrap();
    let output: Result<()> = account.process(identity, command, false).await;

    assert!(matches!(
      output.unwrap_err(),
      Error::CommandError(CommandError::InvalidMethodType(_))
    ));

    let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

    // version is still 0, no events have been committed
    assert_eq!(snapshot.sequence(), Generation::new());
  }

  Ok(())
}

#[tokio::test]
async fn test_create_identity_already_exists() -> Result<()> {
  let account: Account = new_account().await?;
  let identity: IdentityId = IdentityId::from_u32(1);
  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  // initial snapshot version = 0
  assert_eq!(snapshot.sequence(), Generation::new());

  let command: Command = Command::create_identity()
    .authentication(MethodType::Ed25519VerificationKey2018)
    .finish()
    .unwrap();

  account.process(identity, command.clone(), false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  // version is now 3
  assert_eq!(snapshot.sequence(), Generation::from(3));

  let output: Result<()> = account.process(identity, command, false).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::CommandError(CommandError::DocumentAlreadyExists),
  ));

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  // version is still 3, no events have been committed
  assert_eq!(snapshot.sequence(), Generation::from(3));

  Ok(())
}

#[tokio::test]
async fn test_create_identity_from_secret_key() -> Result<()> {
  let account: Account = new_account().await?;
  let account2: Account = new_account().await?;

  let identity: IdentityId = IdentityId::from_u32(1);

  let secret_key = KeyPair::new_ed25519()?.secret().clone();

  let id_create = IdentityCreate::new()
    .key_type(KeyType::Ed25519)
    .method_secret(MethodSecret::Ed25519(secret_key));

  account.create_identity(id_create.clone()).await?;
  account2.create_identity(id_create).await?;

  let ident = account.find_identity(identity).await.unwrap().unwrap();
  let ident2 = account.find_identity(identity).await.unwrap().unwrap();

  // The same secret key should result in the same did
  assert_eq!(ident.identity().did(), ident2.identity().did());

  Ok(())
}

#[tokio::test]
async fn test_create_method() -> Result<()> {
  let account: Account = new_account().await?;
  let identity: IdentityId = IdentityId::from_u32(1);

  let command: Command = Command::create_identity()
    .authentication(MethodType::Ed25519VerificationKey2018)
    .finish()
    .unwrap();

  account.process(identity, command, false).await?;

  let command: Command = Command::create_method()
    .type_(MethodType::Ed25519VerificationKey2018)
    .fragment("key-1")
    .finish()
    .unwrap();

  account.process(identity, command, false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  assert_eq!(snapshot.sequence(), Generation::from_u32(5));
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

  let command: Command = Command::create_identity()
    .authentication(MethodType::Ed25519VerificationKey2018)
    .finish()
    .unwrap();

  account.process(identity, command, false).await?;

  let command: Command = Command::create_method()
    .type_(MethodType::Ed25519VerificationKey2018)
    .fragment("_sign-123")
    .finish()
    .unwrap();

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  // version is now 3
  assert_eq!(snapshot.sequence(), Generation::from_u32(3));

  let output: _ = account.process(identity, command, false).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::CommandError(CommandError::InvalidMethodFragment(_)),
  ));

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  // version is still 3, no new events have been committed
  assert_eq!(snapshot.sequence(), Generation::from_u32(3));

  Ok(())
}

#[tokio::test]
async fn test_create_method_duplicate_fragment() -> Result<()> {
  let account: Account = new_account().await?;
  let identity: IdentityId = IdentityId::from_u32(1);

  let command: Command = Command::create_identity()
    .authentication(MethodType::Ed25519VerificationKey2018)
    .finish()
    .unwrap();

  account.process(identity, command, false).await?;

  let command: Command = Command::create_method()
    .type_(MethodType::Ed25519VerificationKey2018)
    .fragment("key-1")
    .finish()
    .unwrap();

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;
  assert_eq!(snapshot.sequence(), Generation::from_u32(3));

  account.process(identity, command.clone(), false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;
  assert_eq!(snapshot.sequence(), Generation::from_u32(5));

  let output: _ = account.process(identity, command, false).await;

  assert!(matches!(
    output.unwrap_err(),
    Error::CommandError(CommandError::DuplicateKeyFragment(_)),
  ));

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;
  assert_eq!(snapshot.sequence(), Generation::from_u32(5));

  Ok(())
}

#[tokio::test]
async fn test_create_method_from_secret_key() -> Result<()> {
  let account: Account = new_account().await?;
  let identity: IdentityId = IdentityId::from_u32(1);

  let command: Command = Command::create_identity()
    .authentication(MethodType::Ed25519VerificationKey2018)
    .finish()
    .unwrap();

  account.process(identity, command, false).await?;

  let keypair = KeyPair::new_ed25519()?;

  let command: Command = Command::create_method()
    .type_(MethodType::Ed25519VerificationKey2018)
    .fragment("key-1")
    .method_secret(MethodSecret::Ed25519(keypair.secret().clone()))
    .finish()
    .unwrap();

  account.process(identity, command, false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  let method: &TinyMethod = snapshot.identity().methods().fetch("key-1")?;

  let public_key = account.store().key_get(identity, method.location()).await?;

  assert_eq!(public_key.as_ref(), keypair.public().as_ref());

  Ok(())
}
#[tokio::test]
async fn test_create_method_from_invalid_secret_key() -> Result<()> {
  let account: Account = new_account().await?;
  let identity: IdentityId = IdentityId::from_u32(1);

  let command: Command = Command::create_identity()
    .authentication(MethodType::Ed25519VerificationKey2018)
    .finish()
    .unwrap();

  account.process(identity, command, false).await?;

  let secret_bytes: Box<[u8]> = Box::new([0; 33]);
  let secret_key = SecretKey::from(secret_bytes);

  println!("creating method");
  let command: Command = Command::create_method()
    .type_(MethodType::Ed25519VerificationKey2018)
    .fragment("key-1")
    .method_secret(MethodSecret::Ed25519(secret_key))
    .finish()
    .unwrap();

  let err = account.process(identity, command, false).await.unwrap_err();

  println!("{:?}", err);

  assert!(matches!(
    err,
    Error::CommandError(CommandError::InvalidMethodSecret(err_string))
    if err_string == "an ed25519 secret key requires 32 bytes, found 33"
  ));

  Ok(())
}

#[tokio::test]
async fn test_delete_method() -> Result<()> {
  let account: Account = new_account().await?;
  let identity: IdentityId = IdentityId::from_u32(1);

  let command: Command = Command::create_identity()
    .authentication(MethodType::Ed25519VerificationKey2018)
    .finish()
    .unwrap();

  account.process(identity, command, false).await?;

  let command: Command = Command::create_method()
    .type_(MethodType::Ed25519VerificationKey2018)
    .fragment("key-1")
    .finish()
    .unwrap();

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;
  assert_eq!(snapshot.sequence(), Generation::from_u32(3));

  account.process(identity, command, false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  assert_eq!(snapshot.sequence(), Generation::from_u32(5));
  assert_eq!(snapshot.identity().methods().len(), 2);
  assert!(snapshot.identity().methods().contains("key-1"));
  assert!(snapshot.identity().methods().get("key-1").is_some());
  assert!(snapshot.identity().methods().fetch("key-1").is_ok());

  let command: Command = Command::delete_method().fragment("key-1").finish().unwrap();

  account.process(identity, command, false).await?;

  let snapshot: IdentitySnapshot = account.load_snapshot(identity).await?;

  assert_eq!(snapshot.sequence(), Generation::from_u32(7));
  assert_eq!(snapshot.identity().methods().len(), 1);
  assert!(!snapshot.identity().methods().contains("key-1"));
  assert!(snapshot.identity().methods().get("key-1").is_none());
  assert!(snapshot.identity().methods().fetch("key-1").is_err());

  Ok(())
}
