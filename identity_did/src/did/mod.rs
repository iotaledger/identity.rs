// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Provides generic types and traits for working with Decentralized Identifiers.

pub use ::did_url::DID as BaseDIDUrl;

pub use self::did::CoreDID;
pub use self::did::DID;
pub use self::did_url::CoreDIDUrl;
pub use self::did_url::DIDUrl;
pub use self::did_url::RelativeDIDUrl;
pub use self::error::DIDError;
pub use self::traits::IntoDIDUrl;
pub use self::traits::ToDIDUrl;

#[allow(clippy::module_inception)]
mod did;
mod did_url;
mod error;
mod traits;
