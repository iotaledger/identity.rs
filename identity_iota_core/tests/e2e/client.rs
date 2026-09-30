// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::get_funded_test_client;
use identity_iota_core::rebased::migration;
use identity_iota_core::IotaDocument;

use identity_jose::jws::JwsAlgorithm;
use identity_storage::JwkDocumentExt;
use identity_storage::JwkMemStore;
use identity_verification::MethodScope;
use product_core::product_client::ProductClient;

#[tokio::test]
async fn can_create_an_identity() -> anyhow::Result<()> {
  let test_client = get_funded_test_client().await?;
  let signer = test_client.new_signer().await?;

  let identity = test_client
    .create_identity(IotaDocument::new(test_client.network()))
    .finish()
    .execute(&signer, &test_client)
    .await?
    .output;

  let did = identity.did_document().id();
  assert_eq!(did.network_str(), test_client.network().as_ref());

  Ok(())
}

#[tokio::test]
async fn can_update_a_did_document() -> anyhow::Result<()> {
  let test_client = get_funded_test_client().await?;
  let signer = test_client.new_signer().await?;

  let identity = test_client
    .create_identity(IotaDocument::new(test_client.network()))
    .finish()
    .execute(&signer, &test_client)
    .await?
    .output;

  let mut did_document: IotaDocument = identity.into();
  did_document
    .generate_method(
      signer.storage(),
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      Some("my-key"),
      MethodScope::VerificationMethod,
    )
    .await?;

  let updated_document = test_client
    .publish_did_update(did_document, &signer)
    .await?
    .execute(&signer, &test_client)
    .await?
    .output;

  // Fetch the document again and make sure it matches the result of the update.
  let fetched_document = test_client.resolve_did(updated_document.id()).await?;

  assert_eq!(updated_document, fetched_document);

  Ok(())
}

#[tokio::test]
async fn can_resolve_a_new_identity() -> anyhow::Result<()> {
  let test_client = get_funded_test_client().await?;
  let signer = test_client.new_signer().await?;

  let new_identity = test_client
    .create_identity(IotaDocument::new(test_client.network()))
    .finish()
    .execute(&signer, &test_client)
    .await?
    .output;

  let identity = migration::get_identity(&test_client, new_identity.id()).await?;

  assert!(identity.is_some());

  Ok(())
}
