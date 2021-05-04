// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod diff;
mod iota_document;
mod iota_verification_method;
mod properties;

pub use self::diff::DocumentDiff;
pub use self::iota_document::IotaDocument;
pub use self::iota_document::Signer;
pub use self::iota_document::Verifier;
pub use self::iota_verification_method::IotaVerificationMethod;
pub use self::properties::Properties;
