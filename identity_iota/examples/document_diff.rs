//! cargo run --example document_diff
use identity_core::resolver::{resolve, Dereference, Resolution};
use identity_iota::{
    client::Client,
    crypto::KeyPair,
    did::{DocumentDiff, IotaDocument},
    error::Result,
};
use std::{thread::sleep, time::Duration};

#[smol_potat::main]
async fn main() -> Result<()> {
    let client: Client = Client::new()?;

    // Generate a new DID Document and public/private key pair.
    //
    // The generated document will have an authentication key with the tag
    // `authentication`
    let (mut document, keypair): (IotaDocument, KeyPair) =
        IotaDocument::builder().did_network(client.network().as_str()).build()?;

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
    document.publish_with_client(&client).await?;

    // =========================================================================
    // DIFF
    // =========================================================================

    // Wait a bit so the timestamp changes
    sleep(Duration::from_secs(2));

    let mut update: IotaDocument = document.clone();
    update.properties_mut().insert("foo".into(), true.into());
    update.properties_mut().insert("bar".into(), vec![1, 2, 3].into());
    update.set_updated_now();

    let previous_message_id: &str = document.message_id().unwrap();
    let mut diff: DocumentDiff = document.diff(&update, keypair.secret(), previous_message_id.into())?;

    println!("Document Diff (signed) > {:#?}", diff);
    println!();

    // SANITY CHECK: Ensure the signature is valid.
    assert!(dbg!(document.verify_data(&diff)).is_ok());

    // Publish at the diff address for auth document #1
    diff.publish_with_client(&client, 1).await?;

    // Resolve the final DID document with all verified diffs merged.
    let resolution: Resolution = resolve(diff.did().as_str(), Default::default(), &client).await?;

    println!("DID Document Resolution > {:#?}", resolution);
    println!();

    Ok(())
}
