//!
//! cargo run --example credential
use identity_core::{
    common::{AsJson as _, Context, Timestamp},
    did::DID,
    key::PublicKey,
    object,
    vc::{Credential, CredentialBuilder, CredentialSubject, CredentialSubjectBuilder},
};
use identity_crypto::KeyPair;
use identity_iota::{
    client::{Client, ClientBuilder, CreateDocumentResponse, TransactionPrinter},
    did::{IotaDID, IotaDocument},
    error::Result,
    helpers::create_ed25519_key,
    network::Network,
};
use identity_proof::{HasProof, LdSignature, SignatureOptions};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifiableCredential {
    #[serde(flatten)]
    credential: Credential,
    proof: LdSignature,
}

impl HasProof for VerifiableCredential {
    fn proof(&self) -> &LdSignature {
        &self.proof
    }

    fn proof_mut(&mut self) -> &mut LdSignature {
        &mut self.proof
    }
}

#[derive(Debug)]
struct User {
    doc: IotaDocument,
    key: KeyPair,
    name: String,
}

impl User {
    async fn new(name: impl Into<String>, client: &Client) -> Result<Self> {
        // Generate a DID/keypair
        let (mut did, key): (IotaDID, KeyPair) = IotaDID::generate_ed25519()?;

        did.id_segments.insert(0, "com".into());

        // Create a signed DID document
        let pkey: PublicKey = create_ed25519_key(&did, key.public().as_ref())?;
        let mut doc: IotaDocument = IotaDocument::new(did, pkey)?;

        doc.sign(key.secret())?;

        // Publish the document
        let response: CreateDocumentResponse = client
            .create_document(&doc)
            .trace(true)
            .send()
            .await?;

        let printer = TransactionPrinter::hash(&response.tail);

        println!("[+] Doc > {:#}", doc);
        println!("[+]   https://comnet.thetangle.org/transaction/{}", printer);
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

        let credential: Credential = CredentialBuilder::new()
            .id("http://example.edu/credentials/3732")
            .issuer(DID::from(self.doc.did().clone()))
            .context(vec![Context::from(Credential::BASE_CONTEXT)])
            .types(vec![Credential::BASE_TYPE.into(), "UniversityDegreeCredential".into()])
            .subject(vec![subject])
            .issuance_date(Timestamp::now())
            .build()
            .unwrap();

        let mut credential: VerifiableCredential = VerifiableCredential {
            credential,
            proof: LdSignature::new("", SignatureOptions::new("")),
        };

        self.doc.sign_data(&mut credential, self.key.secret())?;

        Ok(credential)
    }
}


#[smol_potat::main]
async fn main() -> Result<()> {
    let client: Client = ClientBuilder::new()
        .network(Network::Comnet)
        .node("https://nodes.comnet.thetangle.org:443")
        .build()?;

    let issuer: User = User::new("Issuer", &client).await?;
    let subject: User = User::new("Subject", &client).await?;
    let credential: VerifiableCredential = issuer.issue(&subject)?;

    let json: String = credential.to_json_pretty()?;

    println!("[+] Credential > {}", json);
    println!("[+]");

    // ====================
    // ====================
    //
    // Exchange DIDs/Credentials
    //
    // ====================
    // ====================

    let issuer_did: IotaDID = issuer.doc.did().to_string().parse()?;
    let issuer_doc: IotaDocument = client.read_document(&issuer_did).send().await?.document;

    println!("[+] Issuer Doc (resolved) > {:#}", issuer_doc);
    println!("[+]");

    let subject_did: IotaDID = subject.doc.did().to_string().parse()?;
    let subject_doc: IotaDocument = client.read_document(&subject_did).send().await?.document;

    println!("[+] Subject Doc (resolved) > {:#}", subject_doc);
    println!("[+]");

    let credential: VerifiableCredential = VerifiableCredential::from_json(&json)?;

    println!(
        "[+] Credential (valid?) > {:#?}",
        issuer_doc.verify_data(&credential).is_ok()
    );
    println!("[+]");

    Ok(())
}
