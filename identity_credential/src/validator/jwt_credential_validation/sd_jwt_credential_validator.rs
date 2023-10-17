use identity_document::document::CoreDocument;
use identity_verification::jws::JwsVerifier;

use crate::{
  credential::Jwt,
  sd_jwt::{Hasher, SdJwt, SdPayloadDecoder},
  validator::FailFast,
};

use super::{
  CompoundCredentialValidationError, DecodedJwtCredential, JwtCredentialValidationOptions, JwtCredentialValidator,
};

/// A type for decoding and validating [`Credential`]s.
#[non_exhaustive]
pub struct SdJwtCredentialValidator<V: JwsVerifier, H: Hasher>(V, SdPayloadDecoder<H>);

impl<V: JwsVerifier, H: Hasher> SdJwtCredentialValidator<V, H> {
  ///
  pub fn new(signature_verifier: V, sd_decoder: SdPayloadDecoder<H>) -> Self {
    Self(signature_verifier, sd_decoder)
  }

  ///
  pub fn validate<DOC, T>(
    &self,
    sd_jwt: &SdJwt,
    issuer: &DOC,
    options: &JwtCredentialValidationOptions,
    fail_fast: FailFast,
  ) -> Result<DecodedJwtCredential<T>, CompoundCredentialValidationError>
  where
    T: ToOwned<Owned = T> + serde::Serialize + serde::de::DeserializeOwned,
    DOC: AsRef<CoreDocument>,
  {
    JwtCredentialValidator::<V>::validate_extended::<CoreDocument, _, T, H>(
      &self.0,
      &Jwt::new(sd_jwt.jwt.to_string()),
      std::slice::from_ref(issuer.as_ref()),
      options,
      fail_fast,
      Some(&self.1),
      Some(&sd_jwt.disclosures),
    )
  }
}
