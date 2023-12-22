use std::str::FromStr;

use identity_core::{convert::{ToJson, FromJson}, common::Object};
use identity_did::DID;
use jsonprooftoken::{jwp::issued::JwpIssuedDecoder, jpt::claims::JptClaims, encoding::SerializationType};

use crate::{validator::{JwtValidationError, SignerContext}, credential::{Jpt, CredentialJwtClaims}};

/// Utility functions for verifying JPT credentials.
#[derive(Debug)]
#[non_exhaustive]
pub struct JptCredentialValidatorUtils;

type ValidationUnitResult<T = ()> = std::result::Result<T, JwtValidationError>;

impl JptCredentialValidatorUtils {

    /// Utility for extracting the issuer field of a credential in JWT representation as DID.
    ///
    /// # Errors
    ///
    /// If the JWT decoding fails or the issuer field is not a valid DID.
    pub fn extract_issuer_from_jpt<D>(credential: &Jpt) -> std::result::Result<D, JwtValidationError>
    where
        D: DID,
        <D as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {

        let decoded = JwpIssuedDecoder::decode(credential.as_str(), SerializationType::COMPACT).map_err(|err| JwtValidationError::JwpDecodingError(err))?;
        let claims = decoded.get_header().claims().ok_or("Claims not present").map_err(|err| JwtValidationError::CredentialStructure(crate::Error::JptClaimsSetDeserializationError(err.into())))?;
        let payloads = decoded.get_payloads();
        let jpt_claims = JptClaims::from_claims_and_payloads(&claims, payloads);
        let jpt_claims_json = jpt_claims.to_json_vec().map_err(|err| JwtValidationError::CredentialStructure(crate::Error::JptClaimsSetDeserializationError(err.into())))?;
        
    
        // Deserialize the raw claims
        let credential_claims: CredentialJwtClaims<'_, Object> =
          CredentialJwtClaims::from_json_slice(&jpt_claims_json).map_err(|err| {
            JwtValidationError::CredentialStructure(crate::Error::JwtClaimsSetDeserializationError(err.into()))
        })?;

        D::from_str(credential_claims.iss.url().as_str()).map_err(|err| JwtValidationError::SignerUrl {
        signer_ctx: SignerContext::Issuer,
        source: err.into(),
        })
    }
}