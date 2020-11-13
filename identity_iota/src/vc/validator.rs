use identity_core::common::{FromJson as _, Object, SerdeInto as _};
use serde::Serialize;
use std::collections::BTreeMap;

use crate::{
    client::{Client, ReadDocumentResponse},
    did::{IotaDID, IotaDocument},
    error::Result,
    vc::{VerifiableCredential, VerifiablePresentation},
};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CredentialValidation {
    pub credential: VerifiableCredential,
    pub verified: bool,
    pub issuer: DocumentValidation,
    pub subjects: BTreeMap<String, DocumentValidation>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PresentationValidation {
    pub presentation: VerifiablePresentation,
    pub verified: bool,
    pub holder: DocumentValidation,
    pub credentials: Vec<CredentialValidation>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
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
        self.validate_credential(VerifiableCredential::from_json(data)?).await
    }

    pub async fn check_presentation<T>(&self, data: &T) -> Result<PresentationValidation>
    where
        T: AsRef<str> + ?Sized,
    {
        self.validate_presentation(VerifiablePresentation::from_json(data)?)
            .await
    }

    async fn validate_credential(&self, credential: VerifiableCredential) -> Result<CredentialValidation> {
        let issuer: DocumentValidation = self.validate_document(credential.issuer.url().as_str()).await?;
        let verified: bool = credential.verify(&issuer.document).is_ok();

        let mut subjects: BTreeMap<String, DocumentValidation> = BTreeMap::new();

        for subject in credential.credential_subject.iter() {
            if let Some(id) = subject.id.as_ref() {
                subjects.insert(id.to_string(), self.validate_document(id.as_str()).await?);
            }
        }

        Ok(CredentialValidation {
            credential,
            verified,
            issuer,
            subjects,
        })
    }

    async fn validate_presentation(&self, presentation: VerifiablePresentation) -> Result<PresentationValidation> {
        let holder: &str = presentation
            .holder
            .as_ref()
            .map(|holder| holder.as_str())
            .ok_or_else(|| identity_core::Error::InvalidPresentation("Presentation missing `holder`".into()))?;

        let holder: DocumentValidation = self.validate_document(holder).await?;
        let verified: bool = presentation.verify(&holder.document).is_ok();

        let mut credentials: Vec<CredentialValidation> = Vec::new();

        for credential in presentation.verifiable_credential.iter() {
            credentials.push(self.validate_credential(credential.serde_into()?).await?);
        }

        Ok(PresentationValidation {
            presentation,
            verified,
            holder,
            credentials,
        })
    }

    async fn validate_document(&self, did: &str) -> Result<DocumentValidation> {
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
