// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use self::ed25519::WasmEd25519;
pub use self::secret_key::WasmEd25519PrivateKey;

pub mod ed25519;
pub mod secret_key;
