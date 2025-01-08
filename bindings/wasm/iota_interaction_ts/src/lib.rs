// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0pub mod iota_client_ts_sdk;

#[cfg(target_arch = "wasm32")]
pub(crate) mod bindings;

#[cfg(target_arch = "wasm32")]
pub mod asset_move_calls;
#[cfg(target_arch = "wasm32")]
pub mod common;
#[cfg(target_arch = "wasm32")]
pub mod error;
#[cfg(target_arch = "wasm32")]
pub mod identity_move_calls;
#[cfg(target_arch = "wasm32")]
pub mod iota_client_ts_sdk;
#[cfg(target_arch = "wasm32")]
mod migration_move_calls;
#[cfg(target_arch = "wasm32")]
pub mod transaction_builder;

#[cfg(target_arch = "wasm32")]
pub use asset_move_calls::AssetMoveCallsTsSdk as AssetMoveCallsAdapter;
#[cfg(target_arch = "wasm32")]
pub use identity_move_calls::IdentityMoveCallsTsSdk as IdentityMoveCallsAdapter;
#[cfg(target_arch = "wasm32")]
pub use iota_client_ts_sdk::IotaClientTsSdk as IotaClientAdapter;
#[cfg(target_arch = "wasm32")]
pub use iota_client_ts_sdk::IotaTransactionBlockResponseProvider as IotaTransactionBlockResponseAdapter;
#[cfg(target_arch = "wasm32")]
pub use migration_move_calls::MigrationMoveCallsTsSdk as MigrationMoveCallsAdapter;
#[cfg(target_arch = "wasm32")]
pub use transaction_builder::TransactionBuilderTsSdk as TransactionBuilderAdapter;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        #[allow(unused_imports)] pub use iota_client_ts_sdk::IotaTransactionBlockResponseAdaptedT;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::IotaTransactionBlockResponseAdaptedTraitObj;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::QuorumDriverApiAdaptedT;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::QuorumDriverApiAdaptedTraitObj;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::ReadApiAdaptedT;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::ReadApiAdaptedTraitObj;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::CoinReadApiAdaptedT;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::CoinReadApiAdaptedTraitObj;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::EventApiAdaptedT;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::EventApiAdaptedTraitObj;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::IotaClientAdaptedT;
        #[allow(unused_imports)] pub use iota_client_ts_sdk::IotaClientAdaptedTraitObj;

        #[allow(unused_imports)] pub use error::TsSdkError as AdapterError;
        #[allow(unused_imports)] pub use bindings::IotaTransactionBlockResponseAdapter as AdapterNativeResponse;

        #[allow(unused_imports)] pub use transaction_builder::NativeTsCodeBindingWrapper;

        #[allow(unused_imports)] pub use bindings::ProgrammableTransaction;
    }
}
