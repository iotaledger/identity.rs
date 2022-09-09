// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
pub(crate) use identity_client::WasmStardustIdentityClient;
pub use identity_client_ext::PromiseStardustDocument;
pub use stardust_did::WasmStardustDID;
pub use stardust_did_url::WasmStardustDIDUrl;
pub use stardust_document::WasmStardustDocument;
pub use stardust_document_metadata::WasmStardustDocumentMetadata;
pub use stardust_service::WasmStardustService;
pub use stardust_verification_method::WasmStardustVerificationMethod;
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
