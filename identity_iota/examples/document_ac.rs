//! A basic example that generates and publishes a new DID document, publishes
//! a replacement document, and retrieves the updated document through DID
//! Document resolution.
//!
//! cargo run --example document_ac
use identity_core::{
    did_doc::{Method, MethodBuilder, MethodData, MethodRef, MethodType},
    proof::JcsEd25519Signature2020,
};
use identity_iota::{client::Client, crypto::KeyPair, did::IotaDocument, error::Result};
use std::{thread::sleep, time::Duration};

#[smol_potat::main]
async fn main() -> Result<()> {
    let client: Client = Client::new()?;

    // Generate a new DID Document and public/private key pair.
    //
    // The generated document will have a verification method with the tag
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
    updated.set_previous_message_id(document.message_id().unwrap());

    // Sign the updated document with the *previous* authentication method
    document.sign_data(&mut updated, keypair.secret())?;

    println!("New Document (signed) > {:#}", updated);
    println!();

    // SANITY CHECK: Ensure the signature is valid.
    assert!(dbg!(document.verify_data(&updated)).is_ok());

    // Publish the updated document.
    updated.publish_with_client(&client).await?;

    // Read the published DID document from the Tangle.
    let response: IotaDocument = client.read_document(document.id()).await?;

    println!("Document Response > {:#?}", response);
    println!();

    Ok(())
}
