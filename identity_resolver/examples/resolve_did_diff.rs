//! Publish new did document and read it from the tangle
//! cargo run --example resolve_did_diff
use anyhow::Result;
use identity_common::Timestamp;
use identity_core::{
    document::DIDDocument,
    utils::{Context, KeyData, PublicKey, Subject},
};
use identity_diff::Diff;
use identity_integration::tangle_writer::{iota_network, Differences, Payload, TangleWriter};
use identity_resolver::resolver::{NetworkNodes, ResolutionInputMetadata, Resolver};
use iota_conversion::Trinary;

#[smol_potat::main]
async fn main() -> Result<()> {
    let nodes = vec!["http://localhost:14265", "https://nodes.comnet.thetangle.org:443"];
    let tangle_writer = TangleWriter::new(nodes.clone(), iota_network::Comnet).await?;

    // Create and publish first document version
    let old = DIDDocument {
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
    let public_key = PublicKey {
        id: "did:iota:123456789abcdefghij#keys-1".into(),
        key_type: "RsaVerificationKey2018".into(),
        controller: "did:iota:com:123456789abcdefghij".into(),
        key_data: KeyData::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into()),
        ..Default::default()
    }
    .init();
    new.update_public_key(public_key);
    new.update_time();
    // diff the two docs.
    let diff = old.diff(&new)?;
    let did_payload = Payload::DIDDocumentDifferences(Differences {
        did: new.derive_did()?,
        diff: serde_json::to_string(&diff)?,
        time: Timestamp::now().to_rfc3339(),
    });
    let tail_transaction = tangle_writer.publish_document(&did_payload).await?;
    println!(
        "DID document difference published: https://comnet.thetangle.org/transaction/{}",
        tail_transaction.as_i8_slice().trytes().expect("Couldn't get Trytes")
    );

    let resolver = Resolver::new(NetworkNodes::Com(nodes))?;
    // Mainnet: did:iota:123456789abcdefghij
    // Comnet: did:iota:com:123456789abcdefghij
    // Devnet: did:iota:dev:123456789abcdefghij
    let resolution_result = resolver
        .resolve(
            "did:iota:com:123456789abcdefghij".into(),
            ResolutionInputMetadata::default(),
        )
        .await?;
    println!("{:#?}", resolution_result);
    println!("Document: {:?}", resolution_result.did_document);

    // merge the diff to the old document.
    let old = old.merge(diff)?;
    // check to see that the old and new docs cotain all of the same fields.
    assert_eq!(resolution_result.did_document.unwrap().to_string(), old.to_string());
    Ok(())
}
