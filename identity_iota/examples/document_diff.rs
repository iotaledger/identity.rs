//! cargo run --example document_diff
use identity_core::{
    did_doc::{MethodBuilder, MethodData, MethodRef, MethodType},
    proof::JcsEd25519Signature2020,
};
use identity_iota::{
    client::{Client, ClientBuilder, Network},
    crypto::KeyPair,
    did::{DocumentDiff, IotaDID, IotaDocument},
    error::Result,
};
use std::{thread::sleep, time::Duration};

#[smol_potat::main]
async fn main() -> Result<()> {
    let client: Client = ClientBuilder::new().network(Network::Comnet).build()?;
    let network: &str = client.network().as_str();

    let mut keys: Vec<KeyPair> = Vec::new();
    let mut diffs: Vec<String> = Vec::new();
    let did: IotaDID;

    // =========================================================================
    // Publish Initial Document
    // =========================================================================

    {
        let (mut document, keypair): (IotaDocument, KeyPair) = IotaDocument::builder().did_network(network).build()?;

        document.sign(keypair.secret())?;
        document.publish_with_client(&client).await?;

        println!("Doc (1) > {:#?}", document);
        println!();

        did = document.id().clone();
        keys.push(keypair);
    }

    // =========================================================================
    // Publish Auth Chain Update
    // =========================================================================

    sleep(Duration::from_secs(1));

    {
        let old: IotaDocument = client.read_document(&did).await?;
        let mut new: IotaDocument = old.clone();
        let keypair: KeyPair = JcsEd25519Signature2020::new_keypair();

        let authentication: MethodRef = MethodBuilder::default()
            .id(did.join("#key-2")?.into())
            .controller(did.clone().into())
            .key_type(MethodType::Ed25519VerificationKey2018)
            .key_data(MethodData::new_b58(keypair.public()))
            .build()
            .map(Into::into)?;

        unsafe {
            new.as_document_mut().authentication_mut().clear();
            new.as_document_mut().authentication_mut().append(authentication.into());
        }

        new.set_updated_now();
        new.set_previous_message_id(old.message_id().unwrap());

        old.sign_data(&mut new, keys[0].secret())?;
        new.publish_with_client(&client).await?;
        keys.push(keypair);

        println!("Doc (2) > {:#?}", new);
        println!();
    }

    // =========================================================================
    // Publish Diff Chain Update
    // =========================================================================

    sleep(Duration::from_secs(1));

    {
        let old: IotaDocument = client.read_document(&did).await?;

        let new: IotaDocument = {
            let mut this: IotaDocument = old.clone();
            this.properties_mut().insert("foo".into(), 123.into());
            this.properties_mut().insert("bar".into(), 456.into());
            this.set_updated_now();
            this
        };

        let prev_msg: String = old.message_id().unwrap().into();
        let mut diff: DocumentDiff = old.diff(&new, keys[1].secret(), prev_msg)?;

        diff.publish_with_client(&client, old.message_id().unwrap()).await?;
        diffs.push(diff.message_id().unwrap().to_string());

        println!("Doc (3) > {:#?}", new);
        println!();
    }

    // =========================================================================
    // Publish Second Diff Chain Update
    // =========================================================================

    sleep(Duration::from_secs(1));

    {
        let old: IotaDocument = client.read_document(&did).await?;

        let new: IotaDocument = {
            let mut this: IotaDocument = old.clone();
            this.properties_mut().insert("baz".into(), 789.into());
            this.properties_mut().remove("bar");
            this.set_updated_now();
            this
        };

        let prev_msg: String = diffs[0].clone();
        let mut diff: DocumentDiff = old.diff(&new, keys[1].secret(), prev_msg)?;

        diff.publish_with_client(&client, old.message_id().unwrap()).await?;
        diffs.push(diff.message_id().unwrap().to_string());

        println!("Doc (4) > {:#?}", new);
        println!();
    }

    // =========================================================================
    // Publish Phony Auth Update
    // =========================================================================

    {
        let old: IotaDocument = client.read_document(&did).await?;
        let mut new: IotaDocument = old.clone();
        let keypair: KeyPair = JcsEd25519Signature2020::new_keypair();

        let authentication: MethodRef = MethodBuilder::default()
            .id(did.join("#bad-key")?.into())
            .controller(did.clone().into())
            .key_type(MethodType::Ed25519VerificationKey2018)
            .key_data(MethodData::new_b58(keypair.public()))
            .build()
            .map(Into::into)?;

        unsafe {
            new.as_document_mut().authentication_mut().clear();
            new.as_document_mut().authentication_mut().append(authentication.into());
        }

        new.set_updated_now();
        new.set_previous_message_id(old.message_id().unwrap());

        new.sign(keypair.secret())?;
        new.publish_with_client(&client).await?;

        println!("Doc (phony) > {:#?}", new);
        println!();
    }

    // =========================================================================
    // Read Document Chain
    // =========================================================================

    println!("{:#?}", client.read_document_chain(&did).await?);
    println!();

    Ok(())
}
