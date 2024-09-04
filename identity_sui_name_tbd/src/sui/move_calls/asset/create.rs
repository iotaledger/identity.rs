use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::Command;
use iota_sdk::types::transaction::ProgrammableMoveCall;
use iota_sdk::types::transaction::ProgrammableTransaction;
use move_core_types::ident_str;
use serde::Serialize;

use crate::utils::MoveType;
use crate::Error;

pub fn new<T: Serialize + MoveType>(
  inner: T,
  mutable: bool,
  transferable: bool,
  deletable: bool,
  package: ObjectID,
) -> Result<ProgrammableTransaction, Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let inner = ptb.pure(inner).map_err(|e| Error::InvalidArgument(e.to_string()))?;
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
