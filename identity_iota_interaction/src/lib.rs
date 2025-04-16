// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(missing_docs)]

mod iota_client_trait;
mod iota_verifiable_credential;
#[cfg(feature = "keytool-signer")]
pub mod keytool_signer;
mod move_call_traits;
mod move_type;
mod transaction_builder_trait;
mod effects_mut_api;

pub use iota_client_trait::*;
pub use iota_verifiable_credential::*;
#[cfg(feature = "keytool-signer")]
pub use keytool_signer::*;
pub use move_call_traits::*;
pub use move_type::*;
pub use transaction_builder_trait::*;
pub use effects_mut_api::*;

#[cfg(target_arch = "wasm32")]
mod sdk_types;
#[cfg(target_arch = "wasm32")]
pub use sdk_types::*;

#[cfg(not(target_arch = "wasm32"))]
pub use iota_sdk::*;
#[cfg(not(target_arch = "wasm32"))]
pub use move_core_types as move_types;
#[cfg(not(target_arch = "wasm32"))]
pub use shared_crypto;

/// BCS serialized Transaction, where a Transaction includes the TransactionData and a Vec<Signature>
pub type TransactionBcs = Vec<u8>;
/// BCS serialized TransactionData
/// TransactionData usually contain the ProgrammableTransaction, sender, kind = ProgrammableTransaction,
/// gas_coin, gas_budget, gas_price, expiration, ...
/// Example usage:
/// * TS: ExecuteTransactionBlockParams::transactionBlock - Base64 encoded TransactionDataBcs
pub type TransactionDataBcs = Vec<u8>;
/// BCS serialized Signature
pub type SignatureBcs = Vec<u8>;
/// BCS serialized ProgrammableTransaction
/// A ProgrammableTransaction
/// * has `inputs` (or *CallArgs*) and `commands` (or *Transactions*)
/// * is the result of ProgrammableTransactionBuilder::finish()
pub type ProgrammableTransactionBcs = Vec<u8>;
/// BCS serialized IotaTransactionBlockResponse
pub type IotaTransactionBlockResponseBcs = Vec<u8>;

// dummy types, have to be replaced with actual types later on
pub type DummySigner = str;
pub type Hashable<T> = Vec<T>;
pub type Identity = ();

/// `ident_str!` is a compile-time validated macro that constructs a
/// `&'static IdentStr` from a const `&'static str`.
///
/// ### Example
///
/// Creating a valid static or const [`IdentStr`]:
///
/// ```rust
/// use move_core_types::ident_str;
/// use move_core_types::identifier::IdentStr;
/// const VALID_IDENT: &'static IdentStr = ident_str!("MyCoolIdentifier");
///
/// const THING_NAME: &'static str = "thing_name";
/// const THING_IDENT: &'static IdentStr = ident_str!(THING_NAME);
/// ```
///
/// In contrast, creating an invalid [`IdentStr`] will fail at compile time:
///
/// ```rust,compile_fail
/// use move_core_types::{ident_str, identifier::IdentStr};
/// const INVALID_IDENT: &'static IdentStr = ident_str!("123Foo"); // Fails to compile!
/// ```
// TODO(philiphayes): this should really be an associated const fn like `IdentStr::new`;
// unfortunately, both unsafe-reborrow and unsafe-transmute don't currently work
// inside const fn's. Only unsafe-transmute works inside static const-blocks
// (but not const-fn's).
#[macro_export]
macro_rules! ident_str {
  ($ident:expr) => {{
    // Only static strings allowed.
    let s: &'static str = $ident;

    // Only valid identifier strings are allowed.
    // Note: Work-around hack to print an error message in a const block.
    let is_valid = $crate::move_types::identifier::is_valid(s);
    ["String is not a valid Move identifier"][!is_valid as usize];

    // SAFETY: the following transmute is safe because
    // (1) it's equivalent to the unsafe-reborrow inside IdentStr::ref_cast()
    //     (which we can't use b/c it's not const).
    // (2) we've just asserted that IdentStr impls RefCast<From = str>, which
    //     already guarantees the transmute is safe (RefCast checks that
    //     IdentStr(str) is #[repr(transparent)]).
    // (3) both in and out lifetimes are 'static, so we're not widening the
    // lifetime. (4) we've just asserted that the IdentStr passes the
    // is_valid check.
    //
    // Note: this lint is unjustified and no longer checked. See issue:
    // https://github.com/rust-lang/rust-clippy/issues/6372
    #[allow(clippy::transmute_ptr_to_ptr)]
    unsafe {
      ::std::mem::transmute::<&'static str, &'static $crate::move_types::identifier::IdentStr>(s)
    }
  }};
}

// Alias name for the Send trait controlled by the "send-sync-transaction" feature
cfg_if::cfg_if! {
  if #[cfg(feature = "send-sync-transaction")] {
    pub trait OptionalSend: Send {}
    impl<T> OptionalSend for T where T: Send {}

    pub trait OptionalSync: Sync {}
    impl<T> OptionalSync for T where T: Sync {}
  } else {
    pub trait OptionalSend: {}
    impl<T> OptionalSend for T {}

    pub trait OptionalSync: {}
    impl<T> OptionalSync for T where T: {}
  }
}
