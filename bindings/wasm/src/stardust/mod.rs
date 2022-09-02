// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use stardust_did::WasmStardustDID;
pub use stardust_did_url::WasmStardustDIDUrl;
pub use stardust_document::WasmIotaDocument;
pub use stardust_document_metadata::WasmIotaDocumentMetadata;
pub use stardust_service::WasmIotaService;
pub use stardust_verification_method::WasmIotaVerificationMethod;
pub use state_metadata_encoding::WasmStateMetadataEncoding;

mod identity_client;
mod identity_client_ext;
mod stardust_did;
mod stardust_did_url;
mod stardust_document;
mod stardust_document_metadata;
mod stardust_service;
mod stardust_verification_method;
mod state_metadata_encoding;
