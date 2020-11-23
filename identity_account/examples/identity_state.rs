//! Publish new did document and read it from the tangle
//! cargo run --example identity_state

use anyhow::Result;
use identity_account::identity_state::State;
use identity_crypto::KeyPair;
use identity_iota::{
    client::{Client, ClientBuilder},
    did::IotaDocument,
    network::Network,
};

#[smol_potat::main]
async fn main() -> Result<()> {
    let filename = "test";
    let (document, keypair, client) = create_and_publish().await?;
    // let (document, keypair): (IotaDocument, KeyPair) = IotaDocument::generate_ed25519("key-1", None)?;
    println!("DID: {}", document.did());
    let mut state = State::new(keypair.clone(), document.clone())?;
    state.write_to_file(filename)?;

    // Adds diff only locally, results in not synced state
    let mut update = document.clone();
    update.set_metadata("new-value", true);
    let signed_diff = document.diff(update.into(), keypair.secret())?;
    assert!(document.verify_diff(&signed_diff).is_ok());
    state.add_diff(signed_diff)?;
    state.write_to_file(filename)?;

    let read_state = State::read_from_file(filename)?;
    assert_eq!(
        document,
        read_state.documents().last().expect("Can't get last doc").document
    );
    let synced = read_state.is_synced(client.clone()).await?;
    // Should be false, because the diff was only updated locally
    assert_eq!(synced, false);
    println!("synced: {}", synced);
    Ok(())
}

pub async fn create_and_publish() -> Result<(IotaDocument, KeyPair, Client)> {
    let client: Client = ClientBuilder::new()
        .node("https://papa.iota.family:14267")
        .network(Network::Mainnet)
        .build()?;

    // Create keypair/DID document
    let (mut document, keypair): (IotaDocument, KeyPair) = IotaDocument::generate_ed25519("key-1", None)?;

    // Sign the document with the authentication method secret
    document.sign(keypair.secret())?;

    // Ensure the document proof is valid
    assert!(document.verify().is_ok());

    let response = client.create_document(&document).send().await?;

    println!("DID document published: {}", client.transaction_url(&response.tail));
    Ok((document, keypair, client))
}
