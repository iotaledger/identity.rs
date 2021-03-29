// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod handle;
mod stronghold;
mod traits;

pub use self::handle::StorageHandle;
pub use self::stronghold::StrongholdAdapter;
pub use self::traits::StorageAdapter;
pub use self::traits::VaultAdapter;
