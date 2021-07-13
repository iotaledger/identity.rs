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
use identity::prelude::*;

// Helper that takes two DID Documents (identities) for issuer and subject, and
// creates a credential with claims about subject by issuer.
pub fn issue_degree(issuer: &IotaDocument, subject: &IotaDocument) -> Result<Credential> {
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
