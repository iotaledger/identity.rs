// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]
//pub mod asset_move_caller;
//pub mod iota_client_provider;

#[cfg(target_arch = "wasm32")]
mod typescript;
#[cfg(not(target_arch = "wasm32"))]
mod rust;

// ************************************************************************
// ************************** WASM32 **************************************
// ************************************************************************

#[cfg(target_arch = "wasm32")]
pub use typescript::iota_client::IotaClientTsSdk as IotaClientAdapter;

#[cfg(target_arch = "wasm32")]
pub use typescript::asset_move_calls::AssetMoveCallsTsSdk as AssetMoveCallsAdapter;

#[cfg(target_arch = "wasm32")]
pub use typescript::identity_move_calls::IdentityMoveCallsTsSdk as IdentityMoveCallsAdapter;

#[cfg(target_arch = "wasm32")]
pub use typescript::transaction_builder::TransactionBuilderTsSdk as TransactionBuilderAdapter;

// ************************************************************************
// ************************** Non WASM32 targets **************************
// ************************************************************************

#[cfg(not(target_arch = "wasm32"))]
pub use rust::iota_client::IotaClientRustSdk as IotaClientAdapter;

#[cfg(not(target_arch = "wasm32"))]
pub use rust::asset_move_calls::AssetMoveCallsRustSdk as AssetMoveCallsAdapter;

#[cfg(not(target_arch = "wasm32"))]
pub use rust::identity_move_calls::IdentityMoveCallsRustSdk as IdentityMoveCallsAdapter;

#[cfg(not(target_arch = "wasm32"))]
pub use rust::transaction_builder::TransactionBuilderRustSdk as TransactionBuilderAdapter;