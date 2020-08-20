//! Publish new did document and read it from the tangle
//! cargo run --example publish_read

use anyhow::Result;
use identity_core::{did::DID, document::DIDDocument};
use identity_integration::{
    did_helper::did_iota_address,
    tangle_reader::TangleReader,
    tangle_writer::{iota_network, DIDMessage, Payload, TangleWriter},
};
use iota_conversion::Trinary;

#[smol_potat::main]
async fn main() -> Result<()> {
    let nodes = vec!["http://localhost:14265", "https://nodes.comnet.thetangle.org:443"];
    let did = "did:iota:123456789abcdefghij";
    let did_address = did_iota_address(&DID::parse_from_str(did).unwrap().id_segments[0]);
    // 1. Publish DID document to the Tangle
    let tangle_writer = TangleWriter {
        nodes: nodes.clone(),
        network: iota_network::Comnet,
    };
    let did_document = DIDDocument::new(String::from("https://www.w3.org/ns/did/v1"), String::from(did)).unwrap();
    let did_message = DIDMessage {
        payload: Payload::DIDDocument(did_document),
        // payload: Payload::DIDDocument(String::from("Document")),
        address: did_address.clone(),
    };
    let tail_transaction = tangle_writer.publish_document(&did_message).await.unwrap();

    println!(
        "DID document published: https://comnet.thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().unwrap()
    );
    // Get confirmation status, promote if unconfirmed, this needs to be done until it's confirmed or older than 150
    // seconds, then a new transaction needs to be sent
    let confirmed = tangle_writer.is_confirmed(tail_transaction).await.unwrap();
    if !confirmed {
        tangle_writer.promote(tail_transaction).await.unwrap();
    }

    // 2. Fetch messages from DID root address
    let tangle_reader = TangleReader { nodes: nodes };
    let received_message = tangle_reader.fetch(&did_address).await.unwrap();
    // Check if sent message is the same as the received one
    let fetched_did_message: DIDMessage = serde_json::from_str(&received_message[0]).unwrap();
    println!("Document from the tangle: {:?}", fetched_did_message);
    assert_eq!(did_message, fetched_did_message);
    Ok(())
}
