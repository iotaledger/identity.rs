//! Publish new did document and read it from the tangle
//! cargo run --example publish_read

use anyhow::Result;
use identity_crypto::{Ed25519, KeyGen, KeyGenerator};
use identity_iota::{
    core::{did::DIDDocument, diff::Diff, key::KeyRelation},
    did::{DIDDiff, DIDProof, TangleDocument as _},
    error::Error,
    helpers::create_document,
    io::{TangleReader, TangleWriter},
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

    // Create keypair
    let keypair = Ed25519::generate(&Ed25519, KeyGenerator::default())?;
    let bs58_auth_key = bs58::encode(keypair.public()).into_string();

    // Create, sign and publish DID document to the Tangle
    let mut did_document = create_document(bs58_auth_key.clone())?;

    did_document.sign_unchecked(keypair.secret())?;

    println!("DID: {}", did_document.did());

    let tail_transaction = tangle_writer.write_json(did_document.did(), &did_document).await?;

    println!(
        "DID document published: https://thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
    );

    // Create, sign and publish diff to the Tangle
    let signed_diff = create_diff(did_document.clone(), &keypair).await?;
    let auth_key = did_document
        .resolve_key(0, KeyRelation::Authentication)
        .ok_or(Error::InvalidAuthenticationKey)?;
    let tail_transaction = tangle_writer
        .publish_diff_json(&did_document.did(), auth_key.key_data(), &signed_diff)
        .await?;

    println!(
        "DID document DIDDiff published: https://thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
    );

    // Get document and diff from the tangle and validate the signatures
    let did = did_document.did();
    let tangle_reader = TangleReader::new(&nodelist)?;

    let received_messages = tangle_reader.fetch(&did).await?;
    println!("{:?}", received_messages);

    let docs = TangleReader::extract_documents(&did, &received_messages)?;
    println!("extracted docs: {:?}", docs);

    let diffs = TangleReader::extract_diffs(&did, &received_messages)?;
    println!("extracted diffs: {:?}", diffs);

    let sig = docs[0].data.verify_unchecked().is_ok();
    println!("Document has valid signature: {}", sig);

    let sig = docs[0].data.verify_diff_unchecked(&diffs[0].data).is_ok();
    println!("Diff has valid signature: {}", sig);

    Ok(())
}

async fn create_diff(did_document: DIDDocument, keypair: &identity_crypto::KeyPair) -> crate::Result<DIDDiff> {
    // updated doc and publish diff
    let mut new = did_document.clone();
    new.update_time();

    // diff the two docs.
    let diff = did_document.diff(&new)?;

    let key_did = new
        .resolve_key(0, KeyRelation::Authentication)
        .ok_or(Error::InvalidAuthenticationKey)?;
    let mut diddiff = DIDDiff {
        id: new.did().clone(),
        diff: serde_json::to_string(&diff)?,
        proof: DIDProof::new(key_did.id().clone())?,
    };

    did_document.sign_diff_unchecked(&mut diddiff, keypair.secret())?;

    Ok(diddiff)
}
