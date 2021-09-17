// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::pin::Pin;

use crate::account::Account;
use crate::identity::{IdentityCreate, IdentitySnapshot, IdentityUpdater};
use crate::{Error as AccountError, Result};
use futures::Future;
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
        Network::Devnet
      } else {
        Network::Mainnet
      };

      let snapshot: IdentitySnapshot = account
        .create_identity(IdentityCreate::new().network(network.name().as_ref()))
        .await?;

      let did: &IotaDID = snapshot.identity().try_did()?;

      let did_updater: IdentityUpdater<'_, '_, _> = account.update_identity(did);

      did_updater
        .create_service()
        .fragment("my-service")
        .type_("LinkedDomains")
        .endpoint(Url::parse("https://example.org").unwrap())
        .apply()
        .await?;

      did_updater
        .create_service()
        .fragment("my-other-service")
        .type_("LinkedDomains")
        .endpoint(Url::parse("https://example.org").unwrap())
        .apply()
        .await?;

      account.publish_updates(did).await?;

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

      did_updater.delete_service().fragment("my-service").apply().await?;

      did_updater
        .delete_service()
        .fragment("my-other-service")
        .apply()
        .await?;

      did_updater.create_method().fragment("new-method").apply().await?;

      account.publish_updates(did).await?;

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
// Network problems, i.e. a ClientError trigger a re-run.
// Other errors end the test immediately.
async fn network_resilient_test(
  test_runs: u32,
  f: impl Fn(u32) -> Pin<Box<dyn Future<Output = Result<()>>>>,
) -> Result<()> {
  for test_run in 0..test_runs {
    let test_attempt = f(test_run).await;

    match test_attempt {
      error @ Err(AccountError::IotaError(IotaError::ClientError(_))) => {
        eprintln!("test run {} errored with {:?}", test_run, error);

        if test_run == test_runs - 1 {
          return error;
        }
      }
      other => return other,
    }
  }

  Ok(())
}
