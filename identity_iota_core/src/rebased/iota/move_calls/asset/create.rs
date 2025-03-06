// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use identity_iota_interaction::types::transaction::Command;
use identity_iota_interaction::types::transaction::ProgrammableMoveCall;
use identity_iota_interaction::types::transaction::ProgrammableTransaction;
use identity_iota_interaction::ident_str;
use serde::Serialize;

use identity_iota_interaction::MoveType;
use crate::rebased::Error;
use super::try_to_argument;

pub(crate) fn new<T: Serialize + MoveType>(
  inner: T,
  mutable: bool,
  transferable: bool,
  deletable: bool,
  package: ObjectID,
) -> Result<ProgrammableTransaction, Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let inner = try_to_argument(&inner, &mut ptb, package)?;
  let mutable = ptb.pure(mutable).map_err(|e| Error::InvalidArgument(e.to_string()))?;
  let transferable = ptb
    .pure(transferable)
    .map_err(|e| Error::InvalidArgument(e.to_string()))?;
  let deletable = ptb.pure(deletable).map_err(|e| Error::InvalidArgument(e.to_string()))?;

  ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
    package,
    module: ident_str!("asset").into(),
    function: ident_str!("new_with_config").into(),
    type_arguments: vec![T::move_type(package)],
    arguments: vec![inner, mutable, transferable, deletable],
  })));

  Ok(ptb.finish())
}
