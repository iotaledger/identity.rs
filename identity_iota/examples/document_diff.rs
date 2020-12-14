//! cargo run --example document_diff
use identity_core::convert::AsJson as _;
use identity_iota::{
    client::{Client, ClientBuilder, Network, PublishDocumentResponse},
    crypto::KeyPair,
    did::{DIDDiff, IotaDocument},
    error::Result,
};
use std::{thread::sleep, time::Duration};

#[smol_potat::main]
async fn main() -> Result<()> {
    // TODO: Make configurable
    let network: Network = Network::Mainnet;
    let node: &str = network.node_url().as_str();

    #[rustfmt::skip]
    println!("Creating Identity Client using network({:?}) and node({})", network, node);
    println!();

    let client: Client = ClientBuilder::new().node(node).network(network).build()?;

    // Generate a new DID Document and public/private key pair.
    //
    // The generated document will have an authentication key with the tag `key-1`
    let (mut document, keypair): (IotaDocument, KeyPair) =
        IotaDocument::generate_ed25519("key-1", network.as_str(), None)?;

    // Sign the DID Document with the default verification method.
    //
    // The default verification method is the first authentication method in the
    // document.
    document.sign(keypair.secret())?;

    println!("DID Document (signed) > {:#}", document);
    println!();

    // SANITY CHECK: Ensure the signature is valid.
    assert!(dbg!(document.verify()).is_ok());

    // Use the client created above to publish the DID Document to the Tangle.
    let response: PublishDocumentResponse = client.publish_document(&document).send().await?;

    println!("DID Document Transaction > {}", client.transaction_url(&response.tail));
    println!();

    let message_id: String = client.transaction_hash(&response.tail);

    // =========================================================================
    // DIFF
    // =========================================================================

    // Wait a bit so the timestamp changes
    sleep(Duration::from_secs(2));

    let mut update: IotaDocument = document.clone();
    update.properties_mut().insert("foo".into(), true.into());
    update.properties_mut().insert("bar".into(), vec![1, 2, 3].into());
    update.set_updated_now();

    let diff: DIDDiff = document.diff(&update, keypair.secret(), message_id)?;

    println!("Document Diff (signed) > {:#?}", diff);
    println!();

    // SANITY CHECK: Ensure the signature is valid.
    assert!(dbg!(document.verify_data(&diff)).is_ok());

    // TODO: Publish and Read Diff

    println!("JSON > {}", diff.to_json_pretty()?);
    println!();

    println!("Doc (old) > {:#}", document);
    println!();

    println!("Doc (new) > {:#}", update);
    println!();

    document.merge(&diff)?;

    println!("Doc (merged) > {:#}", document);
    println!();

    assert_eq!(document, update);

    Ok(())
}
