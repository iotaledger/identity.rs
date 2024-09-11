use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::Command;
use iota_sdk::types::transaction::ObjectArg;
use iota_sdk::types::transaction::ProgrammableTransaction;
use move_core_types::ident_str;
use serde::Serialize;

use crate::utils::MoveType;
use crate::Error;

pub fn update<T>(asset: ObjectRef, new_content: T, package: ObjectID) -> Result<ProgrammableTransaction, Error>
where
  T: MoveType + Serialize,
{
  let mut ptb = ProgrammableTransactionBuilder::new();

  let asset = ptb
    .obj(ObjectArg::ImmOrOwnedObject(asset))
    .map_err(|e| Error::InvalidArgument(e.to_string()))?;
  let new_content = ptb
    .pure(new_content)
    .map_err(|e| Error::InvalidArgument(e.to_string()))?;

  ptb.command(Command::move_call(
    package,
    ident_str!("asset").into(),
    ident_str!("set_content").into(),
    vec![T::move_type(package)],
    vec![asset, new_content],
  ));

  Ok(ptb.finish())
}
