// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod handle;
mod location;
mod resource;
mod stronghold;
mod traits;

pub use self::handle::StorageHandle;
pub use self::location::KeyLocation;
pub use self::resource::ResourceId;
pub use self::resource::ResourceType;
pub use self::stronghold::StrongholdAdapter;
pub use self::traits::StorageAdapter;
pub use self::traits::VaultAdapter;
