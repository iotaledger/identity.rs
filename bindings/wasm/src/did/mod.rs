// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use self::service::IService;
pub use self::service::UServiceEndpoint;
pub use self::service::WasmService;
pub use self::wasm_core_did::IToCoreDID;
pub use self::wasm_core_did::WasmCoreDID;
pub use self::wasm_core_document::ArrayIToCoreDocument;
pub(crate) use self::wasm_core_document::CoreDocumentLock;
pub use self::wasm_core_document::IToCoreDocument;
pub use self::wasm_core_document::WasmCoreDocument;
pub use self::wasm_did_url::WasmDIDUrl;

pub use self::wasm_verifier_options::WasmVerifierOptions;

mod service;
mod wasm_core_did;
mod wasm_core_document;
mod wasm_did_url;
mod wasm_verifier_options;
