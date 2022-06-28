// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::cell::RefCell;
use std::rc::Rc;

use identity_iota::account::AccountBuilder;
use identity_iota::account::IdentitySetup;
use identity_iota::client::Client;
use identity_iota::client::ClientBuilder;
use identity_iota::iota_core::IotaDID;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::account::storage::WasmStorage;
use crate::account::types::OptionAutoSave;
use crate::account::types::WasmIdentitySetup;
use crate::account::wasm_account::PromiseAccount;
use crate::account::wasm_account::WasmAccount;
use crate::did::WasmDID;
use crate::error::Result;
use crate::error::WasmResult;
use crate::tangle::IClientConfig;

type AccountBuilderRc = AccountBuilder<Rc<Client>>;

/// An [`Account`] builder for easy account configuration.
///
/// To reduce memory usage, accounts created from the same builder share the same `Storage`
/// used to store identities, and the same {@link Client} used to publish identities to the Tangle.
///
/// The configuration on the other hand is cloned, and therefore unique for each built account.
/// This means a builder can be reconfigured in-between account creations, without affecting
/// the configuration of previously built accounts.
#[wasm_bindgen(js_name = AccountBuilder)]
pub struct WasmAccountBuilder(Rc<RefCell<AccountBuilderRc>>);

#[wasm_bindgen(js_class = AccountBuilder)]
impl WasmAccountBuilder {
  /// Creates a new `AccountBuilder`.
  #[wasm_bindgen(constructor)]
  pub fn new(options: Option<AccountBuilderOptions>) -> Result<WasmAccountBuilder> {
    let mut builder: AccountBuilderRc = AccountBuilderRc::new();

    if let Some(builder_options) = options {
      if let Some(autopublish) = builder_options.autopublish() {
        builder = builder.autopublish(autopublish);
      }

      if let Some(autosave) = builder_options.autosave().into_serde().wasm_result()? {
        builder = builder.autosave(autosave);
      }

      if let Some(config) = builder_options.clientConfig() {
        let client_builder: ClientBuilder = ClientBuilder::try_from(config)?;
        builder = builder.client_builder(client_builder);
      };

      if let Some(storage) = builder_options.storage() {
        builder = builder.storage(storage);
      }
    }

    Ok(Self(Rc::new(RefCell::new(builder))))
  }

  /// Loads an existing identity with the specified `did` using the current builder configuration.
  /// The identity must exist in the configured `Storage`.
  #[wasm_bindgen(js_name = loadIdentity)]
  pub fn load_identity(&mut self, did: &WasmDID) -> Result<PromiseAccount> {
    let builder: Rc<RefCell<AccountBuilderRc>> = self.0.clone();
    let did: IotaDID = did.0.clone();
    let promise: Promise = future_to_promise(async move {
      builder
        .as_ref()
        .borrow_mut()
        .load_identity(did)
        .await
        .map(WasmAccount::from)
        .map(Into::into)
        .wasm_result()
    });
    Ok(promise.unchecked_into::<PromiseAccount>())
  }

  /// Creates a new identity based on the builder configuration and returns
  /// an {@link Account} object to manage it.
  ///
  /// The identity is stored locally in the `Storage`. The DID network is automatically determined
  /// by the {@link Client} used to publish it.
  ///
  #[wasm_bindgen(js_name = createIdentity)]
  pub fn create_identity(&mut self, identity_setup: Option<WasmIdentitySetup>) -> Result<PromiseAccount> {
    let setup: IdentitySetup = identity_setup.map(IdentitySetup::from).unwrap_or_default();

    let builder: Rc<RefCell<AccountBuilderRc>> = self.0.clone();
    let promise: Promise = future_to_promise(async move {
      builder
        .as_ref()
        .borrow_mut()
        .create_identity(setup)
        .await
        .map(WasmAccount::from)
        .map(Into::into)
        .wasm_result()
    });
    Ok(promise.unchecked_into::<PromiseAccount>())
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "AccountBuilderOptions")]
  pub type AccountBuilderOptions;

  #[wasm_bindgen(getter, method)]
  pub fn autopublish(this: &AccountBuilderOptions) -> Option<bool>;

  #[wasm_bindgen(getter, method)]
  pub fn clientConfig(this: &AccountBuilderOptions) -> Option<IClientConfig>;

  #[wasm_bindgen(getter, method)]
  pub fn milestone(this: &AccountBuilderOptions) -> Option<u32>;

  #[wasm_bindgen(getter, method)]
  pub fn autosave(this: &AccountBuilderOptions) -> OptionAutoSave;

  #[wasm_bindgen(getter, method)]
  pub fn storage(this: &AccountBuilderOptions) -> Option<WasmStorage>;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_ACCOUNT_BUILDER_OPTIONS: &'static str = r#"
/**
 * Options for creating a new {@link AccountBuilder}.
 */
export type AccountBuilderOptions = {

    /**
     * When the account will store its state to the storage.
     */
    autosave?: AutoSave

    /**
     * `autopublish == true` the account will publish messages to the tangle on each update.
     * `autopublish == false` the account will combine and publish message when .publish() is called.
     */
    autopublish?: boolean,

    /**
     * Client for tangle requests.
     */
    clientConfig?: IClientConfig,

    /**
     * The Storage implemantation to use for each account built by this builder.
     */
    storage?: Storage
};
"#;
