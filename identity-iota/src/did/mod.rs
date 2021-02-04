// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::module_inception)]

mod did;
mod did_segments;
mod document;
mod document_builder;
mod document_diff;
mod document_properties;

pub use self::did::IotaDID;
pub use self::did_segments::Segments;
pub use self::document::IotaDocument;
pub use self::document_builder::IotaDocumentBuilder;
pub use self::document_diff::DocumentDiff;
pub use self::document_properties::Properties;
