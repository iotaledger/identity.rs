// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::module_inception)]

mod bitmap_revocation_endpoint;
mod bitmap_revocation_service;
mod builder;
mod service;
mod service_endpoint;

pub use self::bitmap_revocation_endpoint::BitmapRevocationEndpoint;
pub use self::bitmap_revocation_service::BitmapRevocationService;
pub use self::bitmap_revocation_service::BITMAP_REVOCATION_METHOD;
pub use self::builder::ServiceBuilder;
pub use self::service::Service;
pub use self::service_endpoint::ServiceEndpoint;
