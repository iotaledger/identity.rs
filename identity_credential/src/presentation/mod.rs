// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The core types used to create Verifiable Presentations

#![allow(clippy::module_inception)]

mod builder;
mod jwt_presentation;
mod jwt_presentation_builder;
mod jwt_presentation_options;
mod jwt_serialization;
mod presentation;

pub use self::builder::PresentationBuilder;
pub use self::jwt_presentation::JwtPresentation;
pub use self::jwt_presentation_builder::JwtPresentationBuilder;
pub use self::jwt_presentation_options::JwtPresentationOptions;
pub use self::presentation::Presentation;

#[cfg(feature = "validator")]
pub(crate) use self::jwt_serialization::*;
