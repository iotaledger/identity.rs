// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_lazy

use identity::account::Account;
use identity::account::IdentityCreate;
use identity::account::IdentitySnapshot;
use identity::account::Result;
use identity::core::Url;
use identity::iota::IotaDID;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // Create a new Account with auto publishing set to false.
  // This means updates are not pushed to the tangle automatically.
  // Rather, when we publish, multiple updates are batched together.
  let account: Account = Account::builder().autopublish(false).build().await?;

  // Create a new Identity with default settings.
  // The identity will only be written to the local storage - not published to the tangle.
  let snapshot: IdentitySnapshot = account.create_identity(IdentityCreate::default()).await?;

  // Retrieve the DID from the newly created Identity state.
  let did: &IotaDID = snapshot.identity().try_did()?;

  account
    .update_identity(did)
    .create_service()
    .fragment("example-service")
    .type_("LinkedDomains")
    .endpoint(Url::parse("https://example.org")?)
    .apply()
    .await?;

  // Publish the newly created DID document,
  // including the new service, to the tangle.
  account.publish_updates(did).await?;

  // Add another service.
  account
    .update_identity(did)
    .create_service()
    .fragment("another-service")
    .type_("LinkedDomains")
    .endpoint(Url::parse("https://example.org")?)
    .apply()
    .await?;

  // Delete the previously added service.
  account
    .update_identity(did)
    .delete_service()
    .fragment("example-service")
    .apply()
    .await?;

  // Publish the updates as one message to the tangle.
  account.publish_updates(did).await?;

  // Resolve the document to confirm its consistency.
  let doc = account.resolve_identity(did).await?;

  println!("[Example] Document: {:#?}", doc);

  Ok(())
}
