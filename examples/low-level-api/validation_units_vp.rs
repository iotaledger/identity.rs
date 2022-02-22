// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::FromJson;
use identity::core::Timestamp;
use identity::core::ToJson;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::Presentation;
use identity::credential::PresentationBuilder;
use identity::crypto::SignatureOptions;
use identity::did::verifiable::VerifierOptions;
use identity::iota::ClientMap;
use identity::iota::CredentialValidationOptions;
use identity::iota::CredentialValidator;
use identity::iota::PresentationValidator;
use identity::iota::Receipt;
use identity::iota::ResolvedIotaDocument;
use identity::iota::TangleResolve;
use identity::prelude::IotaDocument;

use identity::prelude::*;

mod common;
mod create_did;

/// Returns a triple corresponding of a presentation, the DID Document of a credential issuer and the DID document of
/// the holder.
///
/// This simulates a subject trying to cheat a holder by sending a presentation containing a credential issued to
/// someone else.
pub async fn create_invalid_vp() -> Result<(Presentation, IotaDocument, IotaDocument)> {
  // Create a signed DID Document/KeyPair for the credential issuer (see create_did.rs).
  let (doc_iss, key_iss, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Create a signed DID Document/KeyPair for the presentation holder
  let (holder_doc, key_holder, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Create a signed DID Document/KeyPair for a legitimate credential subject (see create_did.rs).
  let (doc_credential_subject, _, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Create an unsigned Credential with claims about `credential_subject` specified by `issuer`.
  let mut credential: Credential = common::issue_degree(&doc_iss, &doc_credential_subject)?;

  // The issuer sets the nonTransferable property which means that only the credential subject may create a VP
  // containing the credential.
  credential.non_transferable = Some(true);

  // The issuer signs the Credential with their private key.
  doc_iss.sign_data(
    &mut credential,
    key_iss.private(),
    doc_iss.default_signing_method()?.id(),
    SignatureOptions::default(),
  )?;

  // Create an unsigned Presentation from the previously issued Verifiable Credential.
  let mut presentation: Presentation = PresentationBuilder::default()
    .id(Url::parse("asdf:foo:a87w3guasbdfuasbdfs")?)
    .holder(Url::parse(holder_doc.id().as_ref())?)
    .credential(credential)
    .build()?;

  // The holder signs the presentation with their private key and includes a challenge and an expiry timestamp 10
  // minutes from now as demanded by the requester (to mitigate replay attacks). The expiry timestamp enables the
  // issuer to drop the challenge from memory after 10 minutes.

  holder_doc.sign_data(
    &mut presentation,
    key_holder.private(),
    holder_doc.default_signing_method()?.id(),
    SignatureOptions::new()
      .challenge("475a7984-1bb5-4c4c-a56f-822bccd46440".to_owned())
      .expires(Timestamp::from_unix(Timestamp::now_utc().to_unix() + 600)?),
  )?;

  Ok((presentation, doc_iss, holder_doc))
}

#[tokio::main]
async fn main() -> Result<()> {
  // Create a client instance to send messages to the Tangle.
  let client: ClientMap = ClientMap::new();

  // Issue a Verifiable Presentation with a newly created DID Document.
  let (presentation, issuer_doc, holder_doc): (Presentation, IotaDocument, IotaDocument) = create_invalid_vp().await?;

  // Convert the Verifiable Presentation to JSON and "exchange" with a verifier
  let presentation_json: String = presentation.to_json()?;

  // Validate the presentation and all the credentials included in it.

  // deserialize the presentation:
  let presentation: Presentation = Presentation::from_json(&presentation_json)?;

  // Normally we would check the nonTransferable property first, but let us see that other validations pass as a sanity
  // check.

  // Verify the signature
  //Todo: Use the new Resolver to get the necessary DID documents once that becomes available.

  let resolved_holder_document: ResolvedIotaDocument = client.resolve(holder_doc.id()).await?;
  PresentationValidator::verify_presentation_signature(
    &presentation,
    &resolved_holder_document,
    &VerifierOptions::default()
      .challenge("475a7984-1bb5-4c4c-a56f-822bccd46440".to_owned())
      .allow_expired(false),
  )?;
  println!("verified the holder's signature");

  // extract and validate the presentation's credentials
  let resolved_issuer_doc: ResolvedIotaDocument = client.resolve(issuer_doc.id()).await?;
  let fail_fast = true;
  for credential in presentation.verifiable_credential.iter() {
    CredentialValidator::new().validate(
      credential,
      &CredentialValidationOptions::default(),
      &resolved_issuer_doc,
      fail_fast,
    )?;
  }
  println!("validated all credentials of the presentation");

  // Check if there are any nonTransferable violations
  if let Err(non_transferable_error) = PresentationValidator::check_non_transferable(&presentation) {
    // print the error message
    println!("{}", non_transferable_error);
    // give some more context
    println!("Possible attempt to cheat presentation validation detected");
  }
  Ok(())
}
