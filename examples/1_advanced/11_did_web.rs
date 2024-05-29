use std::str::FromStr;

use examples::MemStorage;
use identity_iota::{core::Url, iota::{WebDID, WebDocument}, resolver::Resolver, storage::{JwkMemStore, KeyIdMemstore}, verification::{jws::JwsAlgorithm, MethodScope}};
use identity_iota::storage::JwkDocumentExt;


#[tokio::main]
async fn main() -> anyhow::Result<()> {

  // Create a new Web DID document.
  let mut document: WebDocument = WebDocument::new("https://cybersecurity-links.github.io/.well-known/did.json")?;

  // Insert a new Ed25519 verification method in the DID document.
  let storage: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  document
    .generate_method(
      &storage,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      None,
      MethodScope::VerificationMethod,
    )
    .await?;

  println!("Web DID Document: {:#}", document);

// //   let web_did = WebDID::from_str("did:web:192.168.1.196%3a3000:.well-known:did.json")?; //THIS MUST FAIL!

  let mut resolver = Resolver::<WebDocument>::new();
  resolver.attach_web_handler()?;

  let resolved_document = resolver.resolve(document.id()).await?;
    //TODO: fix Document validation, now is accepting everything
  println!("Resolved Document: {:#}", resolved_document);
  Ok(())
}