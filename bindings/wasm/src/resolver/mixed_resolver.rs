// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_iota::credential::Presentation;
use identity_iota::credential::AbstractValidatorDocument;
use identity_iota::did::CoreDID;
use identity_iota::did::DID;
use identity_iota::resolver::SingleThreadedResolver;
use js_sys::Function;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;

use crate::credential::WasmPresentation;
use crate::error::ErrorString;
use crate::error::JsValueResult;
use crate::error::WasmError;
use crate::resolver::supported_document_types::RustSupportedDocument;

//use super::function_transformation::WasmResolverCommand;
use super::supported_document_types::PromiseArraySupportedDocument;
use super::supported_document_types::PromiseSupportedDocument;
use crate::error::Result;
use crate::error::WasmResult;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_name = MixedResolver)]
pub struct MixedResolver(Rc<SingleThreadedResolver>);

#[wasm_bindgen(js_class = MixedResolver)]
impl MixedResolver {
  /// Constructs a new [`MixedResolver`].
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self(Rc::new(SingleThreadedResolver::new()))
  }

  #[wasm_bindgen(js_name = attachHandler)]
  //TODO: Fix the potential panic below. This can be enforced using something like the builder pattern.
  pub fn attach_handler(&mut self, method: &str, handler: &Function) {
    let fun_closure_clone = handler.clone();
    let fun = move |input: CoreDID| {
      let fun_clone = fun_closure_clone.clone();
      async move {
        let closure_output_promise: Promise = Promise::resolve(
          &JsValueResult::from(fun_clone.call1(&JsValue::null(), &input.as_str().into())).stringify_error()?,
        );
        let awaited_output = JsValueResult::from(JsFuture::from(closure_output_promise).await).stringify_error()?;

        let supported_document: RustSupportedDocument = awaited_output.into_serde().map_err(|error| {
          ErrorString(format!(
            "resolution succeeded, but could not convert the outcome into a supported DID Document: {}",
            error.to_string()
          ))
        })?;
        std::result::Result::<_, ErrorString>::Ok(AbstractValidatorDocument::from(supported_document))
      }
    };

    Rc::<SingleThreadedResolver>::get_mut(&mut self.0)
      .expect("The resolver should be setup before being used")
      .attach_handler(method.to_owned(), fun);
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
    let resolver: Rc<SingleThreadedResolver> = self.0.clone();
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


  /// Fetches the DID Document of the holder of a [`Presentation`].
  ///
  /// # Errors
  ///
  /// Errors if the holder URL is missing, cannot be parsed to a valid DID whose method is supported by the resolver, or
  /// DID resolution fails.
  #[wasm_bindgen(js_name = resolvePresentationHolder)]
  pub fn resolve_presentation_holder(&self, presentation: &WasmPresentation) -> Result<PromiseSupportedDocument> {
    let resolver: Rc<SingleThreadedResolver> = self.0.clone();
    let presentation: Presentation = presentation.0.clone();

    let promise: Promise = future_to_promise( async move {
      resolver.resolve_presentation_holder(&presentation).await
      .wasm_result()
      .and_then(|abstract_doc| RustSupportedDocument::try_from(abstract_doc).map_err(JsValue::from))?
      .to_json()
    });
    Ok(promise.unchecked_into::<PromiseSupportedDocument>())
  }

  #[wasm_bindgen]
  pub fn resolve(&self, did: &str) -> Result<PromiseSupportedDocument> {
    let resolver: Rc<SingleThreadedResolver> = self.0.clone();
    let did: CoreDID = CoreDID::parse(did).wasm_result()?;

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
