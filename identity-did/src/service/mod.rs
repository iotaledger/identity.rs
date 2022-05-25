// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::module_inception)]

mod builder;
mod service;
mod service_endpoint;

pub use self::builder::ServiceBuilder;
pub use self::service::deserialize_id_with_fragment;
pub use self::service::Service;
pub use self::service_endpoint::ServiceEndpoint;
