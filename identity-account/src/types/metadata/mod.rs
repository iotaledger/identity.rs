// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod credential;
mod identity;
mod list;
mod traits;

pub use self::credential::CredentialMetadata;
pub use self::identity::IdentityMetadata;
pub use self::list::MetadataList;
pub use self::traits::MetadataItem;
