// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::IotaDID;

use crate::account::Account;
use crate::account::AccountBuilder;
use crate::error::Result;

#[tokio::test]
async fn test_account_high_level() -> Result<()> {
  let mut builder: AccountBuilder = AccountBuilder::default().testmode(true);

  let account1: Account = builder.create_identity().build().await?;

  builder = builder.autopublish(false);

  let account2: Account = builder.create_identity().build().await?;

  assert!(account1.autopublish());
  assert!(!account2.autopublish());

  let did1 = account1.did().to_owned();
  let did2 = account2.did().to_owned();
  account2.delete_identity().await?;

  assert!(matches!(
    builder.load_identity(did2).await.unwrap_err(),
    crate::Error::IdentityNotFound
  ));

  assert!(builder.load_identity(did1).await.is_ok());

  Ok(())
}

#[tokio::test]
async fn test_account_did_lease() -> Result<()> {
  let mut builder: AccountBuilder = AccountBuilder::default().testmode(true);

  let did: IotaDID = {
    let account: Account = builder.create_identity().build().await?;
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
