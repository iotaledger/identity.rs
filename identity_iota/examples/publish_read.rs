//! Publish new did document and read it from the tangle
//! cargo run --example publish_read

use anyhow::Result;

use identity_core::{common::Timestamp, did::DIDDocument, did::KeyData, did::PublicKey, did::DID};
use identity_crypto::{Ed25519, KeyGen, KeyGenerator};
use identity_diff::Diff;
use identity_iota::{
    helpers::*,
    tangle_reader::{get_ordered_diffs, get_ordered_documents, IOTAReader, IdentityReader},
    tangle_writer::{iota_network, Differences, IOTAWriter, IdentityWriter, Payload},
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

    // Create, sign and publish diff to the Tangle
    let signed_diff_payload = create_diff(did_document.clone(), bs58_auth_key, &keypair).await?;
    let tail_transaction = tangle_writer.publish_document(&signed_diff_payload).await?;
    println!(
        "DID document difference published: https://comnet.thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
    );

    // Get document and diff from the tangle and validate the signatures
    let did = did_document.derive_did();
    let tangle_reader = IOTAReader::new(nodes);

    let received_messages = tangle_reader.fetch(&did).await?;

    let documents = get_ordered_documents(received_messages.clone(), &did)?;
    let fetched_document = documents.first().expect("No document found").document.clone();
    println!("Document from the Tangle: {:?}", fetched_document);
    let sig = has_valid_signature(&Payload::DIDDocument(fetched_document.clone()))?;
    println!("Doc valid signature: {}", sig);

    let diffs = get_ordered_diffs(received_messages, &did)?;
    let fetched_diff = diffs.first().expect("No document found").diff.clone();
    println!("Diff from the Tangle: {:?}", fetched_diff);
    let sig = has_valid_signature(&Payload::DIDDocumentDifferences(fetched_diff))?;
    println!("Diff valid signature: {}", sig);

    // Check if sent message is the same as the received one
    if let Payload::DIDDocument(doc) = signed_payload {
        assert_eq!(doc, fetched_document);
    };
    Ok(())
}

async fn create_diff(
    did_document: DIDDocument,
    bs58_auth_key: String,
    keypair: &identity_crypto::KeyPair,
) -> crate::Result<Payload> {
    // updated doc and publish diff
    let mut new = did_document.clone();
    let public_key = PublicKey {
        id: DID::parse_from_str("did:iota:123456789abcdefghij#keys-1")?,
        // id: "did:iota:123456789abcdefghij#keys-1".into(),
        key_type: "RsaVerificationKey2018".into(),
        controller: DID::parse_from_str("did:iota:com:123456789abcdefghij")?,
        key_data: KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into()),
        ..Default::default()
    }
    .init();
    new.update_public_key(public_key);
    new.update_time();
    // diff the two docs.
    let diff = did_document.diff(&new)?;
    let did_payload = Payload::DIDDocumentDifferences(Differences {
        did: new.derive_did().clone(),
        diff: serde_json::to_string(&diff)?,
        time: Timestamp::now().to_rfc3339(),
        auth_key: bs58_auth_key,
        signature: String::new(),
    });
    Ok(sign_payload(&keypair, did_payload)?)
}
