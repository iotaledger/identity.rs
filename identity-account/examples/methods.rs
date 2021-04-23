// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example methods

use identity_account::account::Account;
use identity_account::error::Result;
use identity_account::storage::MemStore;
use identity_account::types::ChainId;
use identity_account::types::IdentityConfig;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_iota::chain::DocumentChain;
use identity_iota::client::Client;
use identity_iota::client::Network;
use identity_iota::did::Document;
use identity_iota::did::DID;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  let storage: MemStore = MemStore::import_or_default("example-methods.json");
  let account: Account<_> = Account::new(storage).await?;

  // Create a new Identity chain. The chain will be published to the Tangle
  let chain: ChainId = account.create(IdentityConfig::new()).await?;
  let document: DID = account.get(chain).await?.id().clone();

  let mt: MethodType = MethodType::Ed25519VerificationKey2018;
  let ms: MethodScope = MethodScope::Authentication;

  // Add a new Verification Method to the Identity chain. The change is
  // published to the Tangle as a signed DocumentDiff.
  account.create_method(chain, mt, ms, "key-1").await?;

  // Add another Verification Method to the chain
  account.create_method(chain, mt, ms, "key-2").await?;

  println!("[Tangle] Document = {:#?}", account.resolve(&document).await?);

  // account.attach_method(chain, "key-2", MethodScope::AssertionMethod).await?;
  // account.detach_method(chain, "key-2", MethodScope::Authentication).await?;

  // account.create_method(chain, mt, ms, "key-3").await?;

  // let document: Document = account.get(chain).await?;

  // println!("[Account] Document = {:#?}", document);

  // // Fetch the DID Document from the Tangle
  // let resolved: DocumentChain = account.resolve(document).await?;

  // println!("[Tangle] Document = {:#?}", resolved.current());

  // Export the current state of the account
  account.store().export("example-methods.json")?;

  Ok(())
}
