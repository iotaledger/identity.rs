// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::rc::Rc;

use identity_iota::credential::Presentation;
use identity_iota::credential::PresentationValidationOptions;
use identity_iota::credential::PresentationValidator;
use identity_iota::did::CoreDID;
use identity_iota::did::DID;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::resolver::SingleThreadedResolver;
use js_sys::Function;
use js_sys::Map;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;

use crate::common::ImportedDocumentLock;
use crate::common::ImportedDocumentReadGuard;
use crate::common::PromiseVoid;
use crate::credential::WasmFailFast;
use crate::credential::WasmPresentation;
use crate::credential::WasmPresentationValidationOptions;
use crate::did::ArrayIToCoreDocument;
use crate::error::JsValueResult;
use crate::error::WasmError;
use crate::iota::IotaDocumentLock;
use crate::iota::WasmIotaDID;
use crate::iota::WasmIotaDocument;
use crate::iota::WasmIotaIdentityClient;
use crate::resolver::constructor_input::MapResolutionHandler;
use crate::resolver::constructor_input::ResolverConfig;
use crate::resolver::type_definitions::OptionArrayIToCoreDocument;
use crate::resolver::type_definitions::OptionIToCoreDocument;

use super::type_definitions::PromiseArrayIToCoreDocument;
use super::type_definitions::PromiseIToCoreDocument;
use crate::error::Result;
use crate::error::WasmResult;
use futures::stream::StreamExt;
use futures::stream::{self};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

type JsDocumentResolver = SingleThreadedResolver<JsValue>;
/// Convenience type for resolving DID documents from different DID methods.   
///  
/// Also provides methods for resolving DID Documents associated with
/// verifiable `Credentials` and `Presentations`.
///
/// # Configuration
/// The resolver will only be able to resolve DID documents for methods it has been configured for in the constructor.
#[wasm_bindgen(js_name = Resolver)]
pub struct WasmResolver(Rc<JsDocumentResolver>);

#[wasm_bindgen(js_class = Resolver)]
impl WasmResolver {
  /// Constructs a new `Resolver`.
  ///
  /// # Errors
  /// If both a `client` is given and the `handlers` map contains the "iota" key the construction process
  /// will throw an error because the handler for the "iota" method then becomes ambiguous.
  #[wasm_bindgen(constructor)]
  pub fn new(config: ResolverConfig) -> Result<WasmResolver> {
    let mut resolver: JsDocumentResolver = SingleThreadedResolver::new();

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
          let iota_did: IotaDID = IotaDID::parse(did).map_err(identity_iota::iota::Error::DIDSyntaxError)?;
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
  ) -> std::result::Result<WasmIotaDocument, identity_iota::iota::Error> {
    Ok(WasmIotaDocument(Rc::new(IotaDocumentLock::new(
      client.resolve_did(&did.0).await?,
    ))))
  }

  /// attempts to extract (method, handler) pairs from the entries of a map and attaches them to the resolver.
  fn attach_handlers(resolver: &mut JsDocumentResolver, map: &Map, attached_iota_method: &mut bool) -> Result<()> {
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

        WasmResolver::attach_handler(resolver, did_method, handler);
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
  fn attach_handler(resolver: &mut JsDocumentResolver, method: String, handler: Function) {
    let fun = move |input: CoreDID| {
      let fun_clone: Function = handler.clone();
      async move {
        let closure_output_promise: Promise = Promise::resolve(
          &JsValueResult::from(fun_clone.call1(&JsValue::null(), &input.as_str().into())).stringify_error()?,
        );
        let awaited_output: JsValue =
          JsValueResult::from(JsFuture::from(closure_output_promise).await).stringify_error()?;

        std::result::Result::<_, String>::Ok(awaited_output)
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
  pub fn resolve_presentation_issuers(&self, presentation: &WasmPresentation) -> Result<PromiseArrayIToCoreDocument> {
    let resolver: Rc<JsDocumentResolver> = self.0.clone();
    let presentation: Presentation = presentation.0.clone();

    let promise: Promise = future_to_promise(async move {
      let issuer_documents: Vec<JsValue> = resolver
        .resolve_presentation_issuers(&presentation)
        .await
        .wasm_result()?;
      Ok(issuer_documents.into_iter().collect::<js_sys::Array>().into())
    });

    Ok(promise.unchecked_into::<PromiseArrayIToCoreDocument>())
  }

  /// Fetches the DID Document of the holder of a `Presentation`.
  ///
  /// # Errors
  /// Errors if the holder URL is missing, cannot be parsed to a valid DID whose method is supported by the resolver, or
  /// DID resolution fails.
  #[wasm_bindgen(js_name = resolvePresentationHolder)]
  pub fn resolve_presentation_holder(&self, presentation: &WasmPresentation) -> Result<PromiseIToCoreDocument> {
    let resolver: Rc<JsDocumentResolver> = self.0.clone();
    let presentation: Presentation = presentation.0.clone();

    let promise: Promise = future_to_promise(async move {
      resolver
        .resolve_presentation_holder(&presentation)
        .await
        .wasm_result()
        .map(JsValue::from)
    });
    Ok(promise.unchecked_into::<PromiseIToCoreDocument>())
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
    holder: &OptionIToCoreDocument,
    issuers: &OptionArrayIToCoreDocument,
  ) -> Result<PromiseVoid> {
    let resolver: Rc<JsDocumentResolver> = self.0.clone();
    let presentation: Presentation = presentation.0.clone();
    let options: PresentationValidationOptions = options.0.clone();

    let holder_clone: JsValue = JsValue::from(holder);
    let issuers_clone: JsValue = JsValue::from(issuers);

    let promise: Promise = future_to_promise(async move {
      let holder_handler = || async {
        if holder_clone.is_null() || holder_clone.is_undefined() {
          resolver
            .resolve_presentation_holder(&presentation)
            .await
            .map(|value| ImportedDocumentLock::from_js_value_unchecked(&value))
        } else {
          Ok(ImportedDocumentLock::from_js_value_unchecked(&holder_clone))
        }
      };

      let issuers_handler = || async {
        if issuers_clone.is_null() || issuers_clone.is_undefined() {
          resolver.resolve_presentation_issuers(&presentation).await.map(|value| {
            value
              .into_iter()
              .map(|entry| ImportedDocumentLock::from_js_value_unchecked(&entry))
              .collect::<Vec<ImportedDocumentLock>>()
          })
        } else {
          Ok(Vec::<ImportedDocumentLock>::from(
            issuers_clone.unchecked_ref::<ArrayIToCoreDocument>(),
          ))
        }
      };

      let (holder_result, issuers_result) = futures::join!(holder_handler(), issuers_handler());

      let holder = holder_result.wasm_result()?;
      let issuers = issuers_result.wasm_result()?;

      let holder_guard = holder.read().await;
      let issuers_guard: Vec<ImportedDocumentReadGuard<'_>> = stream::iter(issuers.iter())
        .then(ImportedDocumentLock::read)
        .collect()
        .await;

      PresentationValidator::validate(&presentation, &holder_guard, &issuers_guard, &options, fail_fast.into())
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
  pub fn resolve(&self, did: &str) -> Result<PromiseIToCoreDocument> {
    let resolver: Rc<JsDocumentResolver> = self.0.clone();
    let did: CoreDID = CoreDID::parse(did).wasm_result()?;

    let promise: Promise = future_to_promise(async move {
      resolver
        .resolve(&did)
        .await
        .map_err(WasmError::from)
        .map_err(JsValue::from)
        .map(JsValue::from)
    });

    Ok(promise.unchecked_into::<PromiseIToCoreDocument>())
  }
}
