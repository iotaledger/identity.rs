// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use examples::create_did;
use examples::random_stronghold_path;
use examples::MemStorage;
use examples::API_ENDPOINT;
use identity_iota::core::FromJson;
use identity_iota::core::ToJson;
use identity_iota::did::CoreDID;
use identity_iota::did::DID;
use identity_iota::document::CoreDocument;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::resolver::Resolver;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::KeyIdMemstore;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::client::Password;
use iota_sdk::types::block::address::Address;

/// Demonstrates how to set up a resolver using custom handlers.
///
/// NOTE: Since both `IotaDocument` and `FooDocument` implement `Into<CoreDocument>` we could have used
/// Resolver<CoreDocument> in this example and just worked with `CoreDocument` representations throughout.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a resolver returning an enum of the documents we are interested in and attach handlers for the "foo" and
  // "iota" methods.
  let mut resolver: Resolver<Document> = Resolver::new();

  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder()
    .with_primary_node(API_ENDPOINT, None)?
    .finish()
    .await?;

  // This is a convenience method for attaching a handler for the "iota" method by providing just a client.
  resolver.attach_iota_handler(client.clone());
  resolver.attach_handler("foo".to_owned(), resolve_did_foo);

  // A fake did:foo DID for demonstration purposes.
  let did_foo: CoreDID = "did:foo:0e9c8294eeafee326a4e96d65dbeaca0".parse()?;

  // Create a new secret manager backed by a Stronghold.
  let mut secret_manager: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password(Password::from("secure_password".to_owned()))
      .build(random_stronghold_path())?,
  );

  // Create a new DID for us to resolve.
  let storage: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  let (_, iota_document, _): (Address, IotaDocument, String) =
    create_did(&client, &mut secret_manager, &storage).await?;
  let iota_did: IotaDID = iota_document.id().clone();

  // Resolve did_foo to get an abstract document.
  let did_foo_doc: Document = resolver.resolve(&did_foo).await?;

  // Resolve iota_did to get an abstract document.
  let iota_doc: Document = resolver.resolve(&iota_did).await?;

  // The Resolver is mainly meant for validating presentations, but here we will just
  // check that the resolved documents match our expectations.

  let Document::Foo(did_foo_document) = did_foo_doc else {
    anyhow::bail!("expected a foo DID document when resolving a foo DID");
  };

  println!(
    "Resolved DID foo document: {}",
    did_foo_document.as_ref().to_json_pretty()?
  );

  let Document::Iota(iota_document) = iota_doc else {
    anyhow::bail!("expected an IOTA DID document when resolving an IOTA DID")
  };

  println!("Resolved IOTA DID document: {}", iota_document.to_json_pretty()?);

  Ok(())
}

// Type safe representation of a document adhering to the imaginary "foo" method.
struct FooDocument(CoreDocument);
impl FooDocument {
  fn new(document: CoreDocument) -> anyhow::Result<Self> {
    if document.id().method() == "foo" {
      Ok(Self(document))
    } else {
      anyhow::bail!("cannot construct foo document: incorrect method")
    }
  }
}

impl AsRef<CoreDocument> for FooDocument {
  fn as_ref(&self) -> &CoreDocument {
    &self.0
  }
}

impl From<FooDocument> for CoreDocument {
  fn from(value: FooDocument) -> Self {
    value.0
  }
}

// Enum of the document types we want to handle.
enum Document {
  Foo(FooDocument),
  Iota(IotaDocument),
}

impl From<FooDocument> for Document {
  fn from(value: FooDocument) -> Self {
    Self::Foo(value)
  }
}

impl From<IotaDocument> for Document {
  fn from(value: IotaDocument) -> Self {
    Self::Iota(value)
  }
}

impl AsRef<CoreDocument> for Document {
  fn as_ref(&self) -> &CoreDocument {
    match self {
      Self::Foo(doc) => doc.as_ref(),
      Self::Iota(doc) => doc.as_ref(),
    }
  }
}

/// Resolve a did to a DID document if the did method is "foo".
async fn resolve_did_foo(did: CoreDID) -> anyhow::Result<FooDocument> {
  let doc = CoreDocument::from_json(&format!(
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
  .unwrap();
  FooDocument::new(doc)
}
