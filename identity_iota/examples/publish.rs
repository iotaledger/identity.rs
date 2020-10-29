//! Publish new did document and read it from the tangle
//! cargo run --example publish

use identity_core::key::PublicKey;
use identity_crypto::KeyPair;
use identity_iota::{
    client::{Client, ClientBuilder, TransactionPrinter},
    did::{IotaDID, IotaDocument},
    error::Result,
    helpers::create_ed25519_key,
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

    // Create keypair
    let keypair: KeyPair = IotaDocument::generate_ed25519_keypair();

    // Create DID and authentication method
    let did: IotaDID = IotaDID::new(keypair.public().as_ref())?;
    let key: PublicKey = create_ed25519_key(&did, keypair.public().as_ref())?;

    // Create a minimal DID document from the DID and authentication method
    let mut document: IotaDocument = IotaDocument::new(did, key)?;

    // Sign the document with the authentication method secret
    document.sign(keypair.secret())?;

    // Ensure the document proof is valid
    assert!(document.verify().is_ok());

    println!("DID: {}", document.did());

    let response = client.create_document(&document).send().await?;

    println!(
        "DID document published: https://thetangle.org/transaction/{}",
        TransactionPrinter::hash(&response.tail),
    );

    Ok(())
}
