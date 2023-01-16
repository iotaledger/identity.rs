// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use examples::create_did;
use examples::random_stronghold_path;
use examples::API_ENDPOINT;
use identity_iota::core::FromJson;
use identity_iota::core::ToJson;
use identity_iota::credential::AbstractThreadSafeValidatorDocument;
use identity_iota::crypto::KeyPair as IotaKeyPair;
use identity_iota::did::CoreDID;
use identity_iota::document::CoreDocument;
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
  // Create a method agnostic resolver and attach handlers for the "foo" and "iota" methods.
  let mut resolver: Resolver = Resolver::new();

  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder().with_primary_node(API_ENDPOINT, None)?.finish()?;

  // This is a convenience method for attaching a handler for the "iota" method by providing just a client.
  resolver.attach_iota_handler(client.clone());
  resolver.attach_handler("foo".to_owned(), resolve_did_foo);

  // A fake did:foo DID for demonstration purposes.
  let did_foo: CoreDID = "did:foo:0e9c8294eeafee326a4e96d65dbeaca0".parse()?;

  // Create a new secret manager backed by a Stronghold.
  let mut secret_manager: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password")
      .build(random_stronghold_path())?,
  );

  // Create a new DID for us to resolve.
  let (_, iota_document, _): (Address, IotaDocument, IotaKeyPair) = create_did(&client, &mut secret_manager).await?;
  let iota_did: IotaDID = iota_document.id().clone();

  // Resolve did_foo to get an abstract document.
  let did_foo_doc: AbstractThreadSafeValidatorDocument = resolver.resolve(&did_foo).await?;

  // Resolve iota_did to get an abstract document.
  let iota_doc: AbstractThreadSafeValidatorDocument = resolver.resolve(&iota_did).await?;

  // These documents are mainly meant for validating credentials and presentations, but one can also attempt to cast
  // them to concrete document types.

  let did_foo_doc: CoreDocument = *did_foo_doc
    .into_any()
    .downcast::<CoreDocument>()
    .expect("downcasting to the return type of the did:foo handler should be fine");

  println!("Resolved DID foo document: {}", did_foo_doc.to_json_pretty()?);

  let iota_doc: IotaDocument = *iota_doc
    .into_any()
    .downcast::<IotaDocument>()
    .expect("downcasting to the return type of the iota handler should be fine");
  println!("Resolved IOTA DID document: {}", iota_doc.to_json_pretty()?);

  Ok(())
}

/// Resolve a did:foo to a DID document.
async fn resolve_did_foo(did: CoreDID) -> anyhow::Result<CoreDocument> {
  Ok(
    CoreDocument::from_json(&format!(
      r#"{{
      "id": "{did}",
      "verificationMethod": [
        {{
          "id": "{did}#key-1",
          "controller": "{did}",
          "type": "Ed25519VerificationKey2018",
          "publicKeyMultibase": "zGurPxZGpqnJ6j866DNBXYQJH2KzJjmQ9KBpCYp9oYJom"
        }}
      ]
      }}"#,
    ))
    .unwrap(),
  )
}
