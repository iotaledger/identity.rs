// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;
use identity_did::verification::MethodScope;
use identity_iota::did::IotaDID;
use identity_iota::tangle::MessageId;

use crate::account::Account;
use crate::account::AccountBuilder;
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
async fn test_account_chain_state() -> Result<()> {
  let mut builder: AccountBuilder = AccountBuilder::default().testmode(true);

  let mut account: Account = builder.create_identity(IdentitySetup::default()).await?;

  let last_int_id = *account.chain_state().last_integration_message_id();

  assert_ne!(last_int_id, MessageId::null());

  // Assert that the last_diff_message_id is still null.
  assert_eq!(account.chain_state().last_diff_message_id(), &MessageId::null());

  // A diff update.
  account
    .update_identity()
    .create_service()
    .fragment("my-service-1")
    .type_("MyCustomService")
    .endpoint(Url::parse("https://example.com")?)
    .apply()
    .await?;

  // A diff update does not overwrite the int message id.
  assert_eq!(&last_int_id, account.chain_state().last_integration_message_id());

  // Assert that the last_diff_message_id was set.
  assert_ne!(account.chain_state().last_diff_message_id(), &MessageId::null());

  account
    .update_identity()
    .create_method()
    .fragment("my-new-key")
    .scope(MethodScope::capability_invocation())
    .apply()
    .await?;

  // Int message id was overwritten.
  assert_ne!(&last_int_id, account.chain_state().last_integration_message_id());

  Ok(())
}
