// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Result;
use crate::error::WasmError;
use crate::error::WasmResult;
use identity_iota::credential::ValidatorDocument;
use identity_iota::did::CoreDocument;
use identity_stardust::StardustDocument;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
#[serde(untagged)]
/// Temporary type used to convert to and from Box<dyn ValidatorDocument> until
/// we port the Document trait to these bindings.
pub(super) enum RustSupportedDocument {
  Stardust(StardustDocument),
  Core(CoreDocument),
}

impl RustSupportedDocument {
  pub(super) fn to_json(&self) -> Result<JsValue> {
    match self {
      Self::Core(doc) => JsValue::from_serde(doc).wasm_result(),
      Self::Stardust(doc) => JsValue::from_serde(doc).wasm_result(),
    }
  }
}

impl From<RustSupportedDocument> for Box<dyn ValidatorDocument> {
  fn from(document: RustSupportedDocument) -> Self {
    match document {
      RustSupportedDocument::Core(core_doc) => Box::new(core_doc) as Box<dyn ValidatorDocument>,
      RustSupportedDocument::Stardust(stardust_doc) => Box::new(stardust_doc) as Box<dyn ValidatorDocument>,
    }
  }
}

impl TryFrom<Box<dyn ValidatorDocument>> for RustSupportedDocument {
  type Error = WasmError<'static>;
  fn try_from(value: Box<dyn ValidatorDocument>) -> std::result::Result<Self, Self::Error> {
    let upcast = value.into_any();
    let supported_document = match upcast.downcast::<CoreDocument>() {
      Ok(doc) => RustSupportedDocument::Core(*doc),
      Err(retry) => {
        if let Ok(doc) = retry.downcast::<StardustDocument>() {
          RustSupportedDocument::Stardust(*doc)
        } else {
          Err(WasmError::new(
            "ResolutionError".into(),
            "Failed to cast the resolved did output to the required document type".into(),
          ))?
        }
      }
    };
    Ok(supported_document)
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Array<StardustDocument | CoreDocument>>")]
  pub type PromiseArraySupportedDocument;

  #[wasm_bindgen(typescript_type = "Promise<StardustDocument | CoreDocument>")]
  pub type PromiseSupportedDocument;

  #[wasm_bindgen(typescript_type = "StardustDocument | CoreDocument")]
  pub type SupportedDocument;
}
