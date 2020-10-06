//! Publish new did document and read it from the tangle
//! cargo run --example publish_read

use anyhow::Result;
use identity_crypto::{Ed25519, KeyGen, KeyGenerator};
use identity_integration::{
    helpers::*,
    tangle_writer::{iota_network, IOTAWriter, IdentityWriter, Payload},
};
use iota_conversion::Trinary;

#[smol_potat::main]
async fn main() -> Result<()> {
    let nodes = vec!["http://localhost:14265", "https://nodes.comnet.thetangle.org:443"];
    let tangle_writer = IOTAWriter::new(nodes.clone(), iota_network::Comnet).await?;
    // Create keypair
    let keypair = Ed25519::generate(&Ed25519, KeyGenerator::default())?;
    let bs58_auth_key = bs58::encode(hex::decode(keypair.public().to_string())?).into_string();

    // Create, sign and publish DID document to the Tangle
    let did_document = create_document(bs58_auth_key.clone())?;
    let did_payload = Payload::DIDDocument(did_document.clone());
    let signed_payload = sign_payload(&keypair, did_payload.clone())?;
    let tail_transaction = tangle_writer.send(&signed_payload).await?;
    println!(
        "DID document published: https://comnet.thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
    );
    Ok(())
}
