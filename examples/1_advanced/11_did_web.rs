use examples::MemStorage;
use identity_iota::{core::Url, document::CoreDocument, resolver::Resolver, storage::{JwkMemStore, KeyIdMemstore}, verification::{jws::JwsAlgorithm, MethodScope}, web::{WebClient, WebClientBuilder, WebDocument}};
use identity_iota::storage::JwkDocumentExt;
use reqwest::{Certificate, ClientBuilder};
use tokio::{fs::File, io::AsyncReadExt};


#[tokio::main]
async fn main() -> anyhow::Result<()> {

  let did_url: &str = "https://localhost:4443/.well-known/did.json";
  let path_did_file: &str = "C:/Projects/did-web-server/.well-known/did.json";

  // Create a new client to make HTTPS requests.
  // let client = WebClient::default()?;
  let client= WebClient::new(ClientBuilder::new()
  .danger_accept_invalid_certs(true) //TODO: fix problem cannot build WebClient after calling function of inner structure
  .build()?);

  // Create a new Web DID document.
  let mut document: WebDocument = WebDocument::new(did_url)?;
  // let doc: WebDocument = client.get(Url::parse(did_url)?).send().await?.json().await?;
  // println!("PPPPP: {}", doc);

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

  document.write_to_file(Some(path_did_file))?;
  println!("Web DID Document: {:#}", document);


  // let web_did = WebDID::from_str("did:web:192.168.1.196%3a3000:.well-known:did.json")?; //THIS MUST FAIL!


  let mut resolver = Resolver::<WebDocument>::new();
  resolver.attach_web_handler(client)?;

  let resolved_document = resolver.resolve(document.id()).await?;
  println!("Resolved Document: {:#}", resolved_document);


  Ok(())
}