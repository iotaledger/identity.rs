// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod document_diff;
mod iota_document;
mod iota_verification_method;
mod properties;

pub use self::document_diff::DocumentDiff;
pub use self::iota_document::IotaDocument;
pub use self::iota_document::IotaDocumentSigner;
pub use self::iota_document::IotaDocumentVerifier;
pub use self::iota_verification_method::IotaVerificationMethod;
pub use self::properties::Properties;
