// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::{Deref, DerefMut};

use crate::iota_sdk_abstraction::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use crate::iota_sdk_abstraction::ProgrammableTransactionBcs;
use crate::iota_sdk_abstraction::TransactionBuilderT;
use crate::error::Error;

pub struct TransactionBuilderRustSdk {
    pub(crate) builder: ProgrammableTransactionBuilder,
}

impl TransactionBuilderRustSdk {
    pub fn new(builder: ProgrammableTransactionBuilder) -> Self {
        TransactionBuilderRustSdk {builder}
    }
}

impl TransactionBuilderT for TransactionBuilderRustSdk {
    fn finish(self) -> Result<ProgrammableTransactionBcs, Error> {
        let tx = self.builder.finish();
        Ok(bcs::to_bytes(&tx)?)
    }
}

impl Default for TransactionBuilderRustSdk {
    fn default() -> Self {
        TransactionBuilderRustSdk {
            builder: ProgrammableTransactionBuilder::default(),
        }
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