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
    // The generated document will have an authentication key associated with
    // the keypair.
    let (mut document, keypair): (IotaDocument, KeyPair) = IotaDocument::builder()
        .authentication_tag("key-1")
        .did_network(client.network().as_str())
        .build()?;

    // Sign the DID Document with the default authentication key.
    document.sign(keypair.secret())?;

    println!("DID Document (signed) > {:#}", document);
    println!();

    // Use the client created above to publish the DID Document to the Tangle.
    document.publish_with_client(&client).await?;

    let did: &IotaDID = document.id();

    // Resolve the DID and retrieve the published DID Document from the Tangle.
    let resolution: Resolution = resolve(did.as_str(), Default::default(), &client).await?;

    println!("DID Document Resolution > {:#?}", resolution);
    println!();

    // Dereference the DID and retrieve the authentication method generated above.
    let did: IotaDID = did.join("#key-1")?;
    let dereference: Dereference = dereference(did.as_str(), Default::default(), &client).await?;

    println!("DID Document Dereference > {:#?}", dereference);
    println!();

    Ok(())
}
