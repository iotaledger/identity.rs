// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Contains an implementation of RevocationBitmap2022 for managing credential revocation.

mod bitmap;
mod document_ext;
mod error;

pub use self::bitmap::RevocationBitmap;
pub use self::document_ext::RevocationDocumentExt;
pub use self::error::RevocationError;
pub use self::error::RevocationResult;
