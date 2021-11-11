// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::pin::Pin;
use std::sync::Arc;

use crate::account::AccountConfig;
use crate::account::AccountSetup;
use crate::storage::MemStore;
use futures::Future;

use identity_core::common::Url;
use identity_did::verification::MethodScope;
use identity_iota::chain::DocumentHistory;
use identity_iota::tangle::Client;
use identity_iota::tangle::Network;
use identity_iota::Error as IotaError;

use crate::account::Account;
use crate::identity::IdentitySetup;
use crate::Error as AccountError;
use crate::Result;

#[tokio::test]
async fn test_lazy_updates() -> Result<()> {
  network_resilient_test(2, |test_run| {
    Box::pin(async move {
      // ===========================================================================
      // Create, update and publish an identity
      // ===========================================================================

      let network = if test_run % 2 == 0 {
        Network::Devnet
      } else {
        Network::Mainnet
      };

      let config = AccountConfig::default().autopublish(false);
      let account_config = AccountSetup::new(Arc::new(MemStore::new())).config(config);

      let mut account = Account::create_identity(account_config, IdentitySetup::new().network(network.name())?).await?;

      account
        .update_identity()
        .create_service()
        .fragment("my-service")
        .type_("LinkedDomains")
        .endpoint(Url::parse("https://example.org").unwrap())
        .apply()
        .await?;

      account
        .update_identity()
        .create_service()
        .fragment("my-other-service")
        .type_("LinkedDomains")
        .endpoint(Url::parse("https://example.org").unwrap())
        .apply()
        .await?;

      account.publish_updates().await?;

      // ===========================================================================
      // First round of assertions
      // ===========================================================================

      let doc = account.resolve_identity().await?;

      assert_eq!(doc.methods().count(), 1);
      assert_eq!(doc.service().len(), 2);

      for service in ["my-service", "my-other-service"] {
        assert!(doc.service().query(service).is_some());
      }

      // ===========================================================================
      // More updates to the identity
      // ===========================================================================

      account
        .update_identity()
        .delete_service()
        .fragment("my-service")
        .apply()
        .await?;

      account
        .update_identity()
        .delete_service()
        .fragment("my-other-service")
        .apply()
        .await?;

      account
        .update_identity()
        .create_method()
        .fragment("new-method")
        .apply()
        .await?;

      account.publish_updates().await?;

      // ===========================================================================
      // Second round of assertions
      // ===========================================================================

      let doc = account.resolve_identity().await?;

      assert_eq!(doc.service().len(), 0);
      assert_eq!(doc.methods().count(), 2);

      for method in ["sign-0", "new-method"] {
        assert!(doc.resolve_method(method).is_some());
      }

      // ===========================================================================
      // History assertions 1
      // ===========================================================================

      let client: Client = Client::from_network(network).await?;

      let history: DocumentHistory = client.resolve_history(account.did()).await?;

      assert_eq!(history.integration_chain_data.len(), 1);
      assert_eq!(history.diff_chain_data.len(), 1);

      // ===========================================================================
      // More updates to the identity
      // ===========================================================================

      account
        .update_identity()
        .create_method()
        .fragment("signing-key")
        // Forces an integration update.
        .scope(MethodScope::CapabilityInvocation)
        .apply()
        .await?;

      account.publish_updates().await?;

      // ===========================================================================
      // History assertions 2
      // ===========================================================================

      let history: DocumentHistory = client.resolve_history(account.did()).await?;

      assert_eq!(history.integration_chain_data.len(), 2);
      assert_eq!(history.diff_chain_data.len(), 0);

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
