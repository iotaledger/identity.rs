//! Publish new did document and read it from the tangle
//! cargo run --example publish

use anyhow::Result;
use identity_crypto::{Ed25519, KeyGen, KeyGenerator};
use identity_iota::{
    core::io::IdentityWriter,
    helpers::{create_document, sign_document},
    io::TangleWriter,
    network::{Network, NodeList},
};
use iota_conversion::Trinary;

#[smol_potat::main]
async fn main() -> Result<()> {
    let nodes = vec!["http://localhost:14265", "https://nodes.comnet.thetangle.org:443"];
    let nodelist = NodeList::with_network_and_nodes(Network::Comnet, nodes);

    let tangle_writer = TangleWriter::new(&nodelist)?;
    // Create keypair
    let keypair = Ed25519::generate(&Ed25519, KeyGenerator::default())?;
    let bs58_auth_key = bs58::encode(hex::decode(keypair.public().to_string())?).into_string();

    // Create, sign and publish DID document to the Tangle
    let did_document = create_document(bs58_auth_key.clone())?;
    let signed_doc = sign_document(&keypair, did_document.clone())?;
    let tail_transaction = tangle_writer.write_document(&signed_doc).await?;
    println!(
        "DID document published: https://comnet.thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
    );
    Ok(())
}
