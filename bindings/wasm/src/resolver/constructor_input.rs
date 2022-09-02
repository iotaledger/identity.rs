// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::stardust::PromiseStardustDocument;
use crate::stardust::WasmStardustDID;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "ResolutionHandlers")]
  pub type MapResolutionHandler;

  #[wasm_bindgen(typescript_type = "IStardustIdentityClient | undefined")]
  pub(crate) type OptionWasmStardustIdentityClient;

  #[wasm_bindgen(method, js_name = resolveDid)]
  pub(crate) fn resolve_did(this: &OptionWasmStardustIdentityClient, did: WasmStardustDID) -> PromiseStardustDocument;

  #[wasm_bindgen(typescript_type = "ResolverConfig")]
  pub type ResolverConfig;

  #[wasm_bindgen(method, getter)]
  pub(crate) fn client(this: &ResolverConfig) -> OptionWasmStardustIdentityClient;

  #[wasm_bindgen(method, getter)]
  pub(crate) fn handlers(this: &ResolverConfig) -> Option<MapResolutionHandler>;

}

// Workaround because JSDocs does not support arrows (=>) while TS does not support the "function" word in type
// definitions (which would be accepted by JSDocs).
#[wasm_bindgen(typescript_custom_section)]
const HANDLERS: &'static str =
  "export type ResolutionHandlers = Map<string, (did: string) => Promise<StardustDocument | CoreDocument>>;";

#[wasm_bindgen(typescript_custom_section)]
const TS_RESOLVER_CONFIG: &'static str = r#"
/**
 * Configurations for the new {@link MixedResolver}.
 */
export type ResolverConfig = {

    /**
     * Client for resolving DIDs of the iota method. 
     */
    client?: IStardustIdentityClient,

    /**
     * Handlers for resolving DIDs from arbitrary DID methods. 
     * 
     * The keys to the map are expected to match the method name and the values are asynchronous functions returning DID documents. 
     * 
     * Note that if a `client` is given the key "iota" may NOT be present in this map. 
     */
    handlers?: Map<string, (did: string) => Promise<StardustDocument | CoreDocument>>
};
"#;
