// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod builder;
mod embedded_revocation_service;
mod error;

pub use self::builder::EmbeddedServiceBuilder;
pub use self::embedded_revocation_service::EmbeddedRevocationService;
pub use self::ServiceError;
