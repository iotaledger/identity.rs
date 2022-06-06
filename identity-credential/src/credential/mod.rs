// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The core types used to create Verifiable Credentials

#![allow(clippy::module_inception)]

mod bitmap_revocation_status;
mod builder;
mod credential;
mod evidence;
mod issuer;
mod policy;
mod refresh;
mod schema;
mod status;
mod subject;

pub use self::bitmap_revocation_status::BitmapRevocationStatus;
pub use self::builder::CredentialBuilder;
pub use self::credential::Credential;
pub use self::evidence::Evidence;
pub use self::issuer::Issuer;
pub use self::policy::Policy;
pub use self::refresh::RefreshService;
pub use self::schema::Schema;
pub use self::status::Status;
pub use self::subject::Subject;
