// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(target_arch = "wasm32")]
pub mod asset_move_calls;
#[cfg(target_arch = "wasm32")]
pub mod identity_move_calls;
#[cfg(target_arch = "wasm32")]
mod migration_move_calls;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        #[allow(unused_imports)] pub use asset_move_calls::AssetMoveCallsTsSdk as AssetMoveCallsAdapter;
        #[allow(unused_imports)] pub use identity_move_calls::IdentityMoveCallsTsSdk as IdentityMoveCallsAdapter;
        #[allow(unused_imports)] pub use migration_move_calls::MigrationMoveCallsTsSdk as MigrationMoveCallsAdapter;
    }
}
