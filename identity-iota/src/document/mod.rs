// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use self::iota_document::IotaCoreDocument;
pub use self::iota_document::IotaDocument;
pub use self::iota_document::IotaService;
pub use self::iota_document::IotaVerificationMethod;
pub use self::iota_document_metadata::IotaDocumentMetadata;
pub use self::resolved_iota_document::ResolvedIotaDocument;

mod iota_document;
mod iota_document_metadata;
mod resolved_iota_document;
