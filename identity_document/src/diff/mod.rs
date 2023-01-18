// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use self::diff_document::DiffDocument;
pub use self::diff_service::DiffService;
pub use identity_verification::diff::DiffMethod;
pub use identity_verification::diff::DiffMethodData;
pub use identity_verification::diff::DiffMethodRef;

mod diff_document;
mod diff_service;
