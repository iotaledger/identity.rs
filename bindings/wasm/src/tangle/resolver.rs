// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use std::rc::Rc;

use futures::executor;
use identity::core::Url;
use identity::iota::Client;
use identity::iota::ClientBuilder;
use identity::iota::ResolvedIotaDocument;
use identity::iota::Resolver;
use identity::iota::ResolverBuilder;
use identity::iota_core::IotaDID;
use identity::iota_core::NetworkName;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::chain::DiffChainHistory;
use crate::chain::PromiseDiffChainHistory;
use crate::chain::PromiseDocumentHistory;
use crate::chain::WasmDocumentHistory;
use crate::credential::WasmCredential;
use crate::credential::WasmFailFast;
use crate::credential::WasmPresentation;
use crate::credential::WasmPresentationValidationOptions;
use crate::did::PromiseArrayResolvedDocument;
use crate::did::PromiseResolvedDocument;
use crate::did::UWasmDID;
use crate::did::WasmResolvedDocument;
use crate::error::Result;
use crate::error::WasmResult;
use crate::tangle::IClientConfig;
use crate::tangle::WasmClient;

#[wasm_bindgen(js_name = Resolver)]
pub struct WasmResolver(pub(crate) Rc<Resolver<Rc<Client>>>);

#[wasm_bindgen]
extern "C" {
  // Workaround for Typescript type annotations on async function returns.
  #[wasm_bindgen(typescript_type = "Promise<void>")]
  pub type PromiseVoid;

  #[wasm_bindgen(typescript_type = "ResolvedDocument | undefined")]
  pub type OptionResolvedDocument;

  #[wasm_bindgen(typescript_type = "ResolvedDocument[] | undefined")]
  pub type OptionArrayResolvedDocument;
}

#[wasm_bindgen(js_class = Resolver)]
impl WasmResolver {
  /// Constructs a new `Resolver` with a default `Client` for
  /// the `Mainnet`.
  #[wasm_bindgen(constructor)]
  pub fn new() -> Result<WasmResolver> {
    // TODO: any way to avoid blocking? WasmClient does this too...
    executor::block_on(Resolver::<Rc<Client>>::new())
      .map(Self::from)
      .wasm_result()
  }

  /// Returns a {@link ResolverBuilder} to construct a new `Resolver`.
  #[wasm_bindgen]
  pub fn builder() -> WasmResolverBuilder {
    WasmResolverBuilder::new()
  }

  /// Returns the `Client` corresponding to the given network name if one exists.
  #[wasm_bindgen(js_name = getClient)]
  pub fn get_client(&self, network_name: String) -> Option<WasmClient> {
    let network_name: NetworkName = NetworkName::try_from(network_name).ok()?;
    self.0.get_client(&network_name).cloned().map(WasmClient::from)
  }

  /// Fetches the `Document` of the given `DID`.
  #[wasm_bindgen]
  pub fn resolve(&self, did: UWasmDID) -> Result<PromiseResolvedDocument> {
    let did: IotaDID = IotaDID::try_from(did)?;

    let resolver: Rc<Resolver<Rc<Client>>> = Rc::clone(&self.0);
    let promise: Promise = future_to_promise(async move {
      resolver
        .resolve(&did)
        .await
        .map(WasmResolvedDocument::from)
        .map(Into::into)
        .wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseResolvedDocument>())
  }

  /// Fetches the `DocumentHistory` of the given `DID`.
  #[wasm_bindgen(js_name = resolveHistory)]
  pub fn resolve_history(&self, did: UWasmDID) -> Result<PromiseDocumentHistory> {
    let did: IotaDID = IotaDID::try_from(did)?;

    let resolver: Rc<Resolver<Rc<Client>>> = Rc::clone(&self.0);
    let promise: Promise = future_to_promise(async move {
      resolver
        .resolve_history(&did)
        .await
        .map(WasmDocumentHistory::from)
        .map(Into::into)
        .wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseDocumentHistory>())
  }

  /// Returns the `DiffChainHistory` of a diff chain starting from a `Document` on the
  /// integration chain.
  ///
  /// NOTE: the document must have been published to the Tangle and have a valid message id.
  #[wasm_bindgen(js_name = resolveDiffHistory)]
  pub fn resolve_diff_history(&self, document: &WasmResolvedDocument) -> Result<PromiseDiffChainHistory> {
    let resolved_document: ResolvedIotaDocument = document.0.clone();

    let resolver: Rc<Resolver<Rc<Client>>> = Rc::clone(&self.0);
    let promise: Promise = future_to_promise(async move {
      resolver
        .resolve_diff_history(&resolved_document)
        .await
        .map(DiffChainHistory::from)
        .map(Into::into)
        .wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseDiffChainHistory>())
  }

  /// Fetches the DID Document of the issuer on a `Credential`.
  ///
  /// ### Errors
  ///
  /// Errors if the issuer URL is not a valid `DID` or document resolution fails.
  #[wasm_bindgen(js_name = resolveCredentialIssuer)]
  pub fn resolve_credential_issuer(&self, credential: &WasmCredential) -> Result<PromiseResolvedDocument> {
    // TODO: reimplemented function to avoid cloning the entire credential.
    //       Would be solved with Rc internal representation, pending memory leak discussions.
    let issuer: IotaDID = IotaDID::parse(credential.0.issuer.url().as_str()).wasm_result()?;

    let resolver: Rc<Resolver<Rc<Client>>> = Rc::clone(&self.0);
    let promise: Promise = future_to_promise(async move {
      resolver
        .resolve(&issuer)
        .await
        .map(WasmResolvedDocument::from)
        .map(Into::into)
        .wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseResolvedDocument>())
  }

  /// Fetches all DID Documents of `Credential` issuers contained in a `Presentation`.
  /// Issuer documents are returned in arbitrary order.
  ///
  /// ### Errors
  ///
  /// Errors if any issuer URL is not a valid `DID` or document resolution fails.
  #[wasm_bindgen(js_name = resolvePresentationIssuers)]
  pub fn resolve_presentation_issuers(&self, presentation: &WasmPresentation) -> Result<PromiseArrayResolvedDocument> {
    // TODO: reimplemented function to avoid cloning the entire presentation.
    //       Would be solved with Rc internal representation, pending memory leak discussions.

    // Extract unique issuers.
    let issuers: HashSet<IotaDID> = presentation
      .0
      .verifiable_credential
      .iter()
      .map(|credential| IotaDID::parse(credential.issuer.url().as_str()).wasm_result())
      .collect::<Result<_>>()?;

    let resolver: Rc<Resolver<Rc<Client>>> = Rc::clone(&self.0);
    let promise: Promise = future_to_promise(async move {
      // Resolve issuers concurrently.
      Ok(
        futures::future::try_join_all(
          issuers
            .iter()
            .map(|issuer| resolver.resolve(issuer))
            .collect::<Vec<_>>(),
        )
        .await
        .wasm_result()?
        .into_iter()
        .map(WasmResolvedDocument::from)
        .map(JsValue::from)
        .collect::<js_sys::Array>()
        .into(),
      )
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseArrayResolvedDocument>())
  }

  /// Fetches the DID Document of the holder of a `Presentation`.
  ///
  /// ### Errors
  ///
  /// Errors if the holder URL is missing, is not a valid `DID`, or document resolution fails.
  #[wasm_bindgen(js_name = resolvePresentationHolder)]
  pub fn resolve_presentation_holder(&self, presentation: &WasmPresentation) -> Result<PromiseResolvedDocument> {
    // TODO: reimplemented function to avoid cloning the entire presentation.
    //       Would be solved with Rc internal representation, pending memory leak discussions.
    let holder_url: &Url = presentation
      .0
      .holder
      .as_ref()
      .ok_or(identity::iota::ValidationError::MissingPresentationHolder)
      .map_err(identity::iota::Error::from)
      .wasm_result()?;
    let holder: IotaDID = IotaDID::parse(holder_url.as_str()).wasm_result()?;

    let resolver: Rc<Resolver<Rc<Client>>> = Rc::clone(&self.0);
    let promise: Promise = future_to_promise(async move {
      resolver
        .resolve(&holder)
        .await
        .map(WasmResolvedDocument::from)
        .map(Into::into)
        .wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseResolvedDocument>())
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
    holder: OptionResolvedDocument,
    issuers: OptionArrayResolvedDocument,
  ) -> Result<PromiseVoid> {
    // TODO: reimplemented function to avoid cloning the entire presentation and validation options.
    // Would be solved with Rc internal representation, pending memory leak discussions.
    let issuers: Option<Vec<ResolvedIotaDocument>> = issuers.into_serde().wasm_result()?;
    let holder: Option<ResolvedIotaDocument> = holder.into_serde().wasm_result()?;
    let resolver: Rc<Resolver<Rc<Client>>> = Rc::clone(&self.0);
    let presentation: WasmPresentation = presentation.clone();
    let options: WasmPresentationValidationOptions = options.clone();

    let promise: Promise = future_to_promise(async move {
      resolver
        .verify_presentation(
          &presentation.0,
          &options.0,
          fail_fast.into(),
          holder.as_ref(),
          issuers.as_deref(),
        )
        .await
        .map(|_| JsValue::UNDEFINED)
        .wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseVoid>())
  }
}

impl From<Resolver<Rc<Client>>> for WasmResolver {
  fn from(resolver: Resolver<Rc<Client>>) -> Self {
    WasmResolver(Rc::new(resolver))
  }
}

/// Builder for configuring [`Clients`][Client] when constructing a [`Resolver`].
#[wasm_bindgen(js_name = ResolverBuilder)]
pub struct WasmResolverBuilder(ResolverBuilder<Rc<Client>>);

#[allow(clippy::new_without_default)]
#[wasm_bindgen(js_class = ResolverBuilder)]
impl WasmResolverBuilder {
  /// Constructs a new `ResolverBuilder` with no `Clients` configured.
  #[wasm_bindgen(constructor)]
  pub fn new() -> WasmResolverBuilder {
    WasmResolverBuilder(ResolverBuilder::<Rc<Client>>::new())
  }

  /// Inserts a `Client`.
  ///
  /// NOTE: replaces any previous `Client` or `Config` with the same network name.
  #[wasm_bindgen]
  #[must_use]
  pub fn client(mut self, client: &WasmClient) -> WasmResolverBuilder {
    self.0 = self.0.client(Rc::clone(&client.client));
    self
  }

  /// Inserts a `Config` used to create a `Client`.
  ///
  /// NOTE: replaces any previous `Client` or `Config` with the same network name.
  #[wasm_bindgen(js_name = clientConfig)]
  pub fn client_config(mut self, config: IClientConfig) -> Result<WasmResolverBuilder> {
    self.0 = self.0.client_builder(ClientBuilder::try_from(config)?);
    Ok(self)
  }

  /// Constructs a new [`Resolver`] based on the builder configuration.
  #[wasm_bindgen]
  pub fn build(self) -> PromiseResolver {
    // WARNING: this does not validate the return type. Check carefully.
    future_to_promise(async move {
      self
        .0
        .build()
        .await
        .map(WasmResolver::from)
        .map(Into::into)
        .wasm_result()
    })
    .unchecked_into::<PromiseResolver>()
  }
}

impl From<ResolverBuilder<Rc<Client>>> for WasmResolverBuilder {
  fn from(builder: ResolverBuilder<Rc<Client>>) -> Self {
    Self(builder)
  }
}

// Workaround for Typescript type annotations on async function returns.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Resolver>")]
  pub type PromiseResolver;
}
