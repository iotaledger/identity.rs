// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub struct WasmToken {
    protectedHeader: Option<WasmJwsHeader>,
    unprotectedHeader: Option<WasmJwsHeader>, 
    claims: String
}
