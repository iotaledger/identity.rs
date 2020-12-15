//! cargo run --example document_diff
use identity_core::{
    did_doc::{MethodBuilder, MethodData, MethodRef, MethodType},
    proof::JcsEd25519Signature2020,
};
use identity_iota::{
    chain::{AuthChain, DocumentChain},
    client::{Client, ClientBuilder, Network},
    crypto::KeyPair,
    did::{DocumentDiff, IotaDocument},
    error::Result,
    tangle::MessageId,
};
use std::{thread::sleep, time::Duration};

#[smol_potat::main]
async fn main() -> Result<()> {
    let client: Client = ClientBuilder::new().network(Network::Comnet).build()?;
    let network: &str = client.network().as_str();

    // Keep track of the chain state locally, for reference
    let mut chain: DocumentChain;
    let mut keys: Vec<KeyPair> = Vec::new();

    // =========================================================================
    // Publish Initial Document
    // =========================================================================

    {
        let (mut document, keypair): (IotaDocument, KeyPair) = IotaDocument::builder().did_network(network).build()?;

        document.sign(keypair.secret())?;
        document.publish_with_client(&client).await?;

        chain = DocumentChain::new(AuthChain::new(document)?);
        keys.push(keypair);

        println!("Chain (1) > {:#?}", chain);
        println!();
    }

    // =========================================================================
    // Publish Auth Chain Update
    // =========================================================================

    sleep(Duration::from_secs(1));

    {
        let mut new: IotaDocument = chain.current().clone();
        let keypair: KeyPair = JcsEd25519Signature2020::new_keypair();

        let authentication: MethodRef = MethodBuilder::default()
            .id(chain.id().join("#key-2")?.into())
            .controller(chain.id().clone().into())
            .key_type(MethodType::Ed25519VerificationKey2018)
            .key_data(MethodData::new_b58(keypair.public()))
            .build()
            .map(Into::into)?;

        unsafe {
            new.as_document_mut().authentication_mut().clear();
            new.as_document_mut().authentication_mut().append(authentication.into());
        }

        new.set_updated_now();
        new.set_previous_message_id(chain.auth_message_id().clone());

        chain.current().sign_data(&mut new, keys[0].secret())?;
        new.publish_with_client(&client).await?;

        keys.push(keypair);
        chain.try_push_auth(new)?;

        println!("Chain (2) > {:#?}", chain);
        println!();
    }

    // =========================================================================
    // Publish Diff Chain Update
    // =========================================================================

    sleep(Duration::from_secs(1));

    {
        let new: IotaDocument = {
            let mut this: IotaDocument = chain.current().clone();
            this.properties_mut().insert("foo".into(), 123.into());
            this.properties_mut().insert("bar".into(), 456.into());
            this.set_updated_now();
            this
        };

        let message_id: MessageId = chain.diff_message_id().clone();
        let mut diff: DocumentDiff = chain.current().diff(&new, keys[1].secret(), message_id)?;

        diff.publish_with_client(&client, chain.auth_message_id()).await?;
        chain.try_push_diff(diff)?;

        println!("Chain (3) > {:#?}", chain);
        println!();
    }

    // =========================================================================
    // Publish Phony Auth Update
    // =========================================================================

    sleep(Duration::from_secs(1));

    {
        let mut new: IotaDocument = chain.current().clone();
        let keypair: KeyPair = JcsEd25519Signature2020::new_keypair();

        let authentication: MethodRef = MethodBuilder::default()
            .id(new.id().join("#bad-key")?.into())
            .controller(new.id().clone().into())
            .key_type(MethodType::Ed25519VerificationKey2018)
            .key_data(MethodData::new_b58(keypair.public()))
            .build()
            .map(Into::into)?;

        unsafe {
            new.as_document_mut().authentication_mut().clear();
            new.as_document_mut().authentication_mut().append(authentication.into());
        }

        new.set_updated_now();
        new.set_previous_message_id(chain.auth_message_id().clone());

        new.sign(keypair.secret())?;
        new.publish_with_client(&client).await?;

        println!("Chain Err > {:?}", chain.try_push_auth(new).unwrap_err());
    }

    // =========================================================================
    // Publish Second Diff Chain Update
    // =========================================================================

    sleep(Duration::from_secs(1));

    {
        let new: IotaDocument = {
            let mut this: IotaDocument = chain.current().clone();
            this.properties_mut().insert("baz".into(), 789.into());
            this.properties_mut().remove("bar");
            this.set_updated_now();
            this
        };

        let message_id: MessageId = chain.diff_message_id().clone();
        let mut diff: DocumentDiff = chain.current().diff(&new, keys[1].secret(), message_id)?;

        diff.publish_with_client(&client, chain.auth_message_id()).await?;
        chain.try_push_diff(diff)?;

        println!("Chain (4) > {:#?}", chain);
        println!();
    }

    // =========================================================================
    // Read Document Chain
    // =========================================================================

    let remote: DocumentChain = client.read_document_chain(chain.id()).await?;

    println!("Chain (R) {:#?}", remote);
    println!();

    let a: &IotaDocument = chain.current();
    let b: &IotaDocument = remote.current();

    // The current document in the resolved chain should be identical to the
    // current document in our local chain.
    assert_eq!(a, b);

    Ok(())
}
