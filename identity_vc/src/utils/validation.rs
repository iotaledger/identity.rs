use crate::{
  common::{Context, OneOrMany, URI},
  credential::Credential,
  error::{Error, Result},
  presentation::Presentation,
};

pub fn validate_credential_structure(credential: &Credential) -> Result<()> {
  // Ensure the base context is present and in the correct location
  validate_context("Credential", &credential.context)?;

  // The set of types MUST contain the base type
  validate_types("Credential", Credential::BASE_TYPE, &credential.types)?;

  // Ensure the id URI (if provided) adheres to the correct format
  validate_opt_uri("Credential id", credential.id.as_ref())?;

  // Ensure the issuer URI adheres to the correct format
  validate_uri("Credential issuer", credential.issuer.uri())?;

  // Credentials MUST have at least one subject
  if credential.credential_subject.is_empty() {
    return Err(Error::MissingCredentialSubject);
  }

  // Each subject is defined as one or more properties - no empty objects
  for subject in credential.credential_subject.iter() {
    if subject.id.is_none() && subject.properties.is_empty() {
      return Err(Error::InvalidCredentialSubject);
    }
  }

  Ok(())
}

pub fn validate_presentation_structure(presentation: &Presentation) -> Result<()> {
  // Ensure the base context is present and in the correct location
  validate_context("Presentation", &presentation.context)?;

  // The set of types MUST contain the base type
  validate_types("Presentation", Presentation::BASE_TYPE, &presentation.types)?;

  // Ensure the id URI (if provided) adheres to the correct format
  validate_opt_uri("Presentation id", presentation.id.as_ref())?;

  // Ensure the holder URI (if provided) adheres to the correct format
  validate_opt_uri("Presentation holder", presentation.holder.as_ref())?;

  // Validate all verifiable credentials
  for credential in presentation.verifiable_credential.iter() {
    credential.validate()?;
  }

  Ok(())
}

pub fn validate_types(name: &'static str, base: &str, types: &OneOrMany<String>) -> Result<()> {
  if !types.contains(&base.into()) {
    return Err(Error::MissingBaseType(name));
  }

  Ok(())
}

pub fn validate_context(name: &'static str, context: &OneOrMany<Context>) -> Result<()> {
  // The first Credential/Presentation context MUST be a URI representing the base context
  match context.get(0) {
    Some(Context::URI(uri)) if uri == Credential::BASE_CONTEXT => Ok(()),
    Some(_) => Err(Error::InvalidBaseContext(name)),
    None => Err(Error::MissingBaseContext(name)),
  }
}

pub fn validate_uri(name: &'static str, uri: &URI) -> Result<()> {
  const KNOWN: [&str; 4] = ["did:", "urn:", "http:", "https:"];

  // TODO: Proper URI validation
  if !KNOWN.iter().any(|scheme| uri.starts_with(scheme)) {
    return Err(Error::InvalidURI(name));
  }

  Ok(())
}

pub fn validate_opt_uri(name: &'static str, uri: Option<&URI>) -> Result<()> {
  match uri {
    Some(uri) => validate_uri(name, uri),
    None => Ok(()),
  }
}
