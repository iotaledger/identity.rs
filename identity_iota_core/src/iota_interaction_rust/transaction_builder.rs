// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;
use std::ops::DerefMut;

use crate::rebased::Error;
use identity_iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use identity_iota_interaction::ProgrammableTransactionBcs;
use identity_iota_interaction::TransactionBuilderT;

#[derive(Default)]
pub struct TransactionBuilderRustSdk {
  pub builder: ProgrammableTransactionBuilder,
}

impl TransactionBuilderRustSdk {
  pub fn new(builder: ProgrammableTransactionBuilder) -> Self {
    TransactionBuilderRustSdk { builder }
  }
}

impl TransactionBuilderT for TransactionBuilderRustSdk {
  type Error = Error;
  type NativeTxBuilder = ProgrammableTransactionBuilder;

  fn finish(self) -> Result<ProgrammableTransactionBcs, Error> {
    let tx = self.builder.finish();
    Ok(bcs::to_bytes(&tx)?)
  }

  fn as_native_tx_builder(&mut self) -> &mut Self::NativeTxBuilder {
    &mut self.builder
  }

  fn into_native_tx_builder(self) -> Self::NativeTxBuilder {
    self.builder
  }
}

impl Deref for TransactionBuilderRustSdk {
  type Target = ProgrammableTransactionBuilder;

  fn deref(&self) -> &Self::Target {
    &self.builder
  }
}

impl DerefMut for TransactionBuilderRustSdk {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.builder
  }
}
