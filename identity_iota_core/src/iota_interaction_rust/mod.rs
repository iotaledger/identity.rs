// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod asset_move_calls;
pub(crate) mod identity_move_calls;
pub(crate) mod iota_client_rust_sdk;
pub(crate) mod migration_move_calls;
pub(crate) mod transaction_builder;
mod utils;

pub(crate) use asset_move_calls::AssetMoveCallsRustSdk as AssetMoveCallsAdapter;
pub(crate) use identity_move_calls::IdentityMoveCallsRustSdk as IdentityMoveCallsAdapter;
pub(crate) use iota_client_rust_sdk::IotaClientRustSdk as IotaClientAdapter;
pub(crate) use iota_client_rust_sdk::IotaTransactionBlockResponseProvider as IotaTransactionBlockResponseAdapter;
pub(crate) use migration_move_calls::MigrationMoveCallsRustSdk as MigrationMoveCallsAdapter;
pub(crate) use transaction_builder::TransactionBuilderRustSdk as TransactionBuilderAdapter;

#[allow(unused_imports)]
pub(crate) use iota_client_rust_sdk::CoinReadApiAdaptedT;
#[allow(unused_imports)]
pub(crate) use iota_client_rust_sdk::CoinReadApiAdaptedTraitObj;
#[allow(unused_imports)]
pub(crate) use iota_client_rust_sdk::EventApiAdaptedT;
#[allow(unused_imports)]
pub(crate) use iota_client_rust_sdk::EventApiAdaptedTraitObj;
#[allow(unused_imports)]
pub(crate) use iota_client_rust_sdk::IotaClientAdaptedT;
#[allow(unused_imports)]
pub(crate) use iota_client_rust_sdk::IotaClientAdaptedTraitObj;
#[allow(unused_imports)]
pub(crate) use iota_client_rust_sdk::IotaTransactionBlockResponseAdaptedT;
#[allow(unused_imports)]
pub(crate) use iota_client_rust_sdk::IotaTransactionBlockResponseAdaptedTraitObj;
#[allow(unused_imports)]
pub(crate) use iota_client_rust_sdk::QuorumDriverApiAdaptedT;
#[allow(unused_imports)]
pub(crate) use iota_client_rust_sdk::QuorumDriverApiAdaptedTraitObj;
#[allow(unused_imports)]
pub(crate) use iota_client_rust_sdk::ReadApiAdaptedT;
#[allow(unused_imports)]
pub(crate) use iota_client_rust_sdk::ReadApiAdaptedTraitObj;

#[allow(unused_imports)]
pub(crate) use super::rebased::Error as AdapterError;
#[allow(unused_imports)]
pub(crate) use identity_iota_interaction::rpc_types::IotaTransactionBlockResponse as AdapterNativeResponse;
