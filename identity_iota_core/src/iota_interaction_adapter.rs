use cfg_if::cfg_if;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub use iota_interaction_ts::IotaClientAdapter;
        pub use iota_interaction_ts::AssetMoveCallsAdapter;
        pub use iota_interaction_ts::IdentityMoveCallsAdapter;
        pub use iota_interaction_ts::TransactionBuilderAdapter;
    } else {
        pub use crate::iota_interaction_rust::IotaClientAdapter;
        pub use crate::iota_interaction_rust::AssetMoveCallsAdapter;
        pub use crate::iota_interaction_rust::IdentityMoveCallsAdapter;
        pub use crate::iota_interaction_rust::TransactionBuilderAdapter;
    }
}