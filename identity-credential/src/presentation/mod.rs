// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The core types used to create Verifiable Presentations

#![allow(clippy::module_inception)]

mod builder;
mod presentation;

pub use self::builder::PresentationBuilder;
pub use self::presentation::Presentation;
