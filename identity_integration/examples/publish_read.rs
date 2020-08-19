// Publish new did document and read it from the tangle
// cargo run --example publish_read

use anyhow::Result;
use identity_core::{did::DID, document::DIDDocument};
use identity_integration::{
    did_helper::did_iota_address,
    tangle_reader::TangleReader,
    tangle_writer::{iota_network, DIDMessage, Payload, TangleWriter},
};
use iota::transaction::bundled::BundledTransactionField;
use iota_conversion::Trinary;

#[smol_potat::main]
async fn main() -> Result<()> {
    let nodes = vec!["http://localhost:14265", "https://nodes.comnet.thetangle.org:443"];
    let did_root_address =
        did_iota_address(&DID::parse_from_str("did:iota:123456789abcdefghi").unwrap().id_segments[0]);
    // 1. Publish demo DID document to the Tangle
    let tangle_writer = TangleWriter {
        nodes: nodes.clone(),
        network: iota_network::Comnet,
    };
    let did_doc = DIDDocument::new(
        String::from("https://www.w3.org/ns/did/v1"),
        String::from("did:iota:123456789abcdefghi"),
    )
    .unwrap();
    let did_message = DIDMessage {
        payload: Payload::DIDDocument(serde_json::to_string(&did_doc).unwrap()),
        // payload: Payload::DIDDocument(String::from("Document")),
        address: did_root_address.clone(),
    };
    let bundle_trytes = tangle_writer.publish_document(&did_message).await.unwrap();

    println!(
        "DID document published: https://comnet.thetangle.org/bundle/{}",
        bundle_trytes[0].bundle().to_inner().as_i8_slice().trytes().unwrap()
    );
    // Get confirmation status, promote or reattach
    // let (tail_hash, _confirmed) = tangle_writer.is_confirmed(*bundle_trytes[0].bundle()).await.unwrap();
    // if let Some(tail_hash) = tail_hash {
    //   tangle_writer.promote(tail_hash).await.unwrap();
    // } else {
    //   tangle_writer.reattach(bundle_trytes).await.unwrap();
    // }

    // 2. Fetch messages from DID root address
    let tangle_reader = TangleReader { nodes: nodes };
    let received_message = tangle_reader.fetch(&did_root_address).await.unwrap();
    // Check if sent message is the same as the received one
    let fetched_did_message: DIDMessage = serde_json::from_str(&received_message[0]).unwrap();
    assert_eq!(did_message, fetched_did_message);
    Ok(())
}
