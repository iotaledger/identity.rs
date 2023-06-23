// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The core types used to create Verifiable Presentations

#![allow(clippy::module_inception)]

mod jwt_presentation;
mod jwt_presentation_builder;
mod jwt_presentation_options;
mod jwt_serialization;

pub use self::jwt_presentation::JwtPresentation;
pub use self::jwt_presentation_builder::JwtPresentationBuilder;
pub use self::jwt_presentation_options::JwtPresentationOptions;

#[cfg(feature = "validator")]
pub(crate) use self::jwt_serialization::PresentationJwtClaims;
