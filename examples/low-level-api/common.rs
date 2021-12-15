// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This file contains helper functions for the examples.

#![allow(dead_code)]

use identity::core::json;
use identity::core::FromJson;
use identity::core::Timestamp;
use identity::core::ToJson;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::CredentialBuilder;
use identity::credential::Subject;
use identity::did::verifiable::VerifierOptions;
use identity::did::MethodScope;
use identity::did::DID;
use identity::iota::ClientMap;
use identity::iota::CredentialValidation;
use identity::iota::CredentialValidator;
use identity::iota::IotaVerificationMethod;
use identity::iota::Receipt;
use identity::prelude::*;

/// Helper that takes two DID Documents (identities) for issuer and subject, and
/// creates an unsigned credential with claims about subject by issuer.
pub fn issue_degree(issuer: &IotaDocument, subject: &IotaDocument) -> Result<Credential> {
  // Create VC "subject" field containing subject ID and claims about it.
  let subject: Subject = Subject::from_json_value(json!({
    "id": subject.id().as_str(),
    "name": "Alice",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts",
    },
    "GPA": "4.0",
  }))?;

  // Build credential using subject above and issuer.
  let credential: Credential = CredentialBuilder::default()
    .id(Url::parse("https://example.edu/credentials/3732")?)
    .issuer(Url::parse(issuer.id().as_str())?)
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .build()?;

  Ok(credential)
}

/// Convenience function for checking that a verifiable credential is valid and not revoked.
pub async fn check_credential(client: &ClientMap, credential: &Credential) -> Result<CredentialValidation> {
  // Convert the Verifiable Credential to JSON to potentially "exchange" with a verifier
  let credential_json = credential.to_json()?;

  // Create a `CredentialValidator` instance to fetch and validate all
  // associated DID Documents from the Tangle.
  let validator: CredentialValidator<ClientMap> = CredentialValidator::new(client);

  // Perform the validation operation.
  let validation: CredentialValidation = validator
    .check_credential(&credential_json, VerifierOptions::default())
    .await?;
  Ok(validation)
}

/// Convenience function for adding a new `VerificationMethod` with tag #newKey to a DID document
/// and performing an integration chain update, publishing it to the Tangle.
///
/// See "manipulate_did" for further explanation.
pub async fn add_new_key(
  client: &ClientMap,
  doc: &IotaDocument,
  key: &KeyPair,
  receipt: &Receipt,
) -> Result<(IotaDocument, KeyPair, Receipt)> {
  let mut updated_doc = doc.clone();

  // Add #newKey to the document
  let new_key: KeyPair = KeyPair::new_ed25519()?;
  let method: IotaVerificationMethod =
    IotaVerificationMethod::from_did(updated_doc.id().clone(), new_key.type_(), new_key.public(), "newKey")?;
  assert!(updated_doc
    .insert_method(method, MethodScope::VerificationMethod)
    .is_ok());

  // Prepare the update
  updated_doc.metadata.previous_message_id = *receipt.message_id();
  updated_doc.metadata.updated = Timestamp::now_utc();
  updated_doc.sign_self(key.private(), &updated_doc.default_signing_method()?.id())?;

  // Publish the update to the Tangle
  let update_receipt: Receipt = client.publish_document(&updated_doc).await?;
  Ok((updated_doc, new_key, update_receipt))
}
