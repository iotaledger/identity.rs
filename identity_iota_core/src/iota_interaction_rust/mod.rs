// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// IOTA Rust SDK based implementation of the identity_iota_interaction::AssetMoveCalls trait
pub mod asset_move_calls;
/// IOTA Rust SDK based implementation of the identity_iota_interaction::IdentityMoveCalls trait
pub mod identity_move_calls;
/// IOTA Rust SDK based implementation of the identity_iota_interaction::IotaClientTrait trait
pub mod iota_client_rust_sdk;
/// IOTA Rust SDK based implementation of the identity_iota_interaction::MigrationMoveCalls trait
pub mod migration_move_calls;
/// IOTA Rust SDK based implementation of the identity_iota_interaction::TransactionBuilderT trait
pub mod transaction_builder;

mod utils;

pub use super::rebased::Error as AdapterError;
pub use asset_move_calls::AssetMoveCallsRustSdk as AssetMoveCallsAdapter;
pub use identity_iota_interaction::rpc_types::IotaTransactionBlockResponse as NativeTransactionBlockResponse;
pub use identity_move_calls::IdentityMoveCallsRustSdk as IdentityMoveCallsAdapter;
pub use iota_client_rust_sdk::IotaClientRustSdk as IotaClientAdapter;
pub use iota_client_rust_sdk::IotaTransactionBlockResponseProvider as IotaTransactionBlockResponseAdapter;
pub use migration_move_calls::MigrationMoveCallsRustSdk as MigrationMoveCallsAdapter;
#[allow(unused_imports)]
pub use transaction_builder::TransactionBuilderRustSdk as TransactionBuilderAdapter;

#[allow(unused_imports)]
pub use iota_client_rust_sdk::CoinReadApiAdaptedT;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::CoinReadApiAdaptedTraitObj;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::EventApiAdaptedT;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::EventApiAdaptedTraitObj;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::IotaClientAdaptedT;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::IotaClientAdaptedTraitObj;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::IotaTransactionBlockResponseAdaptedT;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::IotaTransactionBlockResponseAdaptedTraitObj;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::QuorumDriverApiAdaptedT;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::QuorumDriverApiAdaptedTraitObj;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::ReadApiAdaptedT;
#[allow(unused_imports)]
pub use iota_client_rust_sdk::ReadApiAdaptedTraitObj;
