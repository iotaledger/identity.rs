// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A Verifiable Presentation (VP) represents a bundle of one or more Verifiable Credentials.
//! This example demonstrates building and usage of VPs.
//!
//! cargo run --example create_vp

use identity::core::FromJson;
use identity::core::ToJson;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::Presentation;
use identity::credential::PresentationBuilder;
use identity::crypto::SignatureOptions;
use identity::did::verifiable::VerifierOptions;

use identity::iota::PresentationValidationOptions;
use identity::iota::PresentationValidator;
use identity::iota::Receipt;
use identity::iota::ResolvedIotaDocument;
use identity::iota::Resolver;
use identity::prelude::*;

mod common;
mod create_did;

/// Returns a triple corresponding to a Presentation, a credential issuer and the presentation holder
pub async fn create_vp() -> Result<(Presentation, IotaDocument, IotaDocument)> {
  // Create a signed DID Document/KeyPair for the credential issuer (see create_did.rs).
  let (doc_iss, key_iss, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Create a signed DID Document/KeyPair for the credential subject (see create_did.rs).
  let (doc_sub, key_sub, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Create an unsigned Credential with claims about `subject` specified by `issuer`.
  let mut credential: Credential = common::issue_degree(&doc_iss, &doc_sub)?;

  // Sign the Credential with the issuers private key.
  doc_iss.sign_data(
    &mut credential,
    key_iss.private(),
    doc_iss.default_signing_method()?.id(),
    SignatureOptions::default(),
  )?;

  // Create an unsigned Presentation from the previously issued Verifiable Credential.
  let mut presentation: Presentation = PresentationBuilder::default()
    .id(Url::parse("asdf:foo:a87w3guasbdfuasbdfs")?)
    .holder(Url::parse(doc_sub.id().as_ref())?)
    .credential(credential)
    .build()?;

  // Sign the presentation with the holders private key.
  //
  // Optionally include a challenge from the requester in the signature.
  // A unique random challenge generated by the requester per presentation can mitigate replay attacks
  // (along with other properties like `expires` and `domain`).
  doc_sub.sign_data(
    &mut presentation,
    key_sub.private(),
    doc_sub.default_signing_method()?.id(),
    SignatureOptions::new().challenge("475a7984-1bb5-4c4c-a56f-822bccd46440".to_owned()),
  )?;

  Ok((presentation, doc_iss, doc_sub))
}

#[tokio::main]
async fn main() -> Result<()> {
  // Issue a Verifiable Presentation with a newly created DID Document.
  let (presentation, issuer_doc, holder_doc): (Presentation, IotaDocument, IotaDocument) = create_vp().await?;

  // Convert the Verifiable Presentation to JSON and "exchange" with a verifier
  let presentation_json: String = presentation.to_json()?;

  // Create a `CredentialValidator` instance to fetch and validate all
  // associated DID Documents from the Tangle.
  let resolver: Resolver = Resolver::new().await?;

  // Validate the presentation and all the credentials included in it.
  //
  // Also verify the challenge matches.

  // deserialize the presentation:
  let presentation: Presentation = Presentation::from_json(&presentation_json)?;
  //Todo: Use the new Resolver to get the necessary DID documents once that becomes available.

  let resolved_holder_document: ResolvedIotaDocument = resolver.resolve(holder_doc.id()).await?;
  let trusted_issuer: ResolvedIotaDocument = resolver.resolve(issuer_doc.id()).await?;
  let trusted_issuers = &[trusted_issuer];

  let presentation_verifier_options =
    VerifierOptions::new().challenge("475a7984-1bb5-4c4c-a56f-822bccd46440".to_owned());

  let presentation_validation_options =
    PresentationValidationOptions::default().presentation_verifier_options(presentation_verifier_options);

  PresentationValidator::validate(
    &presentation,
    &presentation_validation_options,
    &resolved_holder_document,
    trusted_issuers,
    identity::iota::FailFast::Yes,
  )?;

  println!("successfully validated presentation!");
  Ok(())
}
