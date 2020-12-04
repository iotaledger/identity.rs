//! A basic example that generates a DID Document, publishes it to the Tangle,
//! and retrieves information through DID Document resolution/dereferencing.
//!
//! cargo run --example document
use identity_core::resolver::{dereference, resolve, Dereference, Resolution};
use identity_iota::{
    client::Client,
    crypto::KeyPair,
    did::{IotaDID, IotaDocument},
    error::Result,
};

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

    let did: &IotaDID = document.id();

    // Resolve the DID and retrieve the published DID Document from the Tangle.
    //
    // Note: Uses Default values for InputMetadata.
    let resolution: Resolution = resolve(did.as_str(), Default::default(), &client).await?;

    println!("DID Document Resolution > {:#?}", resolution);
    println!();

    // Dereference the DID and retrieve the verification method generated above.
    let did: IotaDID = did.join("#key-1")?;
    let dereference: Dereference = dereference(did.as_str(), Default::default(), &client).await?;

    println!("DID Document Dereference > {:#?}", dereference);
    println!();

    Ok(())
}
