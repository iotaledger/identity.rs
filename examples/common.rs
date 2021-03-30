// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// This file contains helper functions for the examples.

#![allow(dead_code)]

use identity::core::json;
use identity::core::FromJson;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::CredentialBuilder;
use identity::credential::Subject;
use identity::crypto::KeyPair;
use identity::iota::Client;
use identity::iota::Document;
use identity::iota::Result;
use identity::prelude::*;

// A helper function to generate a new DID Document/KeyPair, sign the
// document, publish it to the Tangle, and return the Document/KeyPair.
pub async fn create_did_document(client: &Client) -> Result<(Document, KeyPair)> {
  // Generate a new DID Document and public/private key pair.
  // The generated document will have an authentication key associated with
  // the keypair.
  let keypair: KeyPair = KeyPair::new_ed25519()?;
  let mut document: Document = Document::from_keypair(&keypair)?;

  // Sign the DID Document with the default authentication key.
  document.sign(keypair.secret())?;

  // Use the Client to publish the DID Document to the Tangle.
  document.publish(client).await?;

  // Return document and keypair.
  Ok((document, keypair))
}

// Helper that takes two DID Documents (identities) for issuer and subject, and
// creates a credential with claims about subject by issuer.
pub fn issue_degree(issuer: &Document, subject: &Document) -> Result<Credential> {
  // Create VC "subject" field containing subject ID and claims about it.
  let subject: Subject = Subject::from_json_value(json!({
    "id": subject.id().as_str(),
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts"
    }
  }))?;

  // Build credential using subject above and issuer.
  let credential: Credential = CredentialBuilder::default()
    .issuer(Url::parse(issuer.id().as_str())?)
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .build()?;

  Ok(credential)
}
