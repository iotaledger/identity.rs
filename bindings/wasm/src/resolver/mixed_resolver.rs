// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_iota::credential::Presentation;
use identity_iota::did::CoreDID;
use identity_iota::resolver::Resolver;
use js_sys::Function;
use js_sys::Promise;

use crate::credential::WasmPresentation;
use crate::did::WasmCoreDID;
use crate::error::WasmError;
use crate::resolver::supported_document_types::RustSupportedDocument;

use super::function_transformation::WasmResolverCommand;
use super::supported_document_types::PromiseArraySupportedDocument;
use super::supported_document_types::PromiseSupportedDocument;
use crate::error::Result;
use crate::error::WasmResult;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_name = MixedResolver)]
pub struct MixedResolver(Rc<Resolver>);

#[wasm_bindgen(js_class = MixedResolver)]
impl MixedResolver {
  /// Constructs a new [`MixedResolver`].
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self(Rc::new(Resolver::new()))
  }

  #[wasm_bindgen]
  //TODO: Fix the potential panic below. This can be enforced using something like the builder pattern.
  pub fn attach_handler(&mut self, method: &str, handler: &Function) {
    let command = WasmResolverCommand::new(handler).ptr;
    Rc::<identity_iota::resolver::Resolver>::get_mut(&mut self.0)
      .expect("The resolver should be setup before being used")
      .attach_raw(method.to_string(), command);
  }

  /// Fetches all DID Documents of [`Credential`] issuers contained in a [`Presentation`].
  /// Issuer documents are returned in arbitrary order.
  ///
  /// # Errors
  ///
  /// Errors if any issuer URL cannot be parsed to a DID whose associated method is supported by this Resolver, or
  /// resolution fails.
  // TODO: Improve error handling.
  #[wasm_bindgen(js_name = resolvePresentationIssuers)]
  pub fn resolve_presentation_issuers(&self, presentation: &WasmPresentation) -> Result<PromiseArraySupportedDocument> {
    let resolver: Rc<Resolver> = self.0.clone();
    let presentation: Presentation = presentation.0.clone();

    let promise: Promise = future_to_promise(async move {
      let supported_documents: Vec<RustSupportedDocument> = resolver
        .resolve_presentation_issuers(&presentation)
        .await
        .wasm_result()
        .and_then(|abstractly_resolved| {
          abstractly_resolved
            .into_iter()
            .map(|abstract_doc| RustSupportedDocument::try_from(abstract_doc).map_err(JsValue::from))
            .collect::<Result<_>>()
        })?;

      Ok(
        supported_documents
          .into_iter()
          .map(|doc| doc.to_json())
          .collect::<Result<js_sys::Array>>()?
          .into(),
      )
    });
    Ok(promise.unchecked_into::<PromiseArraySupportedDocument>())
  }

  #[wasm_bindgen]
  pub fn resolve(&self, did: &WasmCoreDID) -> Result<PromiseSupportedDocument> {
    let resolver: Rc<Resolver> = self.0.clone();
    let did: CoreDID = did.0.clone();

    let promise: Promise = future_to_promise(async move {
      Ok(
        resolver
          .resolve(&did)
          .await
          .map_err(WasmError::from)
          .and_then(RustSupportedDocument::try_from)?
          .to_json()?,
      )
    });

    Ok(promise.unchecked_into::<PromiseSupportedDocument>())
  }
}
