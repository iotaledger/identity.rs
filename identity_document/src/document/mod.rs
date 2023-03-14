// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Defines the core (implementation agnostic) DID Document type.

#![allow(clippy::module_inception)]

pub use self::builder::DocumentBuilder;
pub use self::core_document::CoreDocument;

#[cfg(feature = "diff")]
pub(crate) use core_document::CoreDocumentData;
mod builder;
mod core_document;
