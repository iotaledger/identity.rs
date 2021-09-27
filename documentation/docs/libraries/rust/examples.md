---
title: Examples
sidebar_label: Rust Examples
description: Rust Examples. 
image: /img/Identity_icon.png
keywords:
- Rust
- Examples
---

## Example: Creating an Identity

*Cargo.toml*
```rust
[package]
name = "iota_identity_example"
version = "1.0.0"
edition = "2018"

[dependencies]
identity = { git = "https://github.com/iotaledger/identity.rs", branch = "main"}
pretty_env_logger = { version = "0.4" }
tokio = { version = "1.5", features = ["full"] }
```
*main.*<span></span>*rs*
```rust
use std::path::PathBuf;

use identity::account::Account;
use identity::account::AccountStorage;
use identity::account::IdentityCreate;
use identity::account::IdentitySnapshot;
use identity::account::Result;
use identity::iota::IotaDID;
use identity::iota::IotaDocument;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // The Stronghold settings for the storage
  let snapshot: PathBuf = "./example-strong.hodl".into();
  let password: String = "my-password".into();

  // Create a new Account with Stronghold as the storage adapter
  let account: Account = Account::builder()
    .storage(AccountStorage::Stronghold(snapshot, Some(password)))
    .build()
    .await?;

  // Create a new Identity with default settings
  let snapshot1: IdentitySnapshot = account.create_identity(IdentityCreate::default()).await?;

  // Retrieve the DID from the newly created Identity state.
  let document1: &IotaDID = snapshot1.identity().try_did()?;

  println!("[Example] Local Snapshot = {:#?}", snapshot1);
  println!("[Example] Local Document = {:#?}", snapshot1.identity().to_document()?);
  println!("[Example] Local Document List = {:#?}", account.list_identities().await);

  // Fetch the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  let resolved: IotaDocument = account.resolve_identity(document1).await?;

  println!("[Example] Tangle Document = {:#?}", resolved);

  Ok(())
}
```
*Example output*
```rust
DID Document Transaction > https://explorer.iota.org/mainnet/message/de795095cc7970c2aa4efabfe9885bd07be6664219464697b4b7506d9a87fbe3
```
The output link points towards the DID Document transaction, viewable through the IOTA Tangle Explorer, see [here](https://explorer.iota.org/mainnet/message/de795095cc7970c2aa4efabfe9885bd07be6664219464697b4b7506d9a87fbe3). You can see the full DID Document as transaction payload.

## Roadmap and Milestones

For detailed development progress, see the IOTA Identity development [kanban board](https://github.com/iotaledger/identity.rs/projects/3).

IOTA Identity is in heavy development, and will naturally change as it matures and people use it. The chart below isn't meant to be exhaustive, but rather helps to give an idea for some of the areas of development and their relative completion:
