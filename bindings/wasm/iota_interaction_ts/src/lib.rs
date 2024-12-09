// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0pub mod iota_client_ts_sdk;


#[cfg(target_arch = "wasm32")]
pub(crate) mod bindings;

#[cfg(target_arch = "wasm32")]
pub mod iota_client_ts_sdk;
#[cfg(target_arch = "wasm32")]
pub mod asset_move_calls;
#[cfg(target_arch = "wasm32")]
pub mod identity_move_calls;
#[cfg(target_arch = "wasm32")]
pub mod transaction_builder;
#[cfg(target_arch = "wasm32")]
pub mod error;
#[cfg(target_arch = "wasm32")]
pub mod common;

#[cfg(target_arch = "wasm32")]
pub use iota_client_ts_sdk::IotaClientTsSdk as IotaClientAdapter;
#[cfg(target_arch = "wasm32")]
pub use asset_move_calls::AssetMoveCallsTsSdk as AssetMoveCallsAdapter;
#[cfg(target_arch = "wasm32")]
pub use identity_move_calls::IdentityMoveCallsTsSdk as IdentityMoveCallsAdapter;
#[cfg(target_arch = "wasm32")]
pub use transaction_builder::TransactionBuilderTsSdk as TransactionBuilderAdapter;
