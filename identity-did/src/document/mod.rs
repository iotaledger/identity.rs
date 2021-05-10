// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Defines the core (implementation agnostic) DID Document type.

#![allow(clippy::module_inception)]

mod builder;
mod core_document;

pub use self::builder::DocumentBuilder;
pub use self::core_document::CoreDocument;
