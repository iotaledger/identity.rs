// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import "./append_functions.js";

export * from "./iota_identity_client.js";
export * from "./jose";
export * from "./jwk_storage";
export * from "./key_id_storage";

export * from "~identity_wasm";

// keep this export last to override the original `Resolver` from `identity_wasm` in the exports
export { Resolver } from "./resolver";
