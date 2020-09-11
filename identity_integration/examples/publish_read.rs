//! Publish new did document and read it from the tangle
//! cargo run --example publish_read

use anyhow::Result;
use identity_core::{
    document::DIDDocument,
    utils::{Context, Subject},
};
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
    let did = "did:iota:com:123456789abcdefghi";
    let did_document = DIDDocument {
        context: Context::from("https://www.w3.org/ns/did/v1"),
        id: Subject::from(did),
        ..Default::default()
    }
    .init()
    .init_timestamps()?;
    let did_address = did_iota_address(
        &did_document
            .derive_did()?
            .id_segments
            .last()
            .expect("Failed to get id_segment"),
    )?;
    let did_payload = Payload::DIDDocument(did_document);
    // 1. Publish DID document to the Tangle
    let tangle_writer = TangleWriter::new(nodes.clone(), iota_network::Comnet)?;

    let mut tail_transaction = tangle_writer.publish_document(&did_payload).await?;
    println!(
        "DID document published: https://comnet.thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
    );
    // Get confirmation status, promote if unconfirmed, this needs to be done until it's confirmed or older than 150
    // seconds, then a new transaction needs to be sent
    Timer::after(Duration::from_secs(5)).await;
    let mut j = 0;
    while !tangle_writer.is_confirmed(tail_transaction).await? {
        j += 1;
        Timer::after(Duration::from_secs(5)).await;
        let promotehash = tangle_writer.promote(tail_transaction).await?;
        println!("Promoted: https://comnet.thetangle.org/transaction/{}", promotehash);
        // Send the document again if the previous transaction didn't get confirmed after 150 seconds
        if j % 30 == 0 {
            tail_transaction = tangle_writer.publish_document(&did_payload).await?;
            println!(
                "DID document published again: https://comnet.thetangle.org/transaction/{}",
                tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
            );
        }
    }
    println!("Transaction got confirmed!");

    // 2. Fetch messages from DID address
    let tangle_reader = TangleReader::new(nodes);
    let received_message = tangle_reader.fetch(&did_address).await?;
    let fetched_did_document =
        DIDDocument::from_str(&received_message.values().cloned().next().expect("Couldn't get message"))?;
    println!("Document from the Tangle: {:?}", fetched_did_document);
    // Check if sent message is the same as the received one
    if let Payload::DIDDocument(doc) = did_payload {
        assert_eq!(doc, fetched_did_document);
    };
    Ok(())
}
