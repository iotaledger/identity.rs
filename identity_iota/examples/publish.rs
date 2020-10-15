//! Publish new did document and read it from the tangle
//! cargo run --example publish

use identity_crypto::{Ed25519, KeyGen};
use identity_iota::{
    did::TangleDocument as _,
    error::Result,
    helpers::create_document,
    io::TangleWriter,
    network::{Network, NodeList},
};
use iota_conversion::Trinary as _;

#[smol_potat::main]
async fn main() -> Result<()> {
    let nodes = vec!["http://localhost:14265", "https://nodes.comnet.thetangle.org:443"];
    let nodelist = NodeList::with_network_and_nodes(Network::Comnet, nodes);

    let tangle_writer = TangleWriter::new(&nodelist)?;

    // Create keypair
    let keypair = Ed25519::generate(&Ed25519, Default::default())?;
    let bs58_auth_key = bs58::encode(keypair.public()).into_string();

    // Create, sign and publish DID document to the Tangle
    let mut did_document = create_document(bs58_auth_key)?;

    did_document.sign_unchecked(keypair.secret())?;

    println!("DID: {}", did_document.did());

    let tail_transaction = tangle_writer.write_json(did_document.did(), &did_document).await?;

    println!(
        "DID document published: https://comnet.thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
    );

    Ok(())
}
