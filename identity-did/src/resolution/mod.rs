// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types and traits for DID Document resolution.

#![allow(clippy::module_inception)]

mod dereference;
mod document_metadata;
mod error_kind;
mod impls;
mod input_metadata;
mod resolution;
mod resolution_metadata;
mod resource;
mod traits;

pub use self::dereference::Dereference;
pub use self::document_metadata::DocumentMetadata;
pub use self::error_kind::ErrorKind;
pub use self::impls::dereference;
pub use self::impls::resolve;
pub use self::input_metadata::InputMetadata;
pub use self::input_metadata::MIME_DID;
pub use self::input_metadata::MIME_DID_LD;
pub use self::resolution::Resolution;
pub use self::resolution_metadata::ResolutionMetadata;
pub use self::resource::PrimaryResource;
pub use self::resource::Resource;
pub use self::resource::SecondaryResource;
pub use self::traits::MetaDocument;
pub use self::traits::ResolverMethod;
