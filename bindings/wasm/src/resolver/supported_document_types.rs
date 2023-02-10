// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::did::WasmCoreDID;
use crate::did::WasmCoreDocument;
use crate::error::WasmResult;
use crate::iota::WasmIotaDID;
use crate::iota::WasmIotaDocument;
use identity_iota::did::CoreDID;
use identity_iota::did::DID;
use identity_iota::document::CoreDocument;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Deserialize)]
#[serde(untagged)]
/// Temporary type used for the resolver.
pub enum RustSupportedDocument {
  Iota(IotaDocument),
  Core(CoreDocument),
}

impl AsRef<CoreDocument> for RustSupportedDocument {
  fn as_ref(&self) -> &CoreDocument {
    match self {
      Self::Iota(doc) => doc.as_ref(),
      Self::Core(doc) => doc,
    }
  }
}

impl From<IotaDocument> for RustSupportedDocument {
  fn from(value: IotaDocument) -> Self {
    Self::Iota(value)
  }
}

impl From<CoreDocument> for RustSupportedDocument {
  fn from(value: CoreDocument) -> Self {
    Self::Core(value)
  }
}

impl From<RustSupportedDocument> for JsValue {
  fn from(document: RustSupportedDocument) -> Self {
    match document {
      RustSupportedDocument::Core(doc) => JsValue::from(WasmCoreDocument::from(doc)),
      RustSupportedDocument::Iota(doc) => JsValue::from(WasmIotaDocument::from(doc)),
    }
  }
}

impl TryFrom<CoreDID> for SupportedDID {
  type Error = JsValue;

  fn try_from(did: CoreDID) -> Result<Self, Self::Error> {
    let js: JsValue = if did.method() == IotaDID::METHOD {
      let ret: IotaDID = IotaDID::try_from_core(did).wasm_result()?;
      JsValue::from(WasmIotaDID::from(ret))
    } else {
      JsValue::from(WasmCoreDID::from(did))
    };

    Ok(js.unchecked_into::<SupportedDID>())
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Array<CoreDocument | IAsCoreDocument>>")]
  pub type PromiseArraySupportedDocument;

  #[wasm_bindgen(typescript_type = "Promise<CoreDocument | IAsCoreDocument>")]
  pub type PromiseSupportedDocument;

  #[wasm_bindgen(typescript_type = "CoreDocument | IAsCoreDocument")]
  pub type SupportedDocument;

  #[wasm_bindgen(typescript_type = "CoreDocument | IAsCoreDocument | undefined")]
  pub type OptionSupportedDocument;

  #[wasm_bindgen(typescript_type = "Array<CoreDocument | IAsCoreDocument> | undefined")]
  pub type OptionArraySupportedDocument;

  //TODO: Remove this and use IAsCoreDID instead.
  #[wasm_bindgen(typescript_type = "CoreDID | IAsCoreDID")]
  pub type SupportedDID;
}
