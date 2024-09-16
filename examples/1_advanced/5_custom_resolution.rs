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
use identity_iota::resolver::CompoundResolver;
use identity_iota::resolver::Error as ResolverError;
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

// Create a resolver capable of resolving FooDocument.
struct FooResolver;
impl Resolver<CoreDID> for FooResolver {
  type Target = FooDocument;
  async fn resolve(&self, input: &CoreDID) -> Result<Self::Target, ResolverError> {
    Ok(resolve_did_foo(input.clone()).await?)
  }
}

// Combine it with a resolver of IotaDocuments, creating a new resolver capable of resolving both.
#[derive(CompoundResolver)]
struct FooAndIotaResolver {
  #[resolver(CoreDID -> FooDocument)]
  foo: FooResolver,
  #[resolver(IotaDID -> IotaDocument)]
  iota: Client,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder()
    .with_primary_node(API_ENDPOINT, None)?
    .finish()
    .await?;

  // Create a resolver capable of resolving both IotaDocument and FooDocument.
  let resolver = FooAndIotaResolver {
    iota: client.clone(),
    foo: FooResolver {},
  };

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
  let did_foo_doc: FooDocument = resolver.resolve(&did_foo).await?;

  // Resolve iota_did to get an abstract document.
  let iota_doc: IotaDocument = resolver.resolve(&iota_did).await?;

  println!("Resolved DID foo document: {}", did_foo_doc.as_ref().to_json_pretty()?);

  println!("Resolved IOTA DID document: {}", iota_doc.to_json_pretty()?);

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
