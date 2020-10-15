use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    common::{Context, Object, OneOrMany, Url},
    error::{Error, Result},
    vc::{
        validate_presentation_structure, Credential, RefreshService, TermsOfUse, VerifiableCredential,
        VerifiablePresentation,
    },
};

/// A `Presentation` represents a bundle of one or more `VerifiableCredential`s.
///
/// `Presentation`s can be combined with `Proof`s to create `VerifiablePresentation`s.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Builder)]
#[builder(build_fn(name = "build_unchecked"), pattern = "owned")]
pub struct Presentation {
    /// A set of URIs or `Object`s describing the applicable JSON-LD contexts.
    ///
    /// NOTE: The first URI MUST be `https://www.w3.org/2018/credentials/v1`
    #[serde(rename = "@context")]
    #[builder(setter(into))]
    pub context: OneOrMany<Context>,
    /// A unique `URI` referencing the subject of the presentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub id: Option<Url>,
    /// One or more URIs defining the type of presentation.
    ///
    /// NOTE: The VC spec defines this as a set of URIs BUT they are commonly
    /// passed as non-`URI` strings and expected to be processed with JSON-LD.
    /// We're using a `String` here since we don't currently use JSON-LD and
    /// don't have any immediate plans to do so.
    #[serde(rename = "type")]
    #[builder(setter(into, strip_option))]
    pub types: OneOrMany<String>,
    /// TODO
    #[serde(rename = "verifiableCredential")]
    #[builder(default, setter(into, name = "credential"))]
    pub verifiable_credential: OneOrMany<VerifiableCredential>,
    /// The entity that generated the presentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub holder: Option<Url>,
    /// TODO
    #[serde(rename = "refreshService", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub refresh_service: Option<OneOrMany<RefreshService>>,
    /// The terms of use issued by the presentation holder
    #[serde(rename = "termsOfUse", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub terms_of_use: Option<OneOrMany<TermsOfUse>>,
    /// Miscellaneous properties.
    #[serde(flatten)]
    #[builder(default, setter(into))]
    pub properties: Object,
}

impl Presentation {
    pub const BASE_CONTEXT: &'static str = Credential::BASE_CONTEXT;

    pub const BASE_TYPE: &'static str = "VerifiablePresentation";

    pub fn validate(&self) -> Result<()> {
        validate_presentation_structure(self)
    }
}

// =============================================================================
// Presentation Builder
// =============================================================================

impl PresentationBuilder {
    pub fn new() -> Self {
        let mut this: Self = Default::default();

        this.context = Some(vec![Presentation::BASE_CONTEXT.into()].into());
        this.types = Some(vec![Presentation::BASE_TYPE.into()].into());
        this
    }

    /// Consumes the `PresentationBuilder_`, returning a valid `Presentation`
    pub fn build(self) -> Result<Presentation> {
        let this: Presentation = self.build_unchecked().map_err(Error::InvalidPresentation)?;

        this.validate()?;

        Ok(this)
    }

    /// Consumes the `PresentationBuilder_`, returning a valid `VerifiablePresentation`
    pub fn build_verifiable(self, proof: impl Into<OneOrMany<Object>>) -> Result<VerifiablePresentation> {
        self.build().map(|this| VerifiablePresentation::new(this, proof))
    }
}
