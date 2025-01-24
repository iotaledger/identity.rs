// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::ProgrammableTransactionBcs;

pub trait TransactionBuilderT {
    type Error;
    type NativeTxBuilder;

    fn finish(self) -> Result<ProgrammableTransactionBcs, Self::Error>;

    fn as_native_tx_builder(&mut self) -> &mut Self::NativeTxBuilder;

    fn into_native_tx_builder(self) -> Self::NativeTxBuilder;
}
