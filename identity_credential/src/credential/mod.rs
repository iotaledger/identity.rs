// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The core types used to create Verifiable Credentials

#![allow(clippy::module_inception)]

mod builder;
mod credential;
mod domain_linkage_configuration;
mod domain_linkage_credential_builder;
mod domain_linkage_utils;
mod evidence;
mod issuer;
mod policy;
mod refresh;
#[cfg(feature = "revocation-bitmap")]
mod revocation_bitmap_status;
mod schema;
mod status;
mod subject;

pub use self::builder::CredentialBuilder;
pub use self::credential::Credential;
pub use self::domain_linkage_configuration::DomainLinkageConfiguration;
pub use self::domain_linkage_credential_builder::DomainLinkageCredentialBuilder;
pub use self::domain_linkage_utils::DomainLinkageUtils;
pub use self::evidence::Evidence;
pub use self::issuer::Issuer;
pub use self::policy::Policy;
pub use self::refresh::RefreshService;
#[cfg(feature = "revocation-bitmap")]
pub use self::revocation_bitmap_status::RevocationBitmapStatus;
pub use self::schema::Schema;
pub use self::status::Status;
pub use self::subject::Subject;
