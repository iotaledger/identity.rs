//! Publish new did document and read it from the tangle
//! cargo run --example publish_read

use identity_core::key::PublicKey;
use identity_crypto::KeyPair;
use identity_iota::{
    did::{IotaDID, IotaDocument},
    error::Result,
    helpers::create_ed25519_key,
    io::{TangleReader, TangleWriter},
    network::{Network, NodeList},
};
use iota_conversion::Trinary as _;

#[smol_potat::main]
async fn main() -> Result<()> {
    let nodes = vec!["http://localhost:14265", "https://nodes.comnet.thetangle.org:443"];
    let nodelist = NodeList::with_network_and_nodes(Network::Comnet, nodes);

    let tangle_writer = TangleWriter::new(&nodelist)?;

    // Create keypair
    let keypair: KeyPair = IotaDocument::generate_ed25519_keypair();

    // Create comnet DID and authentication method
    let did: IotaDID = IotaDID::with_network(keypair.public().as_ref(), "com")?;
    let key: PublicKey = create_ed25519_key(&did, keypair.public().as_ref())?;

    // Create a minimal DID document from the DID and authentication method
    let mut document: IotaDocument = IotaDocument::new(did, key)?;

    // Sign the document with the authentication method secret
    document.sign(keypair.secret())?;

    // Ensure the document proof is valid
    assert!(document.verify().is_ok());

    let tail_transaction = tangle_writer.write_json(document.did(), &document).await?;

    println!(
        "DID document published: https://comnet.thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
    );

    // Update document and publish diff to the Tangle
    let mut update = document.clone();

    update.set_metadata("new-value", true);

    let signed_diff = document.diff(update.into(), keypair.secret())?;

    // Ensure the diff proof is valid
    assert!(document.verify_diff(&signed_diff).is_ok());

    let tail_transaction = tangle_writer.publish_json(&document.did(), &signed_diff).await?;

    println!(
        "DID document DIDDiff published: https://comnet.thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
    );

    // Get document and diff from the tangle and validate the signatures
    let did = document.did();
    let tangle_reader = TangleReader::new(&nodelist)?;

    let received_messages = tangle_reader.fetch(&did).await?;
    println!("{:?}", received_messages);

    let mut docs = TangleReader::extract_documents(&did, &received_messages)?;
    println!("extracted docs: {:?}", docs);

    let diffs = TangleReader::extract_diffs(&did, &received_messages)?;
    println!("extracted diffs: {:?}", diffs);

    let doc = IotaDocument::try_from_document(docs.remove(0).data)?;
    let sig = doc.verify().is_ok();
    println!("Document has valid signature: {}", sig);

    let sig = doc.verify_diff(&diffs[0].data).is_ok();
    println!("Diff has valid signature: {}", sig);

    Ok(())
}
