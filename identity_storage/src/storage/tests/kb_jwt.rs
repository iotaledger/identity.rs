// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::{
  common::{Object, Url},
  convert::{FromJson, ToJson},
};
use identity_credential::{
  credential::{Credential, CredentialBuilder, Jws, Subject},
  sd_jwt::{KeyBindingJwtClaims, SdJwt, SdObjectDecoder, SdObjectEncoder, Sha256Hasher},
  validator::{FailFast, JwtCredentialValidationOptions, SdJwtValidator},
};
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota_core::IotaDocument;
use serde_json::json;

use crate::{JwkDocumentExt, JwsSignatureOptions};

use super::test_utils::{setup_iotadocument, Setup};

#[tokio::test]
async fn name() {
  let setup: Setup<IotaDocument, IotaDocument> = setup_iotadocument(None, None).await;

  let credential_json: &str = r#"
    {
      "@context": [
        "https://www.w3.org/2018/credentials/v1",
        "https://www.w3.org/2018/credentials/examples/v1"
      ],
      "id": "http://example.edu/credentials/3732",
      "type": ["VerifiableCredential", "UniversityDegreeCredential"],
      "issuer": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
      "issuanceDate": "2010-01-01T19:23:24Z",
      "credentialSubject": {
        "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
        "degree": {
          "type": "BachelorDegree",
          "name": "Bachelor of Science in Mechanical Engineering"
        }
      }
    }"#;
  let subject: Subject = Subject::from_json_value(json!({
    "id": setup.subject_doc.id().to_string(),
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science in Mechanical Engineering"
    }
  }))
  .unwrap();

  // Build credential using subject above and issuer.
  let credential: Credential = CredentialBuilder::default()
    .id(Url::parse("https://example.edu/credentials/3732").unwrap())
    .issuer(Url::parse(setup.issuer_doc.id().to_string()).unwrap())
    .type_("AddressCredential")
    .subject(subject)
    .build()
    .unwrap();

  let payload = credential.serialize_jwt(None).unwrap();

  let mut encoder = SdObjectEncoder::new(&payload).unwrap();
  let mut disclosures = vec![];
  disclosures.push(
    encoder
      .conceal(&["vc", "credentialSubject", "degree", "type"], None)
      .unwrap(),
  );
  disclosures.push(
    encoder
      .conceal(&["vc", "credentialSubject", "degree", "name"], None)
      .unwrap(),
  );
  encoder.add_sd_alg_property();
  let encoded_payload = encoder.try_to_string().unwrap();

  let jwt: Jws = setup
    .issuer_doc
    .create_jws(
      &setup.issuer_storage,
      &setup.issuer_method_fragment,
      encoded_payload.as_bytes(),
      &JwsSignatureOptions::default(),
    )
    .await
    .unwrap();

  let disclosures: Vec<String> = disclosures
    .clone()
    .into_iter()
    .map(|disclosure| disclosure.to_string())
    .collect();
  let nonce = "nonce-test";
  let verifier_id = "did:test:verifier";
  let binding_claims = KeyBindingJwtClaims::new(
    &Sha256Hasher::new(),
    jwt.as_str().to_string(),
    disclosures.clone(),
    nonce.to_string(),
    verifier_id.to_string(),
    None,
  )
  .to_json()
  .unwrap();

  // Setting the `typ` in the header is required.
  let options = JwsSignatureOptions::new().typ(KeyBindingJwtClaims::KB_JWT_HEADER_TYP);

  // Create the KB-JWT.
  let kb_jwt: Jws = setup
    .subject_doc
    .create_jws(
      &setup.subject_storage,
      &setup.subject_method_fragment,
      binding_claims.as_bytes(),
      &options,
    )
    .await
    .unwrap();
  let sd_jwt_obj = SdJwt::new(jwt.into(), disclosures.clone(), Some(kb_jwt.into()));
  let decoder = SdObjectDecoder::new_with_sha256();
  let validator = SdJwtValidator::new(EdDSAJwsVerifier::default(), decoder);
  let validation = validator
    .validate_credential::<_, Object>(
      &sd_jwt_obj,
      &setup.issuer_doc,
      &JwtCredentialValidationOptions::default(),
      FailFast::FirstError,
    )
    .unwrap();
}
