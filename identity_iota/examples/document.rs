//! A basic example that generates a DID Document, publishes it to the Tangle,
//! and retrieves information through DID Document resolution/dereferencing.
//!
//! cargo run --example document
use identity_core::resolver::{dereference, resolve, Dereference, Resolution};
use identity_iota::{
    client::{Client, ClientBuilder, Network, PublishDocumentResponse},
    crypto::KeyPair,
    did::{IotaDID, IotaDocument},
    error::Result,
};

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
