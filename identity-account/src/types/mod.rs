// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod config;
mod identifier;
mod key;
mod location;
mod metadata;
mod signature;
mod timestamps;

pub use self::config::IdentityConfig;
pub use self::config::MethodConfig;
pub use self::identifier::Identifier;
pub use self::key::Key;
pub use self::metadata::CredentialMetadata;
pub use self::metadata::IdentityMetadata;
pub use self::metadata::MetadataItem;
pub use self::metadata::MetadataList;
pub use self::signature::Signature;
pub use self::timestamps::Timestamps;
pub use self::location::Index;
pub use self::location::AuthLocation;
pub use self::location::DiffLocation;
pub use self::location::DocLocation;
pub use self::location::KeyLocation;
pub use self::location::MetaLocation;
pub use self::location::ToKey;
