// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod asset_move_calls;
pub mod identity_move_calls;
pub mod iota_client_rust_sdk;
pub mod transaction_builder;
pub mod utils;

pub use iota_client_rust_sdk::IotaClientRustSdk as IotaClientAdapter;
pub use asset_move_calls::AssetMoveCallsRustSdk as AssetMoveCallsAdapter;
pub use identity_move_calls::IdentityMoveCallsRustSdk as IdentityMoveCallsAdapter;
pub use transaction_builder::TransactionBuilderRustSdk as TransactionBuilderAdapter;