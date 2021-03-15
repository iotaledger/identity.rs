// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod auth_record;
mod config;
mod credential;
mod diff_record;
mod identity;
mod key;
mod key_location;
mod metadata;
mod metadata_list;
mod presentation;
mod resource;
mod signature;

pub use self::auth_record::AuthRecord;
pub use self::config::IdentityConfig;
pub use self::credential::Credential;
pub use self::diff_record::DiffRecord;
pub use self::identity::Identity;
pub use self::key::Key;
pub use self::key_location::KeyLocation;
pub use self::metadata::CredentialMetadata;
pub use self::metadata::IdentityMetadata;
pub use self::metadata::PresentationMetadata;
pub use self::metadata_list::Metadata;
pub use self::metadata_list::MetadataList;
pub use self::presentation::Presentation;
pub use self::resource::ResourceId;
pub use self::resource::ResourceType;
pub use self::signature::Signature;
