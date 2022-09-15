// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
// Use the external did-key crate to avoid implementing the entire DID key method in this example.
use did_key::Config;
use did_key::DIDCore;
use did_key::DIDKey;
use examples::create_did;
use examples::random_stronghold_path;
use examples::API_ENDPOINT;
use identity_iota::core::FromJson;
use identity_iota::core::ToJson;
use identity_iota::credential::AbstractThreadSafeValidatorDocument;
use identity_iota::did::CoreDID;
use identity_iota::did::CoreDocument;
use identity_iota::did::DID;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::resolver::Resolver;
use iota_client::block::address::Address;
use iota_client::secret::stronghold::StrongholdSecretManager;
use iota_client::secret::SecretManager;
use iota_client::Client;

/// Demonstrates how to set up a resolver using custom handlers.
///
/// NOTE: Since both `IotaDocument` and `CoreDocument` implement `Into<CoreDocument>` we could have used
/// Resolver<CoreDocument> in this example and just worked with `CoreDocument` representations throughout.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a method agnostic resolver and attach handlers for the "key" and "iota" methods.
  let mut resolver: Resolver = Resolver::new();
  resolver.attach_handler("key".to_owned(), resolve_ed25519_did_key);

  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder().with_primary_node(API_ENDPOINT, None)?.finish()?;

  resolver.attach_iota_handler(client.clone());

  // A valid Ed25519 did:key value taken from https://w3c-ccg.github.io/did-method-key/#example-1-a-simple-ed25519-did-key-value.
  let did_key: CoreDID = "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".parse()?;

  // Create a new secret manager backed by a Stronghold.
  let mut secret_manager: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password")
      .build(random_stronghold_path())?,
  );

  // Create a new DID for us to resolve.
  let (_, iota_did): (Address, IotaDID) = create_did(&client, &mut secret_manager).await?;

  // Resolve did_key to get an abstract document.
  let did_key_doc: AbstractThreadSafeValidatorDocument = resolver.resolve(&did_key).await?;

  // Resolve iota_did to get an abstract document.
  let iota_doc: AbstractThreadSafeValidatorDocument = resolver.resolve(&iota_did).await?;

  // These documents are mainly meant for validating credentials and presentations, but one can also attempt to cast
  // them to concrete document types.

  let did_key_doc: CoreDocument = *did_key_doc
    .into_any()
    .downcast::<CoreDocument>()
    .expect("downcasting to the return type of the did:key handler should be fine");

  println!("Resolved DID Key document: {}", did_key_doc.to_json_pretty()?);

  let iota_doc: IotaDocument = *iota_doc
    .into_any()
    .downcast::<IotaDocument>()
    .expect("downcasting to the return type of the iota handler should be fine");
  println!("Resolved DID iota document: {}", iota_doc.to_json_pretty()?);

  Ok(())
}

/// Resolves an Ed25519 did:key DID to a spec compliant DID Document represented as a [`CoreDocument`].
///
/// # Errors
/// Errors if the DID is not a valid Ed25519 did:key.  
async fn resolve_ed25519_did_key(did: CoreDID) -> Result<CoreDocument, Box<dyn std::error::Error + Send + Sync>> {
  DIDKey::try_from(did.as_str())
    .map_err(|err| anyhow::anyhow!("the provided DID does not satisfy the did:key format: {:?}", err))
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
