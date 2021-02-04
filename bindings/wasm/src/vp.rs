// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::OneOrMany;
use identity::core::Url;
use identity::credential::Presentation;
use identity::credential::PresentationBuilder;
use identity::credential::VerifiableCredential;
use identity::credential::VerifiablePresentation as VP;
use wasm_bindgen::prelude::*;

use crate::document::Document;
use crate::key::KeyPair;
use crate::utils::err;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct VerifiablePresentation(pub(crate) VP);

#[wasm_bindgen]
impl VerifiablePresentation {
  #[wasm_bindgen(constructor)]
  pub fn new(
    holder_doc: &Document,
    holder_key: &KeyPair,
    credential_data: JsValue,
    presentation_type: Option<String>,
    presentation_id: Option<String>,
  ) -> Result<VerifiablePresentation, JsValue> {
    let credentials: OneOrMany<VerifiableCredential> = credential_data.into_serde().map_err(err)?;
    let holder_url: Url = Url::parse(holder_doc.0.id().as_str()).map_err(err)?;

    let mut builder: PresentationBuilder = PresentationBuilder::default().holder(holder_url);

    for credential in credentials.into_vec() {
      builder = builder.credential(credential);
    }

    if let Some(presentation_type) = presentation_type {
      builder = builder.type_(presentation_type);
    }

    if let Some(presentation_id) = presentation_id {
      builder = builder.id(Url::parse(presentation_id).map_err(err)?);
    }

    let presentation: Presentation = builder.build().map_err(err)?;
    let mut this: Self = Self(VP::new(presentation, Vec::new()));

    this.sign(holder_doc, holder_key)?;

    Ok(this)
  }

  /// Signs the presentation with the given holder `Document` and `KeyPair` object.
  #[wasm_bindgen]
  pub fn sign(&mut self, holder: &Document, key: &KeyPair) -> Result<(), JsValue> {
    holder.0.sign_data(&mut self.0, key.key.secret()).map_err(err)
  }

  /// Verifies the presentation signature against the holder `Document`.
  #[wasm_bindgen]
  pub fn verify(&self, holder: &Document) -> Result<bool, JsValue> {
    holder.0.verify_data(&self.0).map_err(err).map(|_| true)
  }

  /// Serializes a `VerifiablePresentation` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `VerifiablePresentation` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<VerifiablePresentation, JsValue> {
    json.into_serde().map_err(err).map(Self)
  }
}
