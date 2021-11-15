// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use std::path::PathBuf;
use std::sync::Arc;

use identity_iota::did::IotaDID;

use crate::account::Account;
use crate::account::AccountBuilder;
use crate::account::AccountConfig;
use crate::account::AccountSetup;
use crate::storage::Stronghold;
use crate::error::Result;
use crate::identity::IdentitySetup;

#[tokio::test]
async fn test_account_high_level() -> Result<()> {
  let mut builder: AccountBuilder = AccountBuilder::default().testmode(true);

  let account1: Account = builder.create_identity(IdentitySetup::default()).await?;

  builder = builder.autopublish(false);

  let account2: Account = builder.create_identity(IdentitySetup::default()).await?;

  assert!(account1.autopublish());
  assert!(!account2.autopublish());

  let did1 = account1.did().to_owned();
  let did2 = account2.did().to_owned();
  account2.delete_identity().await?;

  assert!(matches!(
    builder.load_identity(did2).await.unwrap_err(),
    crate::Error::IdentityNotFound
  ));

  // Relase the lease on did1.
  std::mem::drop(account1);

  assert!(builder.load_identity(did1).await.is_ok());

  Ok(())
}

#[tokio::test]
async fn test_account_did_lease() -> Result<()> {
  let mut builder: AccountBuilder = AccountBuilder::default().testmode(true);

  let did: IotaDID = {
    let account: Account = builder.create_identity(IdentitySetup::default()).await?;
    account.did().to_owned()
  }; // <-- Lease released here.

  // Lease is not in-use
  let _account = builder.load_identity(did.clone()).await.unwrap();

  // Lease is in-use
  assert!(matches!(
    builder.load_identity(did).await.unwrap_err(),
    crate::Error::IdentityInUse
  ));

  Ok(())
}

#[tokio::test]
async fn test_account_dropsave_deadlock() -> Result<()> {
  let stronghold_path: PathBuf = "./example-strong.hodl".into();

  let storage = Stronghold::new(&stronghold_path, Some("my-password")).await?;

  let setup = AccountSetup::new(Arc::new(storage)).config(AccountConfig::new().testmode(true));

  let account: Account = Account::create_identity(setup.clone(), IdentitySetup::default()).await?;
  let mut account2: Account = Account::create_identity(setup.clone(), IdentitySetup::default()).await?;

  let task = tokio::task::spawn(async move {
    for i in 0..10 {
      add_method(&mut account2, i).await;
    }
  });

  let did = account.did().to_owned();
  std::mem::drop(account);

  for i in 0..10 {
    let mut account = Account::load_identity(setup.clone(), did.clone()).await?;
    // Force the account to run the dropsave
    // add_method(&mut account, i).await;
  }

  task.await.unwrap();

  Ok(())
}

async fn add_method(account: &mut Account, iteration: usize) {
  account
    .update_identity()
    .create_method()
    .fragment(format!("key-{}", iteration))
    .apply()
    .await
    .unwrap();
}