// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::ProgrammableTransactionBcs;

pub trait TransactionBuilderT: Default {
    type Error;

    fn finish(self) -> Result<ProgrammableTransactionBcs, Self::Error>;
}