// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_iota::credential::AbstractValidatorDocument;
use identity_iota::credential::Presentation;
use identity_iota::credential::PresentationValidationOptions;
use identity_iota::did::CoreDID;
use identity_iota::did::DID;
use identity_resolver::SingleThreadedResolver;
use js_sys::Function;
use js_sys::Map;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;

use crate::common::PromiseVoid;
use crate::credential::WasmFailFast;
use crate::credential::WasmPresentation;
use crate::credential::WasmPresentationValidationOptions;
use crate::error::JsValueResult;
use crate::error::WasmError;
use crate::resolver::supported_document_types::ArraySupportedDocument;
use crate::resolver::supported_document_types::MapResolutionHandler;
use crate::resolver::supported_document_types::RustSupportedDocument;
use crate::resolver::supported_document_types::SupportedDocument;

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
  /// Constructs a new `MixedResolver`.
  #[wasm_bindgen(constructor)]
  pub fn new(handlers: &MapResolutionHandler) -> Result<MixedResolver> {
    let mut resolver = SingleThreadedResolver::new();
    let map: &Map = handlers
      .dyn_ref::<js_sys::Map>()
      .ok_or("could not construct resolver: the constructor did not receive a map of asynchronous functions")?;

    for key in map.keys() {
      if let Ok(mthd) = key {
        let fun = map.get(&mthd);
        let method: String = mthd
          .as_string()
          .ok_or("could not construct resolver: the handler map contains a key which is not a string")?;
        let handler: Function = fun
          .dyn_into::<Function>()
          .map_err(|_| "could not construct resolver: the handler map contains a value which is not a function")?;
        MixedResolver::attach_handler(&mut resolver, method, handler);
      } else {
        Err("could not construct resolver: invalid constructor arguments. Expected a map of asynchronous functions")?;
      }
    }
    Ok(Self(Rc::new(resolver)))
  }

  fn attach_handler(resolver: &mut SingleThreadedResolver, method: String, handler: Function) {
    let fun_closure_clone = handler;
    let fun = move |input: CoreDID| {
      let fun_clone = fun_closure_clone.clone();
      async move {
        let closure_output_promise: Promise = Promise::resolve(
          &JsValueResult::from(fun_clone.call1(&JsValue::null(), &input.as_str().into())).stringify_error()?,
        );
        let awaited_output = JsValueResult::from(JsFuture::from(closure_output_promise).await).stringify_error()?;

        let supported_document: RustSupportedDocument = awaited_output.into_serde().map_err(|error| {
          format!(
            "resolution succeeded, but could not convert the outcome into a supported DID Document: {}",
            error.to_string()
          )
        })?;
        std::result::Result::<_, String>::Ok(AbstractValidatorDocument::from(supported_document))
      }
    };
    resolver.attach_handler(method, fun);
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
          .map(JsValue::from)
          .collect::<js_sys::Array>()
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

    let promise: Promise = future_to_promise(async move {
      resolver
        .resolve_presentation_holder(&presentation)
        .await
        .wasm_result()
        .and_then(|abstract_doc| RustSupportedDocument::try_from(abstract_doc).map_err(JsValue::from))
        .map(JsValue::from)
    });
    Ok(promise.unchecked_into::<PromiseSupportedDocument>())
  }

  /// Verifies a `Presentation`.
  ///
  /// ### Important
  /// See `PresentationValidator::validate` for information about which properties get
  /// validated and what is expected of the optional arguments `holder` and `issuer`.
  ///
  /// ### Resolution
  /// The DID Documents for the `holder` and `issuers` are optionally resolved if not given.
  /// If you already have up-to-date versions of these DID Documents, you may want
  /// to use `PresentationValidator::validate`.
  /// See also `Resolver::resolvePresentationIssuers` and `Resolver::resolvePresentationHolder`.
  ///
  /// ### Errors
  /// Errors from resolving the holder and issuer DID Documents, if not provided, will be returned immediately.
  /// Otherwise, errors from validating the presentation and its credentials will be returned
  /// according to the `fail_fast` parameter.
  #[wasm_bindgen(js_name = verifyPresentation)]
  pub fn verify_presentation(
    &self,
    presentation: &WasmPresentation,
    options: &WasmPresentationValidationOptions,
    fail_fast: WasmFailFast,
    holder: Option<SupportedDocument>,
    issuers: Option<ArraySupportedDocument>,
  ) -> Result<PromiseVoid> {
    let resolver: Rc<SingleThreadedResolver> = self.0.clone();
    let presentation: Presentation = presentation.0.clone();
    let options: PresentationValidationOptions = options.0.clone();

    let holder: Option<RustSupportedDocument> = holder.map(|js| js.into_serde().wasm_result()).transpose()?;
    let holder: Option<AbstractValidatorDocument> = holder.map(From::from);
    let issuers: Option<Vec<RustSupportedDocument>> = issuers.map(|js| js.into_serde().wasm_result()).transpose()?;
    let issuers: Option<Vec<AbstractValidatorDocument>> =
      issuers.map(|vector| vector.into_iter().map(AbstractValidatorDocument::from).collect());

    let promise: Promise = future_to_promise(async move {
      resolver
        .verify_presentation(
          &presentation,
          &options,
          fail_fast.into(),
          (holder.as_ref()).as_deref(),
          issuers.as_deref(),
        )
        .await
        .wasm_result()
        .map_err(JsValue::from)
        .map(|_| JsValue::UNDEFINED)
    });

    Ok(promise.unchecked_into::<PromiseVoid>())
  }

  #[wasm_bindgen]
  pub fn resolve(&self, did: &str) -> Result<PromiseSupportedDocument> {
    let resolver: Rc<SingleThreadedResolver> = self.0.clone();
    let did: CoreDID = CoreDID::parse(did).wasm_result()?;

    let promise: Promise = future_to_promise(async move {
      resolver
        .resolve(&did)
        .await
        .map_err(WasmError::from)
        .and_then(RustSupportedDocument::try_from)
        .map_err(JsValue::from)
        .map(JsValue::from)
    });

    Ok(promise.unchecked_into::<PromiseSupportedDocument>())
  }
}
