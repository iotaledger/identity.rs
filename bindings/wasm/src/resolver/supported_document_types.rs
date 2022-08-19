// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::WasmError;
use identity_iota::credential::ValidatorDocument;
use identity_iota::did::CoreDocument;
use identity_stardust::StardustDocument;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
/// Temporary type used to convert to and from Box<dyn ValidatorDocument> until
/// we port the Document trait to these bindings.
pub(super) enum SupportedDocument {
  Core(CoreDocument),
  Stardust(StardustDocument),
}

impl From<SupportedDocument> for Box<dyn ValidatorDocument> {
  fn from(document: SupportedDocument) -> Self {
    match document {
      SupportedDocument::Core(core_doc) => Box::new(core_doc) as Box<dyn ValidatorDocument>,
      SupportedDocument::Stardust(stardust_doc) => Box::new(stardust_doc) as Box<dyn ValidatorDocument>,
    }
  }
}

impl TryFrom<Box<dyn ValidatorDocument>> for SupportedDocument {
  type Error = WasmError<'static>;
  fn try_from(value: Box<dyn ValidatorDocument>) -> Result<Self, Self::Error> {
    let upcast = value.into_any();
    let supported_document = match upcast.downcast::<CoreDocument>() {
      Ok(doc) => SupportedDocument::Core(*doc),
      Err(retry) => {
        if let Ok(doc) = retry.downcast::<StardustDocument>() {
          SupportedDocument::Stardust(*doc)
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
  // TODO: Export CoreDocument to these bindings and change the definition below to Promise<Array<StardustDocument |
  // CoreDocument>>
  #[wasm_bindgen(typescript_type = "Promise<Array<StardustDocument>>")]
  pub type PromiseArraySupportedDocument;
}
