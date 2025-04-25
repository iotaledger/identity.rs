// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;
use std::ops::DerefMut;

use crate::bindings::WasmTransactionBuilder;
use crate::error::TsSdkError;
use crate::error::WasmError;
use iota_interaction::ProgrammableTransactionBcs;
use iota_interaction::TransactionBuilderT;

pub type NativeTsTransactionBuilderBindingWrapper = WasmTransactionBuilder;

pub struct TransactionBuilderTsSdk {
  pub(crate) builder: NativeTsTransactionBuilderBindingWrapper,
}

impl TransactionBuilderTsSdk {
  pub fn new(builder: NativeTsTransactionBuilderBindingWrapper) -> Self {
    TransactionBuilderTsSdk { builder }
  }
}

impl TransactionBuilderT for TransactionBuilderTsSdk {
  type Error = TsSdkError;
  type NativeTxBuilder = NativeTsTransactionBuilderBindingWrapper;

  fn finish(self) -> Result<ProgrammableTransactionBcs, TsSdkError> {
    futures::executor::block_on(self.builder.build())
      .map(|js_arr| js_arr.to_vec())
      .map_err(WasmError::from)
      .map_err(Self::Error::from)
  }

  fn as_native_tx_builder(&mut self) -> &mut Self::NativeTxBuilder {
    &mut self.builder
  }

  fn into_native_tx_builder(self) -> Self::NativeTxBuilder {
    self.builder
  }
}

impl Default for TransactionBuilderTsSdk {
  fn default() -> Self {
    unimplemented!();
  }
}

impl Deref for TransactionBuilderTsSdk {
  type Target = NativeTsTransactionBuilderBindingWrapper;

  fn deref(&self) -> &Self::Target {
    &self.builder
  }
}

impl DerefMut for TransactionBuilderTsSdk {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.builder
  }
}
