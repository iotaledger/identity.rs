// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::did::WasmCoreDocument;
use crate::error::WasmError;
use crate::iota::WasmIotaDocument;
use identity_iota::credential::AbstractValidatorDocument;
use identity_iota::did::CoreDocument;
use identity_iota::iota_core::IotaDocument;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
#[serde(untagged)]
/// Temporary type used to convert to and from Box<dyn ValidatorDocument> until
/// we port the Document trait to these bindings.
pub(super) enum RustSupportedDocument {
  Iota(IotaDocument),
  Core(CoreDocument),
}

impl From<RustSupportedDocument> for JsValue {
  fn from(document: RustSupportedDocument) -> Self {
    match document {
      RustSupportedDocument::Core(doc) => JsValue::from(WasmCoreDocument::from(doc)),
      RustSupportedDocument::Iota(doc) => JsValue::from(WasmIotaDocument(doc)),
    }
  }
}

impl From<RustSupportedDocument> for AbstractValidatorDocument {
  fn from(document: RustSupportedDocument) -> Self {
    match document {
      RustSupportedDocument::Core(core_doc) => AbstractValidatorDocument::from(core_doc),
      RustSupportedDocument::Iota(iota_doc) => AbstractValidatorDocument::from(iota_doc),
    }
  }
}

impl TryFrom<AbstractValidatorDocument> for RustSupportedDocument {
  type Error = WasmError<'static>;
  fn try_from(value: AbstractValidatorDocument) -> std::result::Result<Self, Self::Error> {
    let upcast = value.into_any();
    let supported_document = match upcast.downcast::<CoreDocument>() {
      Ok(doc) => RustSupportedDocument::Core(*doc),
      Err(retry) => {
        if let Ok(doc) = retry.downcast::<IotaDocument>() {
          RustSupportedDocument::Iota(*doc)
        } else {
          Err(WasmError::new(
            "CastingError".into(),
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
  #[wasm_bindgen(typescript_type = "Promise<Array<IotaDocument | CoreDocument>>")]
  pub type PromiseArraySupportedDocument;

  #[wasm_bindgen(typescript_type = "Promise<IotaDocument | CoreDocument>")]
  pub type PromiseSupportedDocument;

  #[wasm_bindgen(typescript_type = "IotaDocument | CoreDocument")]
  pub type SupportedDocument;

  #[wasm_bindgen(typescript_type = "IotaDocument | CoreDocument | undefined")]
  pub type OptionSupportedDocument;

  #[wasm_bindgen(typescript_type = "Array<IotaDocument | CoreDocument>")]
  pub type ArraySupportedDocument;

  #[wasm_bindgen(typescript_type = "Array<IotaDocument | CoreDocument> | undefined")]
  pub type OptionArraySupportedDocument;

}
