// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::did::IotaDID;

use crate::account::Account;
use crate::account::AccountBuilder;
use crate::error::Result;

#[tokio::test]
async fn test_account_builder() -> Result<()> {
  let pre_existing_did: IotaDID = "did:iota:GbVhKptDshJRVfUCRR7PURpLZyCYonSzM3Sa8QbfDemo".parse().unwrap();
  let pre_existing_did2: IotaDID = "did:iota:GbVhKptDshJRVfUCRR7PURpLZyCYonSzM3Sa8QbfMemo".parse().unwrap();

  let mut builder: AccountBuilder = AccountBuilder::default();

  // Create accounts from that builder configuration
  let account: Account = builder.load_identity(pre_existing_did).await?;

  builder = builder.autopublish(false);

  // Both accounts share Storage and ClientMap.
  // Only `account2` has autopublish = false.
  let account2: Account = builder.load_identity(pre_existing_did2).await?;

  assert!(account.autopublish());
  assert!(!account2.autopublish());

  Ok(())
}
