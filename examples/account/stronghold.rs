// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_stronghold

use std::path::PathBuf;

use identity::account::Account;
use identity::account::AccountStorage;
use identity::account::IdentityCreate;
use identity::account::IdentityState;
use identity::account::Result;
use identity::account::Stronghold;
use identity::iota::IotaDID;
use identity::iota::IotaDocument;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  let snapshot: PathBuf = "./example-strong.hodl".into();
  let password: Option<&str> = Some("my-password");

  // Create a new Account with Stronghold as the storage adapter
  // let account: Account = Account::builder()
  //   .storage(AccountStorage::Stronghold(snapshot, Some(password)))
  //   .build()
  //   .await?;
  let adapter: Stronghold = Stronghold::new(&snapshot, password).await?;

  // Create a new Identity with default settings
  let identity1: Account = Account::create_identity(Default::default(), adapter, IdentityCreate::default()).await?;

  println!("[Example] DID = {:#?}", identity1.did());

  Ok(())
}
