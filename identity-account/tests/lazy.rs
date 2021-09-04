// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account::account::Account;
use identity_account::events::Command;
use identity_account::identity::{IdentityCreate, IdentitySnapshot};
use identity_account::Result;
use identity_core::common::Url;
use identity_iota::chain::DocumentHistory;
use identity_iota::did::{IotaDID, IotaVerificationMethod};
use identity_iota::tangle::{Client, Network};

#[tokio::test]
async fn test_lazy_updates() -> Result<()> {
  // ===========================================================================
  // Create, update and publish an identity
  // ===========================================================================
  let account: Account = Account::builder().autopublish(false).build().await?;

  let snapshot: IdentitySnapshot = account.create_identity(IdentityCreate::new().network("test")).await?;

  let did: &IotaDID = snapshot.identity().try_did()?;

  let command: Command = Command::create_service()
    .fragment("my-service")
    .type_("url")
    .endpoint(Url::parse("https://example.org").unwrap())
    .finish()?;
  account.update_identity(did, command).await.unwrap();

  let command: Command = Command::create_service()
    .fragment("my-other-service")
    .type_("url")
    .endpoint(Url::parse("https://example.org").unwrap())
    .finish()?;
  account.update_identity(did, command).await.unwrap();

  account.publish_changes(did).await.unwrap();

  // ===========================================================================
  // First round of assertions
  // ===========================================================================

  let doc = account.resolve_identity(snapshot.identity().did().unwrap()).await?;

  let services = doc.service();

  assert_eq!(doc.methods().count(), 1);
  assert_eq!(services.len(), 2);

  for service in services.iter() {
    let service_fragment = service.as_ref().id().fragment().unwrap();
    assert!(["my-service", "my-other-service"]
      .iter()
      .any(|fragment| *fragment == service_fragment));
  }

  // ===========================================================================
  // More updates to the identity
  // ===========================================================================

  let command: Command = Command::delete_service().fragment("my-service").finish()?;
  account.update_identity(did, command).await.unwrap();

  let command: Command = Command::delete_service().fragment("my-other-service").finish()?;
  account.update_identity(did, command).await.unwrap();

  let command: Command = Command::create_method().fragment("new-method").finish()?;
  account.update_identity(did, command).await.unwrap();

  account.publish_changes(did).await.unwrap();

  // ===========================================================================
  // Second round of assertions
  // ===========================================================================

  let doc = account.resolve_identity(snapshot.identity().did().unwrap()).await?;
  let methods = doc.methods().collect::<Vec<&IotaVerificationMethod>>();

  assert_eq!(doc.service().len(), 0);
  assert_eq!(methods.len(), 2);

  for method in methods {
    let method_fragment = method.id().fragment().unwrap();
    assert!(["_sign-0", "new-method"]
      .iter()
      .any(|fragment| *fragment == method_fragment));
  }

  // ===========================================================================
  // History assertions
  // ===========================================================================

  let client: Client = Client::from_network(Network::Testnet).await?;

  let history: DocumentHistory = client.resolve_history(did).await?;

  assert_eq!(history.integration_chain_data.len(), 1);
  assert_eq!(history.diff_chain_data.len(), 1);

  Ok(())
}
