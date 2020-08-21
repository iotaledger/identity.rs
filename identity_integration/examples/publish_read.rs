//! Publish new did document and read it from the tangle
//! cargo run --example publish_read

use anyhow::Result;
use identity_core::{did::DID, document::DIDDocument};
use identity_integration::{
    did_helper::did_iota_address,
    tangle_reader::TangleReader,
    tangle_writer::{iota_network, Payload, TangleWriter},
};
use iota_conversion::Trinary;
use smol::Timer;
use std::{str::FromStr, time::Duration};

#[smol_potat::main]
async fn main() -> Result<()> {
    let nodes = vec!["http://localhost:14265", "https://nodes.comnet.thetangle.org:443"];
    let did = "did:iota:123456789abcdefghi";
    let did_address = did_iota_address(&DID::parse_from_str(did).unwrap().id_segments[0]);
    let did_document = DIDDocument::new("https://www.w3.org/ns/did/v1".into(), did.into()).unwrap();
    let did_payload = Payload::DIDDocument(did_document);
    // 1. Publish DID document to the Tangle
    let tangle_writer = TangleWriter {
        nodes: nodes.clone(),
        network: iota_network::Comnet,
    };

    let mut tail_transaction = tangle_writer.publish_document(&did_payload).await.unwrap();
    println!(
        "DID document published: https://comnet.thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().unwrap()
    );
    // Get confirmation status, promote if unconfirmed, this needs to be done until it's confirmed or older than 150
    // seconds, then a new transaction needs to be sent
    Timer::after(Duration::from_secs(5)).await;
    let mut j = 0;
    while !tangle_writer.is_confirmed(tail_transaction).await.unwrap() {
        j += 1;
        Timer::after(Duration::from_secs(5)).await;
        let promotehash = tangle_writer.promote(tail_transaction).await.unwrap();
        println!("Promoted: https://comnet.thetangle.org/transaction/{}", promotehash);
        // Send the document again if the previous transaction didn't get confirmed after 150 seconds
        if j % 30 == 0 {
            tail_transaction = tangle_writer.publish_document(&did_payload).await.unwrap();
            println!(
                "DID document published again: https://comnet.thetangle.org/transaction/{}",
                tail_transaction.as_i8_slice().trytes().unwrap()
            );
        }
    }
    println!("Transaction got confirmed!");

    // 2. Fetch messages from DID address
    let tangle_reader = TangleReader { nodes: nodes };
    let received_message = tangle_reader.fetch(&did_address).await.unwrap();
    let fetched_did_document = DIDDocument::from_str(&received_message[0])?;
    println!("Document from the Tangle: {:?}", fetched_did_document);
    // Check if sent message is the same as the received one
    if let Payload::DIDDocument(doc) = did_payload {
        assert_eq!(doc, fetched_did_document);
    };
    Ok(())
}
