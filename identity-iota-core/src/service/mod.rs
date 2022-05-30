// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod embedded_revocation_endpoint;
mod embedded_revocation_service;
mod error;

pub use self::embedded_revocation_endpoint::EmbeddedRevocationEndpoint;
pub use self::embedded_revocation_service::EmbeddedRevocationService;
pub use self::error::ServiceError;
