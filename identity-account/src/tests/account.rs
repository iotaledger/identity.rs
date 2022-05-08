// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::pin::Pin;
use std::sync::Arc;

use futures::Future;

use identity_account_storage::identity::ChainState;
use identity_account_storage::storage::MemStore;
use identity_account_storage::storage::Stronghold;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::crypto::ProofOptions;
use identity_did::utils::Queryable;
use identity_did::verification::MethodScope;
use identity_iota::chain::DocumentChain;
use identity_iota::tangle::Client;
use identity_iota::tangle::ClientBuilder;
use identity_iota_core::did::IotaDID;
use identity_iota_core::diff::DiffMessage;
use identity_iota_core::document::IotaDocument;
use identity_iota_core::tangle::MessageId;
use identity_iota_core::tangle::MessageIdExt;
use identity_iota_core::tangle::Network;

use crate::account::Account;
use crate::account::AccountBuilder;
use crate::account::AccountConfig;
use crate::account::AccountSetup;
use crate::account::AutoSave;
use crate::account::PublishOptions;
use crate::types::IdentitySetup;
use crate::types::MethodContent;
use crate::Error;
use crate::Result;

use super::util::*;

#[tokio::test]
async fn test_account_builder() -> Result<()> {
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

  assert!(builder.load_identity(did1).await.is_ok());

  Ok(())
}

#[tokio::test]
async fn test_account_chain_state() {
  let mut builder: AccountBuilder = AccountBuilder::default().testmode(true);

  let mut account: Account = builder.create_identity(IdentitySetup::default()).await.unwrap();

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
    .endpoint(Url::parse("https://example.com").unwrap())
    .apply()
    .await
    .unwrap();

  // A diff update does not overwrite the int message id.
  assert_eq!(&last_int_id, account.chain_state().last_integration_message_id());

  // Assert that the last_diff_message_id was set.
  assert_ne!(account.chain_state().last_diff_message_id(), &MessageId::null());

  account
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateEd25519)
    .fragment("my-new-key")
    .scope(MethodScope::capability_invocation())
    .apply()
    .await
    .unwrap();

  // Int message id was overwritten.
  assert_ne!(&last_int_id, account.chain_state().last_integration_message_id());
}

#[tokio::test]
async fn test_account_autopublish() {
  // ===========================================================================
  // Create, update and "publish" an identity
  // ===========================================================================

  let config = AccountConfig::default().autopublish(false).testmode(true);
  let client = ClientBuilder::new().node_sync_disabled().build().await.unwrap();
  let account_setup = AccountSetup::new(Arc::new(MemStore::new()), Arc::new(client), config);

  let mut account = Account::create_identity(account_setup, IdentitySetup::new())
    .await
    .unwrap();

  account
    .update_identity()
    .create_service()
    .fragment("my-service")
    .type_("LinkedDomains")
    .endpoint(Url::parse("https://example.org").unwrap())
    .apply()
    .await
    .unwrap();

  account
    .update_identity()
    .create_service()
    .fragment("my-other-service")
    .type_("LinkedDomains")
    .endpoint(Url::parse("https://example.org").unwrap())
    .apply()
    .await
    .unwrap();

  assert!(account.chain_state().last_integration_message_id().is_null());
  assert!(account.chain_state().last_diff_message_id().is_null());

  account.publish().await.unwrap();

  let last_int_message_id = *account.chain_state().last_integration_message_id();

  assert!(!last_int_message_id.is_null());
  assert!(account.chain_state().last_diff_message_id().is_null());

  // ===========================================================================
  // Service assertions
  // ===========================================================================

  let doc = account.document();

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
    .await
    .unwrap();

  account
    .update_identity()
    .delete_service()
    .fragment("my-other-service")
    .apply()
    .await
    .unwrap();

  account
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateEd25519)
    .fragment("new-method")
    .apply()
    .await
    .unwrap();

  account.publish().await.unwrap();

  // No integration message was published
  assert_eq!(
    &last_int_message_id,
    account.chain_state().last_integration_message_id()
  );

  let last_diff_message_id = *account.chain_state().last_diff_message_id();
  assert!(!last_diff_message_id.is_null());

  // ===========================================================================
  // Second round of assertions
  // ===========================================================================

  let doc = account.document();

  assert_eq!(doc.service().len(), 0);
  assert_eq!(doc.methods().count(), 2);

  for method in ["sign-0", "new-method"] {
    assert!(doc.resolve_method(method, None).is_some());
  }

  // ===========================================================================
  // More updates to the identity
  // ===========================================================================

  account
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateEd25519)
    .fragment("signing-key")
    // Forces an integration update by adding a method able to update the document.
    .scope(MethodScope::capability_invocation())
    .apply()
    .await
    .unwrap();

  account.publish().await.unwrap();

  // Another int update was published.
  assert_ne!(
    &last_int_message_id,
    account.chain_state().last_integration_message_id()
  );

  // Diff message id was reset.
  assert!(account.chain_state().last_diff_message_id().is_null());
}

#[tokio::test]
async fn test_account_publish_options_sign_with() {
  let config = AccountConfig::default().autopublish(false).testmode(true);
  let client = ClientBuilder::new().node_sync_disabled().build().await.unwrap();
  let account_config = AccountSetup::new(Arc::new(MemStore::new()), Arc::new(client), config);

  let auth_method = "auth-method";
  let signing_method = "singing-method-2";
  let invalid_signing_method = "invalid-signing-method";

  let mut account = Account::create_identity(account_config, IdentitySetup::new())
    .await
    .unwrap();

  // Add an Authentication method unable to sign DID Document updates.
  account
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateEd25519)
    .fragment(auth_method)
    .scope(MethodScope::authentication())
    .apply()
    .await
    .unwrap();

  // Add a valid CapabilityInvocation method able to sign DID Document updates.
  account
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateEd25519)
    .fragment(signing_method)
    .scope(MethodScope::capability_invocation())
    .apply()
    .await
    .unwrap();

  // Add a CapabilityInvocation method unable to sign DID Document updates.
  account
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateX25519)
    .fragment(invalid_signing_method)
    .scope(MethodScope::capability_invocation())
    .apply()
    .await
    .unwrap();

  // INVALID: try sign with a non-existent method.
  assert!(matches!(
    account
      .publish_with_options(PublishOptions::default().sign_with("non-existent-method"))
      .await
      .unwrap_err(),
    Error::IotaCoreError(identity_iota_core::Error::InvalidDoc(
      identity_did::Error::MethodNotFound
    ))
  ));

  // INVALID: try sign with with an Authentication method.
  assert!(matches!(
    account
      .publish_with_options(PublishOptions::default().sign_with(auth_method))
      .await
      .unwrap_err(),
    Error::IotaCoreError(identity_iota_core::Error::InvalidDoc(
      identity_did::Error::MethodNotFound
    ))
  ));

  // INVALID: try sign with a CapabilityInvocation method with an invalid MethodType.
  assert!(matches!(
    account
      .publish_with_options(PublishOptions::default().sign_with(invalid_signing_method))
      .await
      .unwrap_err(),
    Error::IotaCoreError(identity_iota_core::Error::InvalidDocumentSigningMethodType),
  ));

  assert!(account
    .publish_with_options(PublishOptions::default().sign_with(signing_method))
    .await
    .is_ok());
}

#[tokio::test]
async fn test_account_publish_options_force_integration() {
  let config = AccountConfig::default().autopublish(false).testmode(true);
  let client = ClientBuilder::new().node_sync_disabled().build().await.unwrap();
  let account_setup = AccountSetup::new(Arc::new(MemStore::new()), Arc::new(client), config);
  let mut account = Account::create_identity(account_setup, IdentitySetup::new())
    .await
    .unwrap();

  account.publish().await.unwrap();

  let last_int_id = *account.chain_state().last_integration_message_id();

  account
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateEd25519)
    .fragment("test-auth")
    .scope(MethodScope::authentication())
    .apply()
    .await
    .unwrap();

  account
    .publish_with_options(PublishOptions::default().force_integration_update(true))
    .await
    .unwrap();

  // Ensure update was published on integration chain.
  assert_ne!(account.chain_state().last_integration_message_id(), &last_int_id);
  assert_eq!(account.chain_state().last_diff_message_id(), &MessageId::null());
}

#[tokio::test]
async fn test_account_has_document_with_valid_signature_after_publication() {
  let config = AccountConfig::default().autopublish(false).testmode(true);
  let client = ClientBuilder::new().node_sync_disabled().build().await.unwrap();
  let account_setup = AccountSetup::new(Arc::new(MemStore::new()), Arc::new(client), config);
  let mut account = Account::create_identity(account_setup, IdentitySetup::new())
    .await
    .unwrap();

  account.publish().await.unwrap();

  let initial_doc: IotaDocument = account.document().to_owned();
  initial_doc.verify_document(account.document()).unwrap();

  account
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateEd25519)
    .fragment("another-sign")
    .scope(MethodScope::capability_invocation())
    .apply()
    .await
    .unwrap();

  account
    .update_identity()
    .delete_method()
    .fragment(IotaDocument::DEFAULT_METHOD_FRAGMENT)
    .apply()
    .await
    .unwrap();

  // We have to force an integratino update here, because in test mode,
  // the account still publishes a diff update otherwise.
  account
    .publish_with_options(PublishOptions::default().force_integration_update(true))
    .await
    .unwrap();

  // Account document has a valid signature with respect to the initial doc.
  initial_doc.verify_document(account.document()).unwrap();

  account
    .update_identity()
    .create_method()
    .content(MethodContent::GenerateEd25519)
    .fragment("test-key-2")
    .scope(MethodScope::authentication())
    .apply()
    .await
    .unwrap();

  // If we don't publish, the document will not be signed.
  initial_doc.verify_document(account.document()).unwrap_err();
}

#[tokio::test]
async fn test_account_sync_no_changes() -> Result<()> {
  network_resilient_test(2, |n| {
    Box::pin(async move {
      let network = if n % 2 == 0 { Network::Devnet } else { Network::Mainnet };
      let mut account = create_account(network).await;

      // Case 0: Since nothing has been published to the tangle, read_document must return DID not found
      assert!(account.fetch_document().await.is_err());

      // Case 1: Tangle and account are synched
      account.publish().await.unwrap();

      let old_document: IotaDocument = account.document().clone();
      let old_chain_state: ChainState = account.chain_state().clone();

      account.fetch_document().await.unwrap();

      assert_eq!(&old_document, account.document());
      assert_eq!(&old_chain_state, account.chain_state());

      // Case 2: Local document is ahead of the tangle
      account
        .update_identity()
        .create_service()
        .fragment("my-other-service")
        .type_("LinkedDomains")
        .endpoint(Url::parse("https://example.org").unwrap())
        .apply()
        .await
        .unwrap();

      let old_document: IotaDocument = account.document().clone();
      let old_chain_state: ChainState = account.chain_state().clone();

      account.fetch_document().await.unwrap();

      assert_eq!(&old_document, account.document());
      assert_eq!(&old_chain_state, account.chain_state());

      Ok(())
    })
  })
  .await
  .unwrap();
  Ok(())
}

#[tokio::test]
async fn test_account_sync_integration_msg_update() {
  network_resilient_test(2, |n| {
    Box::pin(async move {
      let network = if n % 2 == 0 { Network::Devnet } else { Network::Mainnet };
      let mut account = create_account(network.clone()).await;
      account.publish().await.unwrap();

      let client: Client = Client::builder().network(network).build().await.unwrap();
      let mut new_doc: IotaDocument = account.document().clone();
      new_doc.properties_mut().insert("foo".into(), 123u32.into());
      new_doc.properties_mut().insert("bar".into(), 456u32.into());
      new_doc.metadata.previous_message_id = *account.chain_state().last_integration_message_id();
      new_doc.metadata.updated = Some(Timestamp::now_utc());
      account
        .sign(
          IotaDocument::DEFAULT_METHOD_FRAGMENT,
          &mut new_doc,
          ProofOptions::default(),
        )
        .await
        .unwrap();
      client.publish_document(&new_doc).await.unwrap();
      let chain: DocumentChain = client.read_document_chain(account.did()).await.unwrap();

      account.fetch_document().await.unwrap();
      assert!(account.document().properties().contains_key("foo"));
      assert!(account.document().properties().contains_key("bar"));
      assert_eq!(
        account.chain_state().last_integration_message_id(),
        chain.integration_message_id()
      );
      assert_eq!(account.chain_state().last_diff_message_id(), chain.diff_message_id());
      // Ensure state was written into storage.
      let storage_document: IotaDocument = account.load_document().await.unwrap();
      assert_eq!(&storage_document, account.document());
      Ok(())
    })
  })
  .await
  .unwrap();
}

#[tokio::test]
async fn test_account_sync_diff_msg_update() {
  network_resilient_test(2, |n| {
    Box::pin(async move {
      let network = if n % 2 == 0 { Network::Devnet } else { Network::Mainnet };
      let mut account = create_account(network.clone()).await;
      account.publish().await.unwrap();

      let client: Client = Client::builder().network(network).build().await.unwrap();
      let mut new_doc: IotaDocument = account.document().clone();
      new_doc.properties_mut().insert("foo".into(), 123u32.into());
      new_doc.properties_mut().insert("bar".into(), 456u32.into());
      new_doc.metadata.updated = Some(Timestamp::now_utc());
      let mut diff_msg: DiffMessage = DiffMessage::new(
        account.document(),
        &new_doc,
        *account.chain_state().last_integration_message_id(),
      )
      .unwrap();
      account
        .sign(
          IotaDocument::DEFAULT_METHOD_FRAGMENT,
          &mut diff_msg,
          ProofOptions::default(),
        )
        .await
        .unwrap();
      client
        .publish_diff(&*account.chain_state().last_integration_message_id(), &diff_msg)
        .await
        .unwrap();
      let chain: DocumentChain = client.read_document_chain(account.did()).await.unwrap();

      let old_chain_state: ChainState = account.chain_state().clone();
      account.fetch_document().await.unwrap();
      assert!(account.document().properties().contains_key("foo"));
      assert!(account.document().properties().contains_key("bar"));
      assert_eq!(
        old_chain_state.last_integration_message_id(),
        account.chain_state().last_integration_message_id()
      );
      assert_eq!(account.chain_state().last_diff_message_id(), chain.diff_message_id());
      // Ensure state was written into storage.
      let storage_document: IotaDocument = account.load_document().await.unwrap();
      assert_eq!(&storage_document, account.document());
      Ok(())
    })
  })
  .await
  .unwrap();
}

async fn create_account(network: Network) -> Account {
  Account::builder()
    .storage(
      Stronghold::new(&temporary_random_path(), "my-password".to_owned(), None)
        .await
        .unwrap(),
    )
    .autopublish(false)
    .autosave(AutoSave::Every)
    .client_builder(ClientBuilder::new().network(network.clone()))
    .create_identity(IdentitySetup::default())
    .await
    .unwrap()
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
      error @ Err(Error::IotaError(identity_iota::Error::ClientError(_))) => {
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

// Ensure that a future that contains an account is `Send` at compile-time.
#[tokio::test]
async fn test_assert_account_futures_are_send() -> Result<()> {
  fn assert_future_send<T: std::future::Future + Send>(_: T) {}

  let mut account: Account = AccountBuilder::default()
    .testmode(true)
    .create_identity(IdentitySetup::default())
    .await?;

  assert_future_send(account.update_identity().create_method().apply());

  Ok(())
}

#[tokio::test]
async fn test_storage_index() {
  for storage in storages().await {
    let setup: AccountSetup = account_setup_storage(storage, Network::Mainnet).await;

    let account1 = Account::create_identity(setup.clone(), IdentitySetup::default())
      .await
      .unwrap();

    let index: Vec<IotaDID> = account1.storage().did_list().await.unwrap();

    assert_eq!(index.len(), 1);
    assert!(index.contains(account1.did()));

    let account2: Account = Account::create_identity(setup, IdentitySetup::default()).await.unwrap();

    let index: Vec<IotaDID> = account2.storage().did_list().await.unwrap();

    assert_eq!(index.len(), 2);
    assert!(index.contains(account1.did()));
    assert!(index.contains(account2.did()));

    assert!(account2.storage().did_exists(account1.did()).await.unwrap());
    assert!(account2.storage().did_exists(account2.did()).await.unwrap());

    let account1_did: IotaDID = account1.did().to_owned();
    account1.delete_identity().await.unwrap();

    assert!(!account2.storage().did_exists(&account1_did).await.unwrap());
    assert!(account2.storage().did_exists(account2.did()).await.unwrap());
    assert_eq!(account2.storage().did_list().await.unwrap().len(), 1);
  }
}
