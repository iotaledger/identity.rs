// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Contains the implementations for all the credential revocation methods that can be used with IOTA's Identity
//! framework.

mod bitmap;
mod document_ext;
mod error;
pub mod status_list_2021;

pub use self::bitmap::RevocationBitmap;
pub use self::document_ext::RevocationDocumentExt;
pub use self::error::RevocationError;
pub use self::error::RevocationResult;
