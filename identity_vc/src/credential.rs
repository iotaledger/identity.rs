use identity_core::common::{Object, OneOrMany, Timestamp, Uri, Value};
use serde_json::{from_str, to_string};

use crate::{
    common::{
        Context, CredentialSchema, CredentialStatus, CredentialSubject, Evidence, Issuer, RefreshService, TermsOfUse,
    },
    error::{Error, Result},
    utils::validate_credential_structure,
    verifiable::VerifiableCredential,
};

/// A `Credential` represents a set of claims describing an entity.
///
/// `Credential`s can be combined with `Proof`s to create `VerifiableCredential`s.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Credential {
    /// A set of URIs or `Object`s describing the applicable JSON-LD contexts.
    ///
    /// NOTE: The first URI MUST be `https://www.w3.org/2018/credentials/v1`
    #[serde(rename = "@context")]
    pub context: OneOrMany<Context>,
    /// A unique `URI` referencing the subject of the credential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uri>,
    /// One or more URIs defining the type of credential.
    ///
    /// NOTE: The VC spec defines this as a set of URIs BUT they are commonly
    /// passed as non-`URI` strings and expected to be processed with JSON-LD.
    /// We're using a `String` here since we don't currently use JSON-LD and
    /// don't have any immediate plans to do so.
    #[serde(rename = "type")]
    pub types: OneOrMany<String>,
    /// One or more `Object`s representing the `Credential` subject(s).
    #[serde(rename = "credentialSubject")]
    pub credential_subject: OneOrMany<CredentialSubject>,
    /// A reference to the issuer of the `Credential`.
    pub issuer: Issuer,
    /// The date and time the `Credential` becomes valid.
    #[serde(rename = "issuanceDate")]
    pub issuance_date: Timestamp,
    /// The date and time the `Credential` is no longer considered valid.
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<Timestamp>,
    /// TODO
    #[serde(rename = "credentialStatus", skip_serializing_if = "Option::is_none")]
    pub credential_status: Option<OneOrMany<CredentialStatus>>,
    /// TODO
    #[serde(rename = "credentialSchema", skip_serializing_if = "Option::is_none")]
    pub credential_schema: Option<OneOrMany<CredentialSchema>>,
    /// TODO
    #[serde(rename = "refreshService", skip_serializing_if = "Option::is_none")]
    pub refresh_service: Option<OneOrMany<RefreshService>>,
    /// The terms of use issued by the `Credential` issuer
    #[serde(rename = "termsOfUse", skip_serializing_if = "Option::is_none")]
    pub terms_of_use: Option<OneOrMany<TermsOfUse>>,
    /// TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<OneOrMany<Evidence>>,
    /// Indicates that the `Credential` must only be contained within a
    /// `Presentation` with a proof issued from the `Credential` subject.
    #[serde(rename = "nonTransferable", skip_serializing_if = "Option::is_none")]
    pub non_transferable: Option<Value>,
    /// Miscellaneous properties.
    #[serde(flatten)]
    pub properties: Object,
}

impl Credential {
    pub const BASE_CONTEXT: &'static str = "https://www.w3.org/2018/credentials/v1";

    pub const BASE_TYPE: &'static str = "VerifiableCredential";

    pub fn validate(&self) -> Result<()> {
        validate_credential_structure(self)
    }

    pub fn from_json(json: &(impl AsRef<str> + ?Sized)) -> Result<Self> {
        from_str(json.as_ref()).map_err(Error::DecodeJSON)
    }

    pub fn to_json(&self) -> Result<String> {
        to_string(self).map_err(Error::EncodeJSON)
    }
}

// =============================================================================
// Credential Builder
// =============================================================================

/// A convenience for constructing a `Credential` or `VerifiableCredential`
/// from dynamic data.
///
/// NOTE: Base context and type are automatically included.
#[derive(Debug)]
pub struct CredentialBuilder {
    context: Vec<Context>,
    id: Option<Uri>,
    types: Vec<String>,
    credential_subject: Vec<CredentialSubject>,
    issuer: Option<Issuer>,
    issuance_date: Timestamp,
    expiration_date: Option<Timestamp>,
    credential_status: Vec<CredentialStatus>,
    credential_schema: Vec<CredentialSchema>,
    refresh_service: Vec<RefreshService>,
    terms_of_use: Vec<TermsOfUse>,
    evidence: Vec<Evidence>,
    non_transferable: Option<Value>,
    properties: Object,
}

impl CredentialBuilder {
    pub fn new() -> Self {
        Self {
            context: vec![Credential::BASE_CONTEXT.into()],
            id: None,
            types: vec![Credential::BASE_TYPE.into()],
            credential_subject: Vec::new(),
            issuer: None,
            issuance_date: Default::default(),
            expiration_date: None,
            credential_status: Vec::new(),
            credential_schema: Vec::new(),
            refresh_service: Vec::new(),
            terms_of_use: Vec::new(),
            evidence: Vec::new(),
            non_transferable: None,
            properties: Default::default(),
        }
    }

    pub fn context(mut self, value: impl Into<Context>) -> Self {
        let value: Context = value.into();

        if !matches!(value, Context::Uri(ref uri) if uri == Credential::BASE_CONTEXT) {
            self.context.push(value);
        }

        self
    }

    pub fn type_(mut self, value: impl Into<String>) -> Self {
        let value: String = value.into();

        if value != Credential::BASE_TYPE {
            self.types.push(value);
        }

        self
    }

    pub fn property(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }

    impl_builder_setter!(id, id, Option<Uri>);
    impl_builder_setter!(subject, credential_subject, Vec<CredentialSubject>);
    impl_builder_setter!(issuer, issuer, Option<Issuer>);
    impl_builder_setter!(issuance_date, issuance_date, Timestamp);
    impl_builder_setter!(expiration_date, expiration_date, Option<Timestamp>);
    impl_builder_setter!(status, credential_status, Vec<CredentialStatus>);
    impl_builder_setter!(schema, credential_schema, Vec<CredentialSchema>);
    impl_builder_setter!(refresh, refresh_service, Vec<RefreshService>);
    impl_builder_setter!(terms_of_use, terms_of_use, Vec<TermsOfUse>);
    impl_builder_setter!(evidence, evidence, Vec<Evidence>);
    impl_builder_setter!(non_transferable, non_transferable, Option<Value>);
    impl_builder_setter!(properties, properties, Object);

    impl_builder_try_setter!(try_subject, credential_subject, Vec<CredentialSubject>);
    impl_builder_try_setter!(try_issuance_date, issuance_date, Timestamp);
    impl_builder_try_setter!(try_expiration_date, expiration_date, Option<Timestamp>);
    impl_builder_try_setter!(try_status, credential_status, Vec<CredentialStatus>);
    impl_builder_try_setter!(try_schema, credential_schema, Vec<CredentialSchema>);
    impl_builder_try_setter!(try_refresh_service, refresh_service, Vec<RefreshService>);
    impl_builder_try_setter!(try_terms_of_use, terms_of_use, Vec<TermsOfUse>);
    impl_builder_try_setter!(try_evidence, evidence, Vec<Evidence>);

    /// Consumes the `CredentialBuilder`, returning a valid `Credential`
    pub fn build(self) -> Result<Credential> {
        let mut credential: Credential = Credential {
            context: self.context.into(),
            id: self.id,
            types: self.types.into(),
            credential_subject: self.credential_subject.into(),
            issuer: self.issuer.ok_or_else(|| Error::MissingCredentialIssuer)?,
            issuance_date: self.issuance_date,
            expiration_date: self.expiration_date,
            credential_status: None,
            credential_schema: None,
            refresh_service: None,
            terms_of_use: None,
            evidence: None,
            non_transferable: self.non_transferable,
            properties: self.properties,
        };

        if !self.credential_status.is_empty() {
            credential.credential_status = Some(self.credential_status.into());
        }

        if !self.credential_schema.is_empty() {
            credential.credential_schema = Some(self.credential_schema.into());
        }

        if !self.refresh_service.is_empty() {
            credential.refresh_service = Some(self.refresh_service.into());
        }

        if !self.terms_of_use.is_empty() {
            credential.terms_of_use = Some(self.terms_of_use.into());
        }

        if !self.evidence.is_empty() {
            credential.evidence = Some(self.evidence.into());
        }

        credential.validate()?;

        Ok(credential)
    }

    /// Consumes the `CredentialBuilder`, returning a valid `VerifiableCredential`
    pub fn build_verifiable(self, proof: impl Into<OneOrMany<Object>>) -> Result<VerifiableCredential> {
        self.build()
            .map(|credential| VerifiableCredential::new(credential, proof))
    }
}

impl Default for CredentialBuilder {
    fn default() -> Self {
        Self::new()
    }
}
