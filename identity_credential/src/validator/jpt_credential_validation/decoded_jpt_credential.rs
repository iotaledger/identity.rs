use identity_core::common::Object;
use jsonprooftoken::jwp::issued::JwpIssued;

use crate::credential::Credential;

pub struct DecodedJptCredential<T = Object> {
    /// The decoded credential parsed to the [Verifiable Credentials Data model](https://www.w3.org/TR/vc-data-model/).
    pub credential: Credential<T>,
    /// The custom claims parsed from the JWT.
    pub custom_claims: Option<Object>,
    /// The decoded Issued Jwp, will be used to construct the Presented Jwp
    pub decoded_jwp: JwpIssued
}