// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::{Deref, DerefMut};

use identity_iota_interaction::ProgrammableTransactionBcs;
use identity_iota_interaction::TransactionBuilderT;
use crate::error::TsSdkError;

type NativeTsCodeBindingWrapper = ();

pub struct TransactionBuilderTsSdk {
    pub(crate) builder: NativeTsCodeBindingWrapper
}

impl TransactionBuilderTsSdk {
    pub fn new(builder: NativeTsCodeBindingWrapper) -> Self {
        TransactionBuilderTsSdk {builder}
    }
}

impl TransactionBuilderT for TransactionBuilderTsSdk {
    type Error = TsSdkError;

    fn finish(self) -> Result<ProgrammableTransactionBcs, TsSdkError> {
        unimplemented!();
    }
}

impl Default for TransactionBuilderTsSdk {
    fn default() -> Self {
        unimplemented!();
    }
}

impl Deref for TransactionBuilderTsSdk {
    type Target = NativeTsCodeBindingWrapper;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

impl DerefMut for TransactionBuilderTsSdk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.builder
    }
}