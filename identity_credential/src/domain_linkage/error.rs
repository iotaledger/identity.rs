use std::error::Error;

pub(crate) type DomainLinkageValidationResult = Result<(), DomainLinkageValidationError>;

#[derive(Debug, thiserror::Error)]
/// An error caused by a failure to verify a Domain Linkage configuration or credential.
pub struct DomainLinkageValidationError {
  /// Cause of the error.
  pub cause: DomainLinkageValidationErrorCause,
  /// Source of the error.
  pub source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for DomainLinkageValidationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.cause)
  }
}

impl From<DomainLinkageValidationError> for &str {
  fn from(value: DomainLinkageValidationError) -> Self {
    value.cause.into()
  }
}

/// The causes for why domain linkage validation can fail.
#[non_exhaustive]
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum DomainLinkageValidationErrorCause {
  #[error("invalid credential")]
  CredentialValidationError,
  #[error("invalid JWT")]
  InvalidJwt,
  #[error("the expiration date is missing")]
  MissingExpirationDate,
  #[error("id property is not allowed")]
  ImpermissibleIdProperty,
  #[error("issuer DID does not match the subject")]
  IssuerSubjectMismatch,
  #[error("subject id is invalid")]
  InvalidSubjectId,
  #[error("credential contains multiple subjects")]
  MultipleCredentialSubjects,
  #[error("invalid issuer DID")]
  InvalidIssuer,
  #[error("subject id property is missing")]
  MissingSubjectId,
  #[error("credential type is invalid")]
  InvalidTypeProperty,
  #[error("the issuer's id does not match the provided DID Document(s)")]
  DocumentMismatch,
  #[error("the subject's origin does not match the provided domain origin")]
  OriginMismatch,
  #[error("the subject's origin property is either invalid or missing")]
  InvalidSubjectOrigin,
  #[error("invalid semantic structure of the domain linkage configuration")]
  InvalidStructure,
  #[error("multiple domain linkage credentials reference the same DID")]
  AmbiguousCredential,
  #[error("a domain linkage credential referencing the provided DID is not found")]
  CredentialNotFound,
}
