// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The core types used to create Verifiable Presentations.

#![allow(clippy::module_inception)]

mod jwt_presentation_options;
mod jwt_serialization;
mod presentation;
mod presentation_builder;

pub use self::jwt_presentation_options::JwtPresentationOptions;
pub use self::presentation::Presentation;
pub use self::presentation_builder::PresentationBuilder;

#[cfg(feature = "validator")]
pub(crate) use self::jwt_serialization::PresentationJwtClaims;
