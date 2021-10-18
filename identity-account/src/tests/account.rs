// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::IotaDID;

use crate::account::Account;
use crate::account::AccountBuilder;
use crate::error::Result;
use crate::identity::IdentityCreate;

#[tokio::test]
async fn test_account_builder_config_() -> Result<()> {
  let mut builder: AccountBuilder = AccountBuilder::default().testmode(true);

  let account: Account = builder.create_identity(IdentityCreate::default()).await?;

  builder = builder.autopublish(false);

  let account2: Account = builder.create_identity(IdentityCreate::default()).await?;

  assert!(account.autopublish());
  assert!(!account2.autopublish());

  Ok(())
}

#[tokio::test]
async fn test_account_load_id() -> Result<()> {
  let non_existent_did: IotaDID = "did:iota:GbVhKptDshJRVfUCRR7PURpLZyCYonSzM3Sa8QbfDemo".parse().unwrap();

  let builder: AccountBuilder = AccountBuilder::default();

  matches!(
    builder.load_identity(non_existent_did).await.unwrap_err(),
    crate::Error::IdentityNotFound
  );

  Ok(())
}
