use std::str::FromStr;
use crate::credential::Credential;

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

    /// Utility for extracting the issuer field of a [`Credential`] as a DID.
    ///
    /// # Errors
    ///
    /// Fails if the issuer field is not a valid DID.
    pub fn extract_issuer<D, T>(credential: &Credential<T>) -> std::result::Result<D, JwtValidationError>
    where
      D: DID,
      <D as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
      D::from_str(credential.issuer.url().as_str()).map_err(|err| JwtValidationError::SignerUrl {
        signer_ctx: SignerContext::Issuer,
        source: err.into(),
      })
    }

    /// Utility for extracting the issuer field of a credential in JPT representation as DID.
    ///
    /// # Errors
    ///
    /// If the JPT decoding fails or the issuer field is not a valid DID.
    pub fn extract_issuer_from_issued_jpt<D>(credential: &Jpt) -> std::result::Result<D, JwtValidationError>
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


  /// Checks whether the credential status has been revoked.
  pub fn check_status<T>(
    credential: &Credential<T>,
    status_check: crate::validator::StatusCheck,
  ) -> ValidationUnitResult {

    use crate::credential::RevocationTimeframeStatus;


    if status_check == crate::validator::StatusCheck::SkipAll {
      return Ok(());
    }

    match &credential.credential_status {
      None => Ok(()),
      Some(status) => {
        if status.type_ == RevocationTimeframeStatus::TYPE {
          let status: crate::credential::RevocationTimeframeStatus =
          crate::credential::RevocationTimeframeStatus::try_from(status.clone())
            .map_err(JwtValidationError::InvalidStatus)?;

          Self::check_revocation_validity_timeframe_status(status)

        } else {
          if status_check == crate::validator::StatusCheck::SkipUnsupported {
            return Ok(());
          }
          return Err(JwtValidationError::InvalidStatus(crate::Error::InvalidStatus(format!(
            "unsupported type '{}'",
            status.type_
          ))));
        }
        
      }
    }
  }


  fn check_revocation_validity_timeframe_status(
    status: crate::credential::RevocationTimeframeStatus,
  ) -> ValidationUnitResult {
    use identity_core::common::Timestamp;


    let now = Timestamp::now_utc();

    let check = status.validity_timeframe().is_ok_and(|t| {
      status.granularity().is_ok_and(|e| {{
        match e {
            crate::credential::ValidityTimeframeGranularity::SECOND => now == t,
            crate::credential::ValidityTimeframeGranularity::MINUTE => now.to_minute() == t,
            crate::credential::ValidityTimeframeGranularity::HOUR => now.to_hour() == t,
        }
      }})
    });

    if !check {
      Err(JwtValidationError::Revoked)
    } else {
      Ok(())
    }
    
  }
}