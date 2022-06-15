// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Provides generic types and traits for working with Decentralized Identifiers.

#[allow(clippy::module_inception)]
mod did;
mod did_url;
mod error;

pub use self::did::CoreDID;
pub use self::did::DID;
pub use self::did_url::CoreDIDUrl;
pub use self::did_url::DIDUrl;
pub use self::did_url::RelativeDIDUrl;
pub use self::error::DIDError;
pub use ::did_url::DID as BaseDIDUrl;
