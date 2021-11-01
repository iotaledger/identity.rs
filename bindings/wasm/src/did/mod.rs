// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod wasm_did;
mod wasm_did_url;
mod wasm_document;
mod wasm_document_diff;
mod wasm_verification_method;

pub use self::wasm_did::WasmDID;
pub use self::wasm_did_url::WasmDIDUrl;
pub use self::wasm_document::WasmDocument;
pub use self::wasm_document_diff::WasmDocumentDiff;
pub use self::wasm_verification_method::WasmVerificationMethod;
