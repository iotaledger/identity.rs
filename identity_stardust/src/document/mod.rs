// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use stardust_document::StardustCoreDocument;
pub(crate) use stardust_document::StardustDID as TmpStardustDID;
pub use stardust_document::StardustDIDUrl;
pub use stardust_document::StardustDocument;
pub use stardust_document::StardustService;
pub use stardust_document::StardustVerificationMethod;
pub use stardust_document_metadata::StardustDocumentMetadata;

mod stardust_document;
mod stardust_document_metadata;
