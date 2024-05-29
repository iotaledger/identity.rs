// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use iota_document::IotaDocument;
pub use iota_document_metadata::IotaDocumentMetadata;
pub use web_document::WebDocument;

mod iota_document;
mod iota_document_metadata;
mod web_document;

#[cfg(test)]
pub(crate) mod test_utils;
