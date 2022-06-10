// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_credential::credential::Credential;
use identity_credential::credential::CredentialBuilder;
use identity_credential::credential::Issuer;
use identity_credential::credential::Subject;
use identity_credential::presentation::Presentation;
use serde_json::json;
use serde_json::Value;

#[test]
fn doc_test_build_credential() {
  // Construct a `Subject` from json
  let json_subject: Value = json!({
    "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts"
    }
  });
  let subject: Subject = serde_json::from_value(json_subject).unwrap();

  // Construct an `Issuer` from json
  let json_issuer: Value = json!({
    "id": "did:example:76e12ec712ebc6f1c221ebfeb1f",
    "name": "Example University"
  });

  let issuer: Issuer = serde_json::from_value(json_issuer).unwrap();

  let credential: Credential = CredentialBuilder::default()
    .context(Url::parse("https://www.w3.org/2018/credentials/examples/v1").unwrap())
    .id(Url::parse("http://example.edu/credentials/3732").unwrap())
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .issuer(issuer)
    .issuance_date(Timestamp::parse("2010-01-01T00:00:00Z").unwrap())
    .build()
    .unwrap();
}
