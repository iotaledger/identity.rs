// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implements the DID Document Service specification.
#![allow(clippy::module_inception)]

mod builder;
mod service;
mod service_endpoint;

pub use self::builder::ServiceBuilder;
pub use self::service::Service;
pub use self::service_endpoint::ServiceEndpoint;
