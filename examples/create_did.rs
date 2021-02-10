use identity::iota::Client;
use identity::iota::IotaDocument;
use identity::crypto::KeyPair;

use identity::iota::Result;


#[smol_potat::main]
async fn main() -> Result<()> {
  let client: Client = Client::new()?;

  let (mut document, keypair): (IotaDocument, KeyPair) = IotaDocument::builder()
    .authentication_tag("key-1")
    .did_network(client.network().as_str())
    .build()?;

  // Sign the DID Document with the default authentication key.
  document.sign(keypair.secret())?;

  // Use the client to publish the DID Document to the Tangle.
  let transaction: _ = client.publish_document(&document).await?;
  println!("DID Document Transaction > {}", client.transaction_url(&transaction));

  Ok(())
}
