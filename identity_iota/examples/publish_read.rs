//! Publish new did document and read it from the tangle
//! cargo run --example publish_read

use anyhow::Result;
use identity_crypto::{Ed25519, KeyGen, KeyGenerator};
use identity_iota::{
    core::{
        common::Timestamp,
        did::{DIDDocument, KeyData, KeyType, PublicKeyBuilder},
        diff::Diff,
        io::IdentityWriter,
    },
    helpers::{
        create_document, diff_has_valid_signature, doc_has_valid_signature, get_auth_key, sign_diff, sign_document,
    },
    io::{TangleReader, TangleWriter},
    network::{Network, NodeList},
    types::DIDDiff,
};
use iota_conversion::Trinary;

#[smol_potat::main]
async fn main() -> Result<()> {
    let nodes = vec!["http://localhost:14265", "https://nodes.comnet.thetangle.org:443"];
    let nodelist = NodeList::with_network_and_nodes(Network::Comnet, nodes);

    let tangle_writer = TangleWriter::new(&nodelist)?;
    // Create keypair
    let keypair = Ed25519::generate(&Ed25519, KeyGenerator::default())?;
    let bs58_auth_key = bs58::encode(hex::decode(keypair.public().to_string())?).into_string();

    // Create, sign and publish DID document to the Tangle
    let did_document = create_document(bs58_auth_key.clone())?;
    let signed_doc = sign_document(&keypair, did_document.clone())?;
    let tail_transaction = tangle_writer.write_document(&signed_doc).await?;
    println!(
        "DID document published: https://comnet.thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
    );

    // Create, sign and publish diff to the Tangle
    let signed_diff = create_diff(did_document.clone(), &keypair).await?;
    let tail_transaction = tangle_writer
        .publish_json(&did_document.derive_did(), &signed_diff)
        .await?;
    println!(
        "DID document DIDDiff published: https://comnet.thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
    );

    // Get document and diff from the tangle and validate the signatures
    let did = did_document.derive_did();
    let tangle_reader = TangleReader::new(&nodelist)?;

    let received_messages = tangle_reader.fetch(&did).await?;
    println!("{:?}", received_messages);
    let docs = TangleReader::extract_documents(&did, &received_messages)?;
    println!("extracted docs: {:?}", docs);
    let diffs = TangleReader::extract_diffs(&did, &received_messages)?;
    println!("extracted diffs: {:?}", diffs);

    let sig = doc_has_valid_signature(&docs[0].data)?;
    println!("Document has valid signature: {}", sig);

    //get auth key from DIDDocument
    if let Some(auth_key) = get_auth_key(&docs[0].data) {
        let sig = diff_has_valid_signature(diffs[0].data.clone(), &auth_key)?;
        println!("Diff has valid signature: {}", sig);
    }
    Ok(())
}

async fn create_diff(did_document: DIDDocument, keypair: &identity_crypto::KeyPair) -> crate::Result<DIDDiff> {
    // updated doc and publish diff
    let mut new = did_document.clone();

    let public_key = PublicKeyBuilder::default()
        .id("did:iota:123456789abcdefghij#keys-1".parse()?)
        .controller("did:iota:com:123456789abcdefghij".parse()?)
        .key_type(KeyType::RsaVerificationKey2018)
        .key_data(KeyData::PublicKeyBase58(
            "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
        ))
        .build()
        .unwrap();

    new.update_public_key(public_key);
    new.update_time();
    // diff the two docs.
    let diff = did_document.diff(&new)?;
    let diddiff = DIDDiff {
        did: new.derive_did().clone(),
        diff: serde_json::to_string(&diff)?,
        time: Timestamp::now(),
        signature: String::new(),
    };
    Ok(sign_diff(&keypair, diddiff)?)
}
