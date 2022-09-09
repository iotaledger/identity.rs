// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::rc::Rc;

use identity_iota::credential::AbstractValidatorDocument;
use identity_iota::credential::Presentation;
use identity_iota::credential::PresentationValidationOptions;
use identity_iota::did::CoreDID;
use identity_iota::did::DID;
use identity_iota::iota_core::IotaDID;
use identity_iota::iota_core::IotaDocument;
use identity_iota::iota_core::IotaIdentityClientExt;
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
use crate::iota::WasmIotaDID;
use crate::iota::WasmIotaIdentityClient;
use crate::resolver::constructor_input::MapResolutionHandler;
use crate::resolver::constructor_input::ResolverConfig;
use crate::resolver::supported_document_types::OptionArraySupportedDocument;
use crate::resolver::supported_document_types::OptionSupportedDocument;
use crate::resolver::supported_document_types::RustSupportedDocument;

use super::supported_document_types::PromiseArraySupportedDocument;
use super::supported_document_types::PromiseSupportedDocument;
use crate::error::Result;
use crate::error::WasmResult;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

/// Convenience type for resolving DID documents from different DID methods.   
///  
/// Also provides methods for resolving DID Documents associated with
/// verifiable `Credentials` and `Presentations`.
///
/// # Configuration
/// The resolver will only be able to resolve DID documents for methods it has been configured for in the constructor.
#[wasm_bindgen(js_name = MixedResolver)]
pub struct MixedResolver(Rc<SingleThreadedResolver>);

#[wasm_bindgen(js_class = MixedResolver)]
impl MixedResolver {
  /// Constructs a new `MixedResolver`.
  ///
  /// # Errors
  /// If both a `client` is given and the `handlers` map contains the "iota" key the construction process
  /// will throw an error as it is then ambiguous what should be .
  #[wasm_bindgen(constructor)]
  pub fn new(config: ResolverConfig) -> Result<MixedResolver> {
    let mut resolver: SingleThreadedResolver = SingleThreadedResolver::new();

    let mut attached_iota_method = false;
    let resolution_handlers: Option<MapResolutionHandler> = config.handlers();
    let client: Option<WasmIotaIdentityClient> = config.client();

    if let Some(handlers) = resolution_handlers {
      let map: &Map = handlers.dyn_ref::<js_sys::Map>().ok_or_else(|| {
        WasmError::new(
          Cow::Borrowed("ResolverError::ConstructionError"),
          Cow::Borrowed(
            "could not construct resolver: the constructor did not receive a map of asynchronous functions",
          ),
        )
      })?;

      Self::attach_handlers(&mut resolver, map, &mut attached_iota_method)?;
    }

    if let Some(wasm_client) = client {
      if attached_iota_method {
        Err(WasmError::new(
          Cow::Borrowed("ResolverError::ConstructionError"),
          Cow::Borrowed("could not construct resolver: cannot attach the iota method twice"),
        ))?;
      }

      let rc_client: Rc<WasmIotaIdentityClient> = Rc::new(wasm_client);
      // Take CoreDID (instead of IotaDID) to avoid inconsistent error messages between the
      // cases when the iota handler is attached by passing a client or directly as a handler.
      let handler = move |did: CoreDID| {
        let rc_client_clone: Rc<WasmIotaIdentityClient> = rc_client.clone();
        async move {
          let iota_did: IotaDID = IotaDID::parse(did)?;
          Self::client_as_handler(rc_client_clone.as_ref(), iota_did.into()).await
        }
      };
      resolver.attach_handler(IotaDID::METHOD.to_owned(), handler);
    }

    Ok(Self(Rc::new(resolver)))
  }

  pub(crate) async fn client_as_handler(
    client: &WasmIotaIdentityClient,
    did: WasmIotaDID,
  ) -> std::result::Result<IotaDocument, identity_iota::iota_core::Error> {
    client.resolve_did(&did.0).await
  }

  /// attempts to extract (method, handler) pairs from the entries of a map and attaches them to the resolver.
  fn attach_handlers(resolver: &mut SingleThreadedResolver, map: &Map, attached_iota_method: &mut bool) -> Result<()> {
    for key in map.keys() {
      if let Ok(js_method) = key {
        let js_handler: JsValue = map.get(&js_method);
        let did_method: String = js_method.as_string().ok_or_else(|| {
          WasmError::new(
            Cow::Borrowed("ResolverError::ConstructionError"),
            Cow::Borrowed("could not construct resolver: the handler map contains a key which is not a string"),
          )
        })?;
        let handler: Function = js_handler
          .dyn_into::<Function>()
          .map_err(|_| "could not construct resolver: the handler map contains a value which is not a function")?;

        if did_method == IotaDID::METHOD {
          *attached_iota_method = true;
        }

        MixedResolver::attach_handler(resolver, did_method, handler);
      } else {
        Err(WasmError::new(
          Cow::Borrowed("ResolverError::ConstructionError"),
          Cow::Borrowed(
            "could not construct resolver: invalid constructor arguments. Expected a map of asynchronous functions",
          ),
        ))?;
      }
    }

    Ok(())
  }

  /// Converts a JS handler to a Rust closure and attaches it to the given `resolver`.
  fn attach_handler(resolver: &mut SingleThreadedResolver, method: String, handler: Function) {
    let fun = move |input: CoreDID| {
      let fun_clone: Function = handler.clone();
      async move {
        let closure_output_promise: Promise = Promise::resolve(
          &JsValueResult::from(fun_clone.call1(&JsValue::null(), &input.as_str().into())).stringify_error()?,
        );
        let awaited_output: JsValue =
          JsValueResult::from(JsFuture::from(closure_output_promise).await).stringify_error()?;

        let supported_document: RustSupportedDocument = awaited_output.into_serde().map_err(|error| {
          format!(
            "unable to convert the resolved document into a supported DID document: {}",
            error
          )
        })?;
        std::result::Result::<_, String>::Ok(AbstractValidatorDocument::from(supported_document))
      }
    };

    resolver.attach_handler(method, fun);
  }

  /// Fetches all DID Documents of `Credential` issuers contained in a `Presentation`.
  /// Issuer documents are returned in arbitrary order.
  ///
  /// # Errors
  /// Errors if any issuer URL cannot be parsed to a DID whose associated method is supported by this Resolver, or
  /// resolution fails.
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

  /// Fetches the DID Document of the holder of a `Presentation`.
  ///
  /// # Errors
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
    holder: &OptionSupportedDocument,
    issuers: &OptionArraySupportedDocument,
  ) -> Result<PromiseVoid> {
    let resolver: Rc<SingleThreadedResolver> = self.0.clone();
    let presentation: Presentation = presentation.0.clone();
    let options: PresentationValidationOptions = options.0.clone();

    let holder: Option<RustSupportedDocument> = holder.into_serde().wasm_result()?;
    let holder: Option<AbstractValidatorDocument> = holder.map(From::from);
    let issuers: Option<Vec<RustSupportedDocument>> = issuers.into_serde().wasm_result()?;
    let issuers: Option<Vec<AbstractValidatorDocument>> =
      issuers.map(|vector| vector.into_iter().map(AbstractValidatorDocument::from).collect());

    let promise: Promise = future_to_promise(async move {
      resolver
        .verify_presentation(
          &presentation,
          &options,
          fail_fast.into(),
          holder.as_ref(),
          issuers.as_deref(),
        )
        .await
        .wasm_result()
        .map(|_| JsValue::UNDEFINED)
    });

    Ok(promise.unchecked_into::<PromiseVoid>())
  }

  /// Fetches the DID Document of the given DID.
  ///
  /// ### Errors
  ///
  /// Errors if the resolver has not been configured to handle the method
  /// corresponding to the given DID or the resolution process itself fails.
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
