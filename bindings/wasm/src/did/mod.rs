// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use self::service::IService;
pub use self::service::UServiceEndpoint;
pub use self::service::WasmService;
pub use self::wasm_core_did::IAsCoreDID;
pub use self::wasm_core_did::WasmCoreDID;
pub use self::wasm_core_document::WasmCoreDocument;
pub(crate) use self::wasm_core_document::CoreDocumentLock;
pub use self::wasm_did_url::WasmDIDUrl;
pub use self::wasm_method_data::WasmMethodData;
pub use self::wasm_method_relationship::WasmMethodRelationship;
pub use self::wasm_method_scope::OptionMethodScope;
pub use self::wasm_method_scope::RefMethodScope;
pub use self::wasm_method_scope::WasmMethodScope;
pub use self::wasm_method_type::WasmMethodType;
pub use self::wasm_verification_method::WasmVerificationMethod;
pub use self::wasm_verifier_options::WasmVerifierOptions;

mod service;
mod wasm_core_did;
mod wasm_core_document;
mod wasm_did_url;
mod wasm_method_data;
mod wasm_method_relationship;
mod wasm_method_scope;
mod wasm_method_type;
mod wasm_verification_method;
mod wasm_verifier_options;
