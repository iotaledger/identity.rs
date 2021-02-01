// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The core types used to create Verifiable Presentations

#![allow(clippy::module_inception)]

mod builder;
mod presentation;
mod verifiable;

pub use self::builder::Builder;
pub use self::presentation::Presentation;
pub use self::verifiable::VerifiablePresentation;
