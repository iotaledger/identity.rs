// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) use identity_client::WasmIotaIdentityClient;
pub use identity_client_ext::PromiseIotaDocument;
pub use iota_did::WasmIotaDID;
pub use iota_did_url::WasmIotaDIDUrl;
pub use iota_document::WasmIotaDocument;
pub use iota_document_metadata::WasmIotaDocumentMetadata;
pub use iota_metadata_encoding::WasmStateMetadataEncoding;
pub use iota_service::WasmIotaService;
pub use iota_verification_method::WasmIotaVerificationMethod;

mod identity_client;
mod identity_client_ext;
mod iota_did;
mod iota_did_url;
mod iota_document;
mod iota_document_metadata;
mod iota_metadata_encoding;
mod iota_service;
mod iota_verification_method;
