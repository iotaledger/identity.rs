#[cfg(target_arch = "wasm32")]
pub mod asset_move_calls;
#[cfg(target_arch = "wasm32")]
pub mod identity_move_calls;
#[cfg(target_arch = "wasm32")]
mod migration_move_calls;

