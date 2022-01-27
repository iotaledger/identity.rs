// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::method_secret::WasmMethodSecret;
use crate::account::wasm_account::WasmAccount;
use crate::crypto::KeyType;
use crate::did::WasmDID;
use crate::error::Result;
use crate::error::WasmResult;
use crate::tangle::{Client as WasmClient, Config};

use identity::account::AccountBuilder;
use identity::account::AccountConfig;
use identity::account::AutoSave;
use identity::account::IdentitySetup;

use js_sys::Promise;
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use crate::account::auto_save::WasmAutoSave;

/// An [`Account`] builder for easy account configuration.
///
/// To reduce memory usage, accounts created from the same builder share the same `Storage`
/// used to store identities, and the same {@link Client} used to publish identities to the Tangle.
///
/// The configuration on the other hand is cloned, and therefore unique for each built account.
/// This means a builder can be reconfigured in-between account creations, without affecting
/// the configuration of previously built accounts.
#[wasm_bindgen(js_name = AccountBuilder)]
pub struct WasmAccountBuilder(Rc<WasmRefCell<AccountBuilder>>);

#[wasm_bindgen(js_class = AccountBuilder)]
impl WasmAccountBuilder {
  /// Creates a new `AccountBuilder`.
  #[wasm_bindgen(constructor)]
  pub fn new(options: Option<AccountBuilderOptions>) -> Result<WasmAccountBuilder> {
    let default_config: AccountConfig = AccountConfig::default();
    let mut builder = AccountBuilder::new();

    if let Some(o) = options {
      builder = builder
        .autopublish(o.autopublish().unwrap_or(default_config.autopublish))
        .milestone(o.milestone().unwrap_or(default_config.milestone))
        .autosave(
          o.autoSave()
            .map(|auto_save| auto_save.0)
            .unwrap_or(default_config.autosave),
        );
      //todo storage
      if let Some(mut config) = o.clientConfig() {
        let client: WasmClient = WasmClient::from_config(&mut config)?;
        builder = builder.client(Arc::new(client.client.as_ref().clone()));
      };
    }

    Ok(Self(Rc::new(WasmRefCell::new(builder))))
  }

  /// Loads an existing identity with the specified `did` using the current builder configuration.
  /// The identity must exist in the configured `Storage`.
  #[wasm_bindgen(js_name = loadIdentity)]
  pub fn load_identity(&mut self, _did: WasmDID) -> Result<PromiseAccount> {
    todo!()
    // let builder = self.0.clone();
    // let promise: Promise = future_to_promise(async move {
    //
    //   builder
    //     .as_ref()
    //     .borrow_mut()
    //     .load_identity(did.0)
    //     .await
    //     .map(WasmAccount::from)
    //     .map(Into::into)
    //     .wasm_result()
    // });
    // Ok(promise.unchecked_into::<PromiseAccount>())
  }

  /// Creates a new identity based on the builder configuration and returns
  /// an {@link Account} object to manage it.
  ///
  /// The identity is stored locally in the `Storage`. The DID network is automatically determined
  /// by the {@link Client} used to publish it.
  ///
  /// @See {@link IdentitySetup} to customize the identity creation.
  #[wasm_bindgen(js_name = createIdentity)]
  pub fn create_identity(&mut self, identity_setup: Option<WasmIdentitySetup>) -> Result<PromiseAccount> {
    // Create IdentitySetup
    let mut setup = IdentitySetup::new();
    if let Some(identity_setup) = identity_setup {
      if let Some(key_type) = identity_setup.keyType() {
        setup = setup.key_type(key_type.into());
      }
      if let Some(method_secret) = identity_setup.methodSecret() {
        setup = setup.method_secret(method_secret.0);
      }
    }

    // Call the builder.
    let builder: Rc<WasmRefCell<AccountBuilder>> = self.0.clone();
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
  #[wasm_bindgen(typescript_type = "Promise<Account>")]
  pub type PromiseAccount;

  #[wasm_bindgen(typescript_type = "AccountBuilderOptions")]
  pub type AccountBuilderOptions;

  #[wasm_bindgen(structural, getter, method)]
  pub fn autopublish(this: &AccountBuilderOptions) -> Option<bool>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn clientConfig(this: &AccountBuilderOptions) -> Option<Config>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn milestone(this: &AccountBuilderOptions) -> Option<u32>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn autoSave(this: &AccountBuilderOptions) -> Option<WasmAutoSave>;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type AccountBuilderOptions = {

    /**
     * When the account will store its state to the storage.
     */
    autoSave?: AutoSave

    /**
     * `autopublish == true` the account will publish messages to the tangle on each update.
     * `autopublish == false` the account will combine and publish message when .publish() is called.
     */
    autopublish?: boolean,

    /**
     * Number of actions required to save a snapshot.
     */
    milestone?: number,

    /**
     * Client for tangle requests.
     */
    clientConfig?: Config
};
"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IdentitySetup")]
  pub type WasmIdentitySetup;

  #[wasm_bindgen(structural, getter, method)]
  pub fn keyType(this: &WasmIdentitySetup) -> Option<KeyType>;

  #[wasm_bindgen(structural, getter, method)]
  pub fn methodSecret(this: &WasmIdentitySetup) -> Option<WasmMethodSecret>;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT_2: &'static str = r#"
/**
 * Overrides the default creation of private and public keys.
 */
export type IdentitySetup = {
    keyType?: KeyType,
    methodSecret?: MethodSecret
};
"#;
