// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod identity;
mod proposals;
mod wasm_identity_client;
mod wasm_identity_client_read_only;
mod transaction_builder;

pub use identity::*;
pub use wasm_identity_client::*;
pub use wasm_identity_client_read_only::*;
pub use transaction_builder::*;

pub type WasmIotaAddress = String;
pub type WasmObjectID = String;
