// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::OneOrMany;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::CredentialBuilder;
use identity::credential::Subject;
use identity::credential::VerifiableCredential as VC;
use wasm_bindgen::prelude::*;

use crate::document::Document;
use crate::key::KeyPair;
use crate::utils::err;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct VerifiableCredential(pub(crate) VC);

#[wasm_bindgen]
impl VerifiableCredential {
  #[wasm_bindgen(constructor)]
  pub fn new(
    issuer_doc: &Document,
    issuer_key: &KeyPair,
    subject_data: JsValue,
    credential_type: Option<String>,
    credential_id: Option<String>,
  ) -> Result<VerifiableCredential, JsValue> {
    let subjects: OneOrMany<Subject> = subject_data.into_serde().map_err(err)?;
    let issuer_url: Url = Url::parse(issuer_doc.0.id().as_str()).map_err(err)?;

    let mut builder: CredentialBuilder = CredentialBuilder::default().issuer(issuer_url);

    for subject in subjects.into_vec() {
      builder = builder.subject(subject);
    }

    if let Some(credential_type) = credential_type {
      builder = builder.type_(credential_type);
    }

    if let Some(credential_id) = credential_id {
      builder = builder.id(Url::parse(credential_id).map_err(err)?);
    }

    let credential: Credential = builder.build().map_err(err)?;
    let mut this: Self = Self(VC::new(credential, Vec::new()));

    this.sign(issuer_doc, issuer_key)?;

    Ok(this)
  }

  /// Signs the credential with the given issuer `Document` and `KeyPair` object.
  #[wasm_bindgen]
  pub fn sign(&mut self, issuer: &Document, key: &KeyPair) -> Result<(), JsValue> {
    issuer.0.sign_data(&mut self.0, key.key.secret()).map_err(err)
  }

  /// Verifies the credential signature against the issuer `Document`.
  #[wasm_bindgen]
  pub fn verify(&self, issuer: &Document) -> Result<bool, JsValue> {
    issuer.0.verify_data(&self.0).map_err(err).map(|_| true)
  }

  /// Serializes a `VerifiableCredential` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `VerifiableCredential` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<VerifiableCredential, JsValue> {
    json.into_serde().map_err(err).map(Self)
  }
}
