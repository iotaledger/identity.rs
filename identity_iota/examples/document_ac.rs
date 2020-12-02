//! A basic example that generates and publishes a new DID document, publishes
//! a replacement document, and retrieves the updated document through DID
//! Document resolution.
//!
//! cargo run --example document_ac
use identity_core::{
    did_doc::{Method, MethodBuilder, MethodData, MethodRef, MethodType},
    proof::JcsEd25519Signature2020,
};
use identity_iota::{
    client::{Client, ClientBuilder, Network, PublishDocumentResponse, ReadDocumentResponse},
    crypto::KeyPair,
    did::IotaDocument,
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
    // AUTH CHAIN
    // =========================================================================

    // Wait a bit so the timestamp changes
    sleep(Duration::from_secs(2));

    let mut updated: IotaDocument = document.clone();

    let key1: KeyPair = JcsEd25519Signature2020::new_keypair();
    let key2: KeyPair = JcsEd25519Signature2020::new_keypair();

    let authentication: Method = MethodBuilder::default()
        .id((**document.id()).join("#key-2")?)
        .controller(document.id().clone().into())
        .key_type(MethodType::Ed25519VerificationKey2018)
        .key_data(MethodData::new_b58(key1.public()))
        .build()?;

    let authentication_ref: MethodRef = authentication.id().clone().into();

    let agreement: MethodRef = MethodBuilder::default()
        .id((**document.id()).join("#key-3")?)
        .controller(document.id().clone().into())
        .key_type(MethodType::Ed25519VerificationKey2018)
        .key_data(MethodData::new_b58(key2.public()))
        .build()?
        .into();

    unsafe {
        let doc = updated.as_document_mut();
        doc.verification_method_mut().clear();
        doc.verification_method_mut().append(authentication.into());
        doc.authentication_mut().clear();
        doc.authentication_mut().append(authentication_ref.into());
        doc.key_agreement_mut().append(agreement.into());
    }

    updated.set_updated_now();
    updated.set_prev_msg(message_id);

    // Sign the updated document with the *previous* authentication method
    document.sign_data(&mut updated, keypair.secret())?;

    println!("New Document (signed) > {:#}", updated);
    println!();

    // SANITY CHECK: Ensure the signature is valid.
    assert!(dbg!(document.verify_data(&updated)).is_ok());

    // Publish the updated document.
    let response: PublishDocumentResponse = client.publish_document(&updated).send().await?;

    println!("New Document Transaction > {}", client.transaction_url(&response.tail));
    println!();

    let response: ReadDocumentResponse = client.read_document(document.id()).send().await?;

    println!("Document Response > {:#?}", response);
    println!();

    Ok(())
}
