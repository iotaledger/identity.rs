// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use anyhow::Context;
use did_key::Config;
use did_key::DIDCore;
use did_key::DIDKey;
use did_key::KeyPair;
use examples::create_did;
use examples::random_stronghold_path;
use examples::API_ENDPOINT;
use identity_iota::core::FromJson;
use identity_iota::core::ToJson;
use identity_iota::did::CoreDID;
use identity_iota::did::CoreDocument;
use identity_iota::did::MethodScope;
use identity_iota::did::DID;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaVerificationMethod;
use identity_iota::prelude::KeyType;
use identity_iota::resolver::Resolver;
use iota_client::block::address::Address;
use iota_client::block::output::AliasOutput;
use iota_client::secret::stronghold::StrongholdSecretManager;
use iota_client::secret::SecretManager;
use iota_client::Client;

/// Resolves an Ed25519 did:key DID to a spec compliant DID Document.
///
/// # Errors
/// Errors if the DID is not a valid Ed25519 did:key  
async fn resolve_ed25519_did_key(did: CoreDID) -> Result<CoreDocument, Box<dyn std::error::Error + Send + Sync>> {
  DIDKey::try_from(did.as_str())
    .map_err(|err| anyhow::anyhow!("The provided DID does not satisfy the did:key format: {:?}", err))
    .map(|key| {
      key.get_did_document(Config {
        use_jose_format: false,
        serialize_secrets: false,
      })
    })?
    .to_json()
    .and_then(|doc_json| CoreDocument::from_json(&doc_json))
    .context("failed to obtain a supported DID Document")
    .map_err(Into::into)
}

/// Demonstrates how to set up a resolver using custom handlers.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let mut resolver: Resolver<CoreDocument> = Resolver::new();
  resolver.attach_handler("key".to_owned(), resolve_ed25519_did_key);

  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder().with_primary_node(API_ENDPOINT, None)?.finish()?;

  resolver.attach_iota_handler(client);

  // Create a new secret manager backed by a Stronghold.
  let mut secret_manager: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password")
      .build(random_stronghold_path())?,
  );

  Ok(())
}
