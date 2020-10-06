use crate::{
    common::{Context, Uri},
    error::{Error, Result},
    vc::{Credential, Presentation},
};

pub fn validate_credential_structure(credential: &Credential) -> Result<()> {
    // Ensure the base context is present and in the correct location
    match credential.context.get(0) {
        Some(Context::Uri(uri)) if uri == Credential::BASE_CONTEXT => {}
        Some(_) => return Err(Error::InvalidCredential("Invalid Base Context".into())),
        None => return Err(Error::InvalidCredential("Missing Base Context".into())),
    }

    // The set of types MUST contain the base type
    if !credential.types.contains(&Credential::BASE_TYPE.into()) {
        return Err(Error::InvalidCredential("Missing Base Type".into()));
    }

    // Ensure the id URI (if provided) adheres to the correct format
    if !is_valid_uri(credential.id.as_ref()) {
        return Err(Error::InvalidCredential("Invalid ID".into()));
    }

    // Ensure the issuer URI adheres to the correct format
    if !is_valid_uri(credential.issuer.uri()) {
        return Err(Error::InvalidCredential("Invalid Issuer".into()));
    }

    // Credentials MUST have at least one subject
    if credential.credential_subject.is_empty() {
        return Err(Error::InvalidCredential("Missing Subject".into()));
    }

    // Each subject is defined as one or more properties - no empty objects
    for subject in credential.credential_subject.iter() {
        if subject.id.is_none() && subject.properties.is_empty() {
            return Err(Error::InvalidCredential("Invalid Subject".into()));
        }
    }

    Ok(())
}

pub fn validate_presentation_structure(presentation: &Presentation) -> Result<()> {
    // Ensure the base context is present and in the correct location
    match presentation.context.get(0) {
        Some(Context::Uri(uri)) if uri == Presentation::BASE_CONTEXT => {}
        Some(_) => return Err(Error::InvalidPresentation("Invalid Base Context".into())),
        None => return Err(Error::InvalidPresentation("Missing Base Context".into())),
    }

    // The set of types MUST contain the base type
    if !presentation.types.contains(&Presentation::BASE_TYPE.into()) {
        return Err(Error::InvalidPresentation("Missing Base Type".into()));
    }

    // Ensure the id URI (if provided) adheres to the correct format
    if !is_valid_uri(presentation.id.as_ref()) {
        return Err(Error::InvalidPresentation("Invalid ID".into()));
    }

    // Ensure the holder URI (if provided) adheres to the correct format
    if !is_valid_uri(presentation.holder.as_ref()) {
        return Err(Error::InvalidPresentation("Invalid Holder".into()));
    }

    // Validate all verifiable credentials
    for credential in presentation.verifiable_credential.iter() {
        credential.validate()?;
    }

    Ok(())
}

fn is_valid_uri<'a>(value: impl Into<Option<&'a Uri>>) -> bool {
    const KNOWN: [&str; 4] = ["did:", "urn:", "http:", "https:"];

    if let Some(uri) = value.into() {
        // TODO: Proper URI validation
        if !KNOWN.iter().any(|scheme| uri.starts_with(scheme)) {
            return false;
        }
    }

    true
}
