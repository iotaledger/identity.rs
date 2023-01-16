// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use self::diff_document::DiffDocument;
pub use identity_verification::verification_method::diff::DiffMethod;
pub use self::diff_service::DiffService;
pub use identity_verification::verification_method::diff::DiffMethodData;
pub use identity_verification::verification_method::diff::DiffMethodRef;

mod diff_document;
mod diff_service;
