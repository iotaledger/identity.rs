//! Publish new did document and read it from the tangle
//! cargo run --example publish

use identity_crypto::KeyPair;
use identity_iota::{
    did::IotaDocument,
    error::Result,
    io::TangleWriter,
    network::{Network, NodeList},
};
use iota_conversion::Trinary as _;

#[smol_potat::main]
async fn main() -> Result<()> {
    let nodes = vec![
        "http://localhost:14265",
        "https://nodes.thetangle.org:443",
        "https://iotanode.us:14267",
        "https://pow.iota.community:443",
    ];
    let nodelist = NodeList::with_network_and_nodes(Network::Mainnet, nodes);

    let tangle_writer = TangleWriter::new(&nodelist)?;

    // Create keypair/DID document
    let (mut document, keypair): (IotaDocument, KeyPair) = IotaDocument::generate_ed25519("key-1", None)?;

    // Sign the document with the authentication method secret
    document.sign(keypair.secret())?;

    // Ensure the document proof is valid
    assert!(document.verify().is_ok());

    println!("DID: {}", document.did());

    let tail_transaction = tangle_writer.write_json(document.did(), &document).await?;

    println!(
        "DID document published: https://thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
    );

    Ok(())
}
