// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::pin::Pin;

use futures::Future;
use identity_account::account::Account;
use identity_account::events::Command;
use identity_account::identity::{IdentityCreate, IdentitySnapshot};
use identity_account::{Error as AccountError, Result};
use identity_core::common::Url;
use identity_iota::chain::DocumentHistory;
use identity_iota::did::{IotaDID, IotaVerificationMethod};
use identity_iota::tangle::{Client, Network};
use identity_iota::Error as IotaError;

#[tokio::test]
async fn test_lazy_updates() -> Result<()> {
  network_resilient_test(2, |test_run| {
    Box::pin(async move {
      // ===========================================================================
      // Create, update and publish an identity
      // ===========================================================================
      let account: Account = Account::builder().autopublish(false).build().await?;

      let network = if test_run % 2 == 0 {
        Network::Testnet
      } else {
        Network::Mainnet
      };

      let snapshot: IdentitySnapshot = account
        .create_identity(IdentityCreate::new().network(network.name().as_ref()))
        .await?;

      let did: &IotaDID = snapshot.identity().try_did()?;

      let command: Command = Command::create_service()
        .fragment("my-service")
        .type_("url")
        .endpoint(Url::parse("https://example.org").unwrap())
        .finish()?;
      account.update_identity(did, command).await?;

      let command: Command = Command::create_service()
        .fragment("my-other-service")
        .type_("url")
        .endpoint(Url::parse("https://example.org").unwrap())
        .finish()?;
      account.update_identity(did, command).await?;

      account.publish_changes(did).await?;

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
      account.update_identity(did, command).await?;

      let command: Command = Command::delete_service().fragment("my-other-service").finish()?;
      account.update_identity(did, command).await?;

      let command: Command = Command::create_method().fragment("new-method").finish()?;
      account.update_identity(did, command).await?;

      account.publish_changes(did).await?;

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

      let client: Client = Client::from_network(network).await?;

      let history: DocumentHistory = client.resolve_history(did).await?;

      assert_eq!(history.integration_chain_data.len(), 1);
      assert_eq!(history.diff_chain_data.len(), 1);

      Ok(())
    })
  })
  .await?;

  Ok(())
}

// Repeats the test in the closure `test_runs` number of times.
// Network problems, i.e. a ClientError triggers a re-run.
// Other errors end the test immediately.
async fn network_resilient_test(
  test_runs: u32,
  f: impl Fn(u32) -> Pin<Box<dyn Future<Output = Result<()>>>>,
) -> Result<()> {
  for test_run in 0..test_runs {
    let test_attempt = f(test_run).await;

    match test_attempt {
      ok @ Ok(_) => {
        return ok;
      }
      Err(AccountError::IotaError(IotaError::ClientError(client_err))) => {
        eprintln!("test run {} errored with {:?}", test_run, client_err);

        if test_run == test_runs - 1 {
          return Err(AccountError::IotaError(IotaError::ClientError(client_err)));
        }
      }
      error @ Err(_) => {
        return error;
      }
    }
  }

  Ok(())
}
