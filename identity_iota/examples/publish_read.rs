//! Publish new did document and read it from the tangle
//! cargo run --example publish_read

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

    // Update document and publish diff to the Tangle
    let mut update = document.clone();

    update.set_metadata("new-value", true);

    let signed_diff = document.diff(update.into(), keypair.secret())?;

    // Ensure the diff proof is valid
    assert!(document.verify_diff(&signed_diff).is_ok());

    // let tail_transaction = tangle_writer.publish_json(&document.did(), &signed_diff).await?;

    // println!(
    //     "DID document DIDDiff published: https://thetangle.org/transaction/{}",
    //     tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
    // );

    // // Get document and diff from the tangle and validate the signatures
    // let did = document.did();
    // let tangle_reader = TangleReader::new(&nodelist)?;

    // let received_messages = tangle_reader.fetch(&did).await?;
    // println!("{:?}", received_messages);

    // let mut docs = TangleReader::extract_documents(&did, &received_messages)?;
    // println!("extracted docs: {:?}", docs);

    // let diffs = TangleReader::extract_diffs(&did, &received_messages)?;
    // println!("extracted diffs: {:?}", diffs);

    // let doc = IotaDocument::try_from_document(docs.remove(0).data)?;
    // let sig = doc.verify().is_ok();
    // println!("Document has valid signature: {}", sig);

    // let sig = doc.verify_diff(&diffs[0].data).is_ok();
    // println!("Diff has valid signature: {}", sig);

    Ok(())
}
