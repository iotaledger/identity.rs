// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use self::wasm_core_did::WasmCoreDID;
pub use self::wasm_core_document::WasmCoreDocument;
pub use self::wasm_did_url::WasmDIDUrl;
pub use self::wasm_diff_message::WasmDiffMessage;
pub use self::wasm_document::WasmDocument;
pub use self::wasm_document_metadata::WasmDocumentMetadata;
pub use self::wasm_iota_did::UWasmIotaDID;
pub use self::wasm_iota_did::WasmIotaDID;
pub use self::wasm_method_data::WasmMethodData;
pub use self::wasm_method_relationship::WasmMethodRelationship;
pub use self::wasm_method_scope::OptionMethodScope;
pub use self::wasm_method_scope::RefMethodScope;
pub use self::wasm_method_scope::WasmMethodScope;
pub use self::wasm_method_type::WasmMethodType;
pub use self::wasm_resolved_document::ArrayDocumentOrResolvedDocument;
pub use self::wasm_resolved_document::ArrayResolvedDocument;
pub use self::wasm_resolved_document::DocumentOrResolvedDocument;
pub use self::wasm_resolved_document::PromiseArrayResolvedDocument;
pub use self::wasm_resolved_document::PromiseResolvedDocument;
pub use self::wasm_resolved_document::WasmResolvedDocument;
pub use self::wasm_service::IService;
pub use self::wasm_service::UServiceEndpoint;
pub use self::wasm_service::WasmService;
pub use self::wasm_verification_method::WasmVerificationMethod;
pub use self::wasm_verifier_options::WasmVerifierOptions;

mod wasm_core_did;
mod wasm_core_document;
mod wasm_did_url;
mod wasm_diff_message;
mod wasm_document;
mod wasm_document_metadata;
mod wasm_iota_did;
mod wasm_method_data;
mod wasm_method_relationship;
mod wasm_method_scope;
mod wasm_method_type;
mod wasm_resolved_document;
mod wasm_service;
mod wasm_verification_method;
mod wasm_verifier_options;
