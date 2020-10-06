use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    common::{Context, Object, OneOrMany, Timestamp, Uri, Value},
    vc::{
        validate_credential_structure, CredentialSchema, CredentialStatus, CredentialSubject, Error, Evidence, Issuer,
        RefreshService, Result, TermsOfUse, VerifiableCredential,
    },
};

/// A `Credential` represents a set of claims describing an entity.
///
/// `Credential`s can be combined with `Proof`s to create `VerifiableCredential`s.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Builder)]
#[builder(build_fn(name = "build_unchecked"), pattern = "owned")]
pub struct Credential {
    /// A set of URIs or `Object`s describing the applicable JSON-LD contexts.
    ///
    /// NOTE: The first URI MUST be `https://www.w3.org/2018/credentials/v1`
    #[serde(rename = "@context")]
    #[builder(setter(into))]
    pub context: OneOrMany<Context>,
    /// A unique `URI` referencing the subject of the credential.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<Uri>,
    /// One or more URIs defining the type of credential.
    ///
    /// NOTE: The VC spec defines this as a set of URIs BUT they are commonly
    /// passed as non-`URI` strings and expected to be processed with JSON-LD.
    /// We're using a `String` here since we don't currently use JSON-LD and
    /// don't have any immediate plans to do so.
    #[serde(rename = "type")]
    #[builder(setter(into, strip_option))]
    pub types: OneOrMany<String>,
    /// One or more `Object`s representing the `Credential` subject(s).
    #[serde(rename = "credentialSubject")]
    #[builder(default, setter(into, name = "subject"))]
    pub credential_subject: OneOrMany<CredentialSubject>,
    /// A reference to the issuer of the `Credential`.
    #[builder(setter(into))]
    pub issuer: Issuer,
    /// The date and time the `Credential` becomes valid.
    #[serde(rename = "issuanceDate")]
    #[builder(default, setter(into))]
    pub issuance_date: Timestamp,
    /// The date and time the `Credential` is no longer considered valid.
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub expiration_date: Option<Timestamp>,
    /// TODO
    #[serde(rename = "credentialStatus", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub credential_status: Option<OneOrMany<CredentialStatus>>,
    /// TODO
    #[serde(rename = "credentialSchema", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub credential_schema: Option<OneOrMany<CredentialSchema>>,
    /// TODO
    #[serde(rename = "refreshService", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub refresh_service: Option<OneOrMany<RefreshService>>,
    /// The terms of use issued by the `Credential` issuer
    #[serde(rename = "termsOfUse", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub terms_of_use: Option<OneOrMany<TermsOfUse>>,
    /// TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub evidence: Option<OneOrMany<Evidence>>,
    /// Indicates that the `Credential` must only be contained within a
    /// `Presentation` with a proof issued from the `Credential` subject.
    #[serde(rename = "nonTransferable", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub non_transferable: Option<Value>,
    /// Miscellaneous properties.
    #[serde(flatten)]
    #[builder(default, setter(into))]
    pub properties: Object,
}

impl Credential {
    pub const BASE_CONTEXT: &'static str = "https://www.w3.org/2018/credentials/v1";

    pub const BASE_TYPE: &'static str = "VerifiableCredential";

    pub fn validate(&self) -> Result<()> {
        validate_credential_structure(self)
    }
}

// =============================================================================
// Credential Builder
// =============================================================================

impl CredentialBuilder {
    pub fn new() -> Self {
        let mut this: Self = Default::default();

        this.context = Some(vec![Credential::BASE_CONTEXT.into()].into());
        this.types = Some(vec![Credential::BASE_TYPE.into()].into());
        this
    }

    /// Consumes the `CredentialBuilder`, returning a valid `Credential`
    pub fn build(self) -> Result<Credential> {
        let this: Credential = self.build_unchecked().map_err(Error::InvalidCredential)?;

        this.validate()?;

        Ok(this)
    }

    /// Consumes the `CredentialBuilder`, returning a valid `VerifiableCredential`
    pub fn build_verifiable(self, proof: impl Into<OneOrMany<Object>>) -> Result<VerifiableCredential> {
        self.build().map(|this| VerifiableCredential::new(this, proof))
    }
}
