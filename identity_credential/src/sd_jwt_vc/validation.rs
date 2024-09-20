use crate::validator::JwtCredentialValidationOptions;

/// Options to decide which operations should be performed during SD-JWT VC validation.
#[derive(Debug, Clone)]
pub struct ValidationOptions {
  /// Credential validation options.
  pub credential_validation_options: JwtCredentialValidationOptions,
  /// The credential will be checked using the credential type
  /// specified through the `vct` claim.
  pub vct: bool,
}
