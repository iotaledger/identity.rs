use identity_core::{
    common::{FromJson as _, Object},
    vc::Credential,
};
use identity_proof::LdSignature;
use std::collections::BTreeMap;

use crate::{
    client::{Client, ReadDocumentResponse},
    did::{IotaDID, IotaDocument},
    error::{Error, Result},
    vc::VerifiableCredential,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CredentialValidation {
    pub credential: VerifiableCredential,
    pub verified: bool,
    pub issuer: DocumentValidation,
    pub subjects: BTreeMap<String, DocumentValidation>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DocumentValidation {
    pub did: IotaDID,
    pub document: IotaDocument,
    pub metadata: Object,
    pub verified: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct CredentialValidator<'a> {
    client: &'a Client,
}

impl<'a> CredentialValidator<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn check<T>(&self, data: &T) -> Result<CredentialValidation>
    where
        T: AsRef<str> + ?Sized,
    {
        let mut credential: Credential = Credential::from_json(data)?;
        let mut subjects: BTreeMap<String, DocumentValidation> = BTreeMap::new();

        let issuer: DocumentValidation = self.document(credential.issuer.url().as_str()).await?;

        let proof: LdSignature = credential
            .properties
            .remove("proof")
            .map(LdSignature::from_json_value)
            .transpose()?
            .ok_or(Error::InvalidProof)?;

        let credential: VerifiableCredential = VerifiableCredential::with_proof(credential, proof);
        let verified: bool = credential.verify(&issuer.document).is_ok();

        if verified {
            for subject in credential.credential_subject.iter() {
                if let Some(id) = subject.id.as_ref() {
                    subjects.insert(id.to_string(), self.document(id.as_str()).await?);
                }
            }
        }

        Ok(CredentialValidation {
            credential,
            verified,
            issuer,
            subjects,
        })
    }

    async fn document(&self, did: &str) -> Result<DocumentValidation> {
        let did: IotaDID = did.parse()?;
        let doc: ReadDocumentResponse = self.client.read_document(&did).send().await?;
        let verified: bool = doc.document.verify().is_ok();

        Ok(DocumentValidation {
            did,
            document: doc.document,
            metadata: doc.metadata,
            verified,
        })
    }
}
