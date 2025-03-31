// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

#[cfg(not(target_arch = "wasm32"))]
use identity_iota_interaction::rpc_types::IotaTransactionBlockResponse;

use crate::iota_interaction_adapter::IotaTransactionBlockResponseAdaptedTraitObj;
use super::transaction_builder::TransactionBuilder;

/// The output type of a [`Transaction`].
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
pub struct TransactionOutput<T> {
  /// The parsed Transaction output. See [`Transaction::Output`].
  pub output: T,
  /// The "raw" transaction execution response received.
  pub response: IotaTransactionBlockResponse,
}

/// The output type of a [`Transaction`].
pub struct TransactionOutputInternal<T> {
  /// The parsed Transaction output. See [`Transaction::Output`].
  pub output: T,
  /// The "raw" transaction execution response received.
  pub response: IotaTransactionBlockResponseAdaptedTraitObj,
}

impl<T> Deref for TransactionOutputInternal<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &self.output
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl<T> From<TransactionOutputInternal<T>> for TransactionOutput<T> {
  fn from(value: TransactionOutputInternal<T>) -> Self {
    let TransactionOutputInternal::<T> {
      output: out,
      response: internal_response,
    } = value;
    let response = internal_response.clone_native_response();
    TransactionOutput { output: out, response }
  }
}

/// Interface to describe an operation that can eventually
/// be turned into a [`Transaction`], given the right input.
pub trait ProtoTransaction {
  /// The input required by this operation.
  type Input;
  /// This operation's next state. Can either be another [`ProtoTransaction`]
  /// or a whole [`Transaction`] ready to be executed.
  type Tx: ProtoTransaction;

  /// Feed this operation with its required input, advancing its
  /// state to another [`ProtoTransaction`] that may or may not
  /// be ready for execution.
  fn with(self, input: Self::Input) -> Self::Tx;
}

// Every Transaction is a QuasiTransaction that requires no input
// and that has itself as its next state.
impl<T> ProtoTransaction for TransactionBuilder<T>
where
  T: super::transaction_builder::Transaction,
{
  type Input = ();
  type Tx = Self;

  fn with(self, _: Self::Input) -> Self::Tx {
    self
  }
}
