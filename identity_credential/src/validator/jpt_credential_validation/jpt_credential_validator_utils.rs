use std::str::FromStr;
use crate::{credential::Credential, revocation::{RevocationDocumentExt, RevocationTimeframeStatus, VerifierRevocationTimeframeStatus}};

use identity_core::{common::{Object, Timestamp}, convert::{FromJson, ToJson}};
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


  // /// Checks whether the credential status has been revoked.
  // pub fn check_status<T>(
  //   credential: &Credential<T>,
  //   status_check: crate::validator::StatusCheck,
  // ) -> ValidationUnitResult {


  //   if status_check == crate::validator::StatusCheck::SkipAll {
  //     return Ok(());
  //   }

  //   match &credential.credential_status {
  //     None => Ok(()),
  //     Some(status) => {
  //       if status.type_ == RevocationTimeframeStatus::TYPE {
  //         let status: RevocationTimeframeStatus = RevocationTimeframeStatus::try_from(status.clone())
  //           .map_err(JwtValidationError::InvalidStatus)?;

  //         Self::check_validity_timeframe(status, Some(Timestamp::now_utc()))

  //       } else {
  //         if status_check == crate::validator::StatusCheck::SkipUnsupported {
  //           return Ok(());
  //         }
  //         return Err(JwtValidationError::InvalidStatus(crate::Error::InvalidStatus(format!(
  //           "unsupported type '{}'",
  //           status.type_
  //         ))));
  //       }
        
  //     }
  //   }
  // }



  /// Check timeframe interval in credentialStatus with `RevocationTimeframeStatus`.
  pub fn check_timeframes_with_validity_timeframe_2024<T>(
    credential: &Credential<T>,
    validity_timeframe: Option<Timestamp>,
    status_check: crate::validator::StatusCheck,
  ) -> ValidationUnitResult {

    if status_check == crate::validator::StatusCheck::SkipAll {
      return Ok(());
    }

    match &credential.credential_status {
      None => Ok(()),
      Some(status) => {
        if status.type_ == RevocationTimeframeStatus::TYPE {
          let status: RevocationTimeframeStatus = RevocationTimeframeStatus::try_from(status.clone())
            .map_err(JwtValidationError::InvalidStatus)?;

            Self::check_validity_timeframe(status, validity_timeframe)

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


  pub(crate) fn check_validity_timeframe(
    status: RevocationTimeframeStatus,
    validity_timeframe: Option<Timestamp>
  ) -> ValidationUnitResult {

    let timeframe = validity_timeframe.unwrap_or(Timestamp::now_utc());

    let check = status.start_validity_timeframe().is_ok_and(|start| {

      status.end_validity_timeframe().is_ok_and(|end| {
        timeframe >= start && timeframe <= end
      })
    });

    if !check {
      Err(JwtValidationError::OutsideTimeframe)
    } else {
      Ok(())
    }
    
  }

  /// Checks whether the credential status has been revoked
  /// 
  /// Only supports `RevocationTimeframe2024`.
  pub fn check_revocation_with_validity_timeframe_2024<DOC: AsRef<identity_document::document::CoreDocument> + ?Sized, T>(
    credential: &Credential<T>,
    issuer: &DOC,
    status_check: crate::validator::StatusCheck,
  ) -> ValidationUnitResult {

    if status_check == crate::validator::StatusCheck::SkipAll {
      return Ok(());
    }

    match &credential.credential_status {
      None => Ok(()),
      Some(status) => {
        if status.type_ == RevocationTimeframeStatus::TYPE {
          let status: RevocationTimeframeStatus = RevocationTimeframeStatus::try_from(status.clone())
            .map_err(JwtValidationError::InvalidStatus)?;

            Self::check_revocation_bitmap(issuer, status)

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


  /// Check the given `status` against the matching [`RevocationBitmap`] service in the issuer's DID Document.
  fn check_revocation_bitmap<DOC: AsRef<identity_document::document::CoreDocument> + ?Sized>(
    issuer: &DOC,
    status: RevocationTimeframeStatus,
  ) -> ValidationUnitResult {

    let issuer_service_url: identity_did::DIDUrl = status.id().map_err(JwtValidationError::InvalidStatus)?;

    // Check whether index is revoked.
    let revocation_bitmap: crate::revocation::RevocationBitmap = issuer
      .as_ref()
      .resolve_revocation_bitmap(issuer_service_url.into())
      .map_err(|_| JwtValidationError::ServiceLookupError)?;
    let index: u32 = status.index().map_err(JwtValidationError::InvalidStatus)?;
    if revocation_bitmap.is_revoked(index) {
      Err(JwtValidationError::Revoked)
    } else {
      Ok(())
    }
  }


  
}