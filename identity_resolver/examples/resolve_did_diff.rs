//! Publish new did document and read it from the tangle
//! cargo run --example resolve_did_diff
use anyhow::Result;
use chrono::prelude::*;
use identity_core::{
    did::DID,
    document::DIDDocument,
    utils::{Context, PublicKey, Subject},
};
use identity_integration::tangle_writer::{iota_network, Differences, Payload, TangleWriter};
use identity_resolver::resolver::{NetworkNodes, ResolutionInputMetadata, Resolver};
use iota_conversion::Trinary;
use serde_diff::{Apply, Diff};

#[smol_potat::main]
async fn main() -> Result<()> {
    let nodes = vec!["http://localhost:14265", "https://nodes.comnet.thetangle.org:443"];
    let tangle_writer = TangleWriter::new(nodes.clone(), iota_network::Comnet)?;

    // Create and publish first document version
    let mut old = DIDDocument {
        context: Context::from("https://w3id.org/did/v1"),
        id: Subject::from("did:iota:com:123456789abcdefghij"),
        ..Default::default()
    }
    .init()
    .init_timestamps()?;
    let did_payload = Payload::DIDDocument(old.clone());
    let tail_transaction = tangle_writer.publish_document(&did_payload).await?;
    println!(
        "DID document published: https://comnet.thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
    );

    // updated doc and publish diff
    let mut new = old.clone();
    let public_key = PublicKey::new(
        "did:iota:123456789abcdefghij#keys-1".into(),
        "RsaVerificationKey2018".into(),
        "did:iota:com:123456789abcdefghij".into(),
        "publicKeyBase58".into(),
        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
    )
    .unwrap();
    new.update_public_key(public_key);
    new.update_time();
    // diff the two docs and create a json string of the diff.
    let json_diff = serde_json::to_string(&Diff::serializable(&old, &new)).unwrap();
    let did_payload = Payload::DIDDocumentDifferences(Differences {
        did: new.derive_did()?,
        diff: json_diff.clone(),
        time: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true).to_string(),
    });
    let tail_transaction = tangle_writer.publish_document(&did_payload).await?;
    println!(
        "DID document difference published: https://comnet.thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
    );

    let resolver = Resolver::new(NetworkNodes::Com(nodes));
    // Mainnet: did:iota:123456789abcdefghij
    // Comnet: did:iota:com:123456789abcdefghij
    // Devnet: did:iota:dev:123456789abcdefghij
    let did = DID::parse_from_str("did:iota:com:123456789abcdefghij")?;
    let resolution_result = resolver.resolve(did, ResolutionInputMetadata::default()).await?;
    println!("{:#?}", resolution_result);
    println!("Document: {:?}", resolution_result.did_document);

    let mut deserializer = serde_json::Deserializer::from_str(&json_diff);
    // apply the json string to the old document.
    Apply::apply(&mut deserializer, &mut old).unwrap();
    // check to see that the old and new docs cotain all of the same fields.
    assert_eq!(resolution_result.did_document.to_string(), old.to_string());
    Ok(())
}
