//! Publish new did document and read it from the tangle
//! cargo run --example publish

use identity_crypto::KeyPair;
use identity_iota::{
    client::{Client, ClientBuilder},
    did::IotaDocument,
    error::Result,
    network::Network,
};

#[smol_potat::main]
async fn main() -> Result<()> {
    let client: Client = ClientBuilder::new()
        .node("http://localhost:14265")
        .node("https://nodes.thetangle.org:443")
        .node("https://iotanode.us:14267")
        .node("https://pow.iota.community:443")
        .network(Network::Mainnet)
        .build()?;

    // Create keypair/DID document
    let (mut document, keypair): (IotaDocument, KeyPair) = IotaDocument::generate_ed25519("key-1", None)?;

    // Sign the document with the authentication method secret
    document.sign(keypair.secret())?;

    // Ensure the document proof is valid
    assert!(document.verify().is_ok());

    println!("DID: {}", document.did());

    let response = client.create_document(&document).send().await?;

    println!("DID document published: {}", client.transaction_url(&response.tail));

    Ok(())
}
