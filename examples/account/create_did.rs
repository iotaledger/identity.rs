// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_create

use std::path::PathBuf;

use identity::account::Account;
use identity::account::AccountBuilder;
use identity::account::AccountStorage;
use identity::account::IdentityCreate;
use identity::account::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let stronghold_path: PathBuf = "./example-strong.hodl".into();
  let password: String = "my-password".into();

  let mut builder: AccountBuilder = Account::builder()
    .storage(AccountStorage::Stronghold(stronghold_path, Some(password)))
    .dropsave(true);

  let account = builder.create_identity(IdentityCreate::default()).await?;

  // To show how existing identities are loaded into accounts
  let mut account = builder.load_identity(account.did().to_owned()).await?;

  // Requires &mut self to disallow concurrent updates on the same identity
  account
    .update_identity()
    .create_method()
    .fragment("new-method")
    .apply()
    .await?;

  // Consumes the account
  account.delete_identity().await?;

  Ok(())
}
