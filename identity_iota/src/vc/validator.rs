use identity_core::{
    common::Object,
    convert::FromJson as _,
    error::Error,
    vc::{VerifiableCredential, VerifiablePresentation},
};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::BTreeMap;

use crate::{
    client::Client,
    did::{IotaDID, IotaDocument},
    error::Result,
};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CredentialValidation<T = Object> {
    pub credential: VerifiableCredential<T>,
    pub issuer: DocumentValidation,
    pub subjects: BTreeMap<String, DocumentValidation>,
    pub verified: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PresentationValidation<T = Object, U = Object> {
    pub presentation: VerifiablePresentation<T, U>,
    pub holder: DocumentValidation,
    pub credentials: Vec<CredentialValidation<U>>,
    pub verified: bool,
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
    /// Creates a new `CredentialValidator`.
    pub const fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Deserializes the given JSON-encoded `VerifiableCredential` and validates
    /// all associated DID documents.
    pub async fn check<T>(&self, data: &str) -> Result<CredentialValidation<T>>
    where
        T: DeserializeOwned + Serialize,
    {
        self.validate_credential(VerifiableCredential::from_json(data)?).await
    }

    /// Deserializes the given JSON-encoded `VerifiablePresentation` and
    /// validates all associated DID documents/`VerifiableCredential`s.
    pub async fn check_presentation<T, U>(&self, data: &str) -> Result<PresentationValidation<T, U>>
    where
        T: Clone + DeserializeOwned + Serialize,
        U: Clone + DeserializeOwned + Serialize,
    {
        self.validate_presentation(VerifiablePresentation::from_json(data)?)
            .await
    }

    /// Validates the `VerifiableCredential` proof and all relevant DID documents.
    ///
    /// Note: The credential is expected to have a proof created by the issuing party.
    /// Note: The credential issuer URL is expected to be a valid DID.
    /// Note: Credential subject IDs are expected to be valid DIDs (if present).
    pub async fn validate_credential<T>(&self, credential: VerifiableCredential<T>) -> Result<CredentialValidation<T>>
    where
        T: Serialize,
    {
        let issuer: DocumentValidation = self.validate_document(credential.issuer.url().as_str()).await?;
        let verified: bool = issuer.document.verify_data(&credential).is_ok();

        let mut subjects: BTreeMap<String, DocumentValidation> = BTreeMap::new();

        for subject in credential.credential_subject.iter() {
            if let Some(id) = subject.id.as_ref() {
                subjects.insert(id.to_string(), self.validate_document(id.as_str()).await?);
            }
        }

        Ok(CredentialValidation {
            credential,
            issuer,
            subjects,
            verified,
        })
    }

    /// Validates the `VerifiablePresentation` proof and all relevant DID documents.
    ///
    /// Note: The presentation holder is expected to be present and a valid DID.
    /// Note: The presentation is expected to have a proof created by the holder.
    pub async fn validate_presentation<T, U>(
        &self,
        presentation: VerifiablePresentation<T, U>,
    ) -> Result<PresentationValidation<T, U>>
    where
        T: Clone + Serialize,
        U: Clone + Serialize,
    {
        let holder: &str = presentation
            .holder
            .as_ref()
            .map(|holder| holder.as_str())
            .ok_or_else(|| Error::InvalidPresentation("Presentation missing `holder`".into()))?;

        let holder: DocumentValidation = self.validate_document(holder).await?;
        let verified: bool = holder.document.verify_data(&presentation).is_ok();

        let mut credentials: Vec<CredentialValidation<U>> = Vec::new();

        for credential in presentation.verifiable_credential.iter() {
            credentials.push(self.validate_credential(credential.clone()).await?);
        }

        Ok(PresentationValidation {
            presentation,
            holder,
            credentials,
            verified,
        })
    }

    async fn validate_document(&self, did: &str) -> Result<DocumentValidation> {
        let did: IotaDID = did.parse()?;
        let document: IotaDocument = self.client.read_document(&did).await?;
        let verified: bool = document.verify().is_ok();

        Ok(DocumentValidation {
            did,
            document,
            metadata: Object::new(),
            verified,
        })
    }
}
