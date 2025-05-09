// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_interaction::types::base_types::ObjectID;
use iota_interaction::types::base_types::ObjectRef;
use iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_interaction::types::transaction::Command;
use iota_interaction::types::transaction::ObjectArg;
use iota_interaction::types::transaction::ProgrammableTransaction;
use iota_interaction::ident_str;

use iota_interaction::MoveType;
use crate::rebased::Error;

pub(crate) fn delete<T>(asset: ObjectRef, package: ObjectID) -> Result<ProgrammableTransaction, Error>
where
  T: MoveType,
{
  let mut ptb = ProgrammableTransactionBuilder::new();

  let asset = ptb
    .obj(ObjectArg::ImmOrOwnedObject(asset))
    .map_err(|e| Error::InvalidArgument(e.to_string()))?;

  ptb.command(Command::move_call(
    package,
    ident_str!("asset").into(),
    ident_str!("delete").into(),
    vec![T::move_type(package)],
    vec![asset],
  ));

  Ok(ptb.finish())
}
