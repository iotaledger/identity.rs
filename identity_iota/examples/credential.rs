//!
//! cargo run --example credential
use identity_core::{
    common::{AsJson as _, Context, Timestamp},
    did::DID,
    object,
    vc::{Credential, CredentialBuilder, CredentialSubject, CredentialSubjectBuilder},
};
use identity_crypto::KeyPair;
use identity_iota::{
    client::{Client, ClientBuilder, PublishDocumentResponse},
    did::IotaDocument,
    error::Result,
    network::Network,
    vc::{CredentialValidation, CredentialValidator, VerifiableCredential},
};

#[derive(Debug)]
struct User {
    doc: IotaDocument,
    key: KeyPair,
    name: String,
}

impl User {
    async fn new(name: impl Into<String>, client: &Client) -> Result<Self> {
        // Create a DID document with a generated DID/authentication key
        let (mut doc, key): (IotaDocument, KeyPair) = IotaDocument::generate_ed25519("key-1", None)?;

        // Sign the document
        doc.sign(key.secret())?;

        // Publish the document
        let response: PublishDocumentResponse = client.create_document(&doc).trace(true).send().await?;

        println!("[+] Doc > {:#}", doc);
        println!("[+]   {}", client.transaction_url(&response.tail));
        println!("[+]");

        Ok(Self {
            doc,
            key,
            name: name.into(),
        })
    }

    fn issue(&self, user: &User) -> Result<VerifiableCredential> {
        let subject: CredentialSubject = CredentialSubjectBuilder::default()
            .id(DID::from(user.doc.did().clone()))
            .properties(object!(
                name: user.name.clone(),
                degree:
                    object!(
                      name: "Bachelor of Science and Arts",
                      type: "BachelorDegree",
                    )
            ))
            .build()
            .unwrap();

        let mut credential: VerifiableCredential = CredentialBuilder::new()
            .id("http://example.edu/credentials/3732")
            .issuer(DID::from(self.doc.did().clone()))
            .context(vec![Context::from(Credential::BASE_CONTEXT)])
            .types(vec![Credential::BASE_TYPE.into(), "UniversityDegreeCredential".into()])
            .subject(vec![subject])
            .issuance_date(Timestamp::now())
            .build()
            .map(VerifiableCredential::new)
            .unwrap();

        credential.sign(&self.doc, self.key.secret())?;

        Ok(credential)
    }
}

#[smol_potat::main]
async fn main() -> Result<()> {
    let client: Client = ClientBuilder::new()
        .network(Network::Mainnet)
        .node("https://nodes.thetangle.org:443")
        .build()?;

    let issuer: User = User::new("Issuer", &client).await?;
    let subject: User = User::new("Subject", &client).await?;
    let vc: VerifiableCredential = issuer.issue(&subject)?;

    let json: String = vc.to_json_pretty()?;

    println!("[+] Credential > {}", json);
    println!("[+]");

    // ====================
    // ====================
    //
    // Exchange DIDs/Credentials
    //
    // ====================
    // ====================

    let validator: CredentialValidator<'_> = CredentialValidator::new(&client);
    let validation: CredentialValidation = validator.check(&json).await?;

    println!("[+] Credential Validation > {:#?}", validation);
    println!("[+]");

    Ok(())
}
