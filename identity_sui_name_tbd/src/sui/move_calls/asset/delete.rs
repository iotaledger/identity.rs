use std::str::FromStr;

use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::Command;
use iota_sdk::types::transaction::ObjectArg;
use iota_sdk::types::transaction::ProgrammableTransaction;
use iota_sdk::types::Identifier;

use crate::utils::MoveType;
use crate::Error;

pub fn delete<T>(asset: ObjectRef, package: ObjectID) -> Result<ProgrammableTransaction, Error>
where
  T: MoveType,
{
  let mut ptb = ProgrammableTransactionBuilder::new();

  let asset = ptb
    .obj(ObjectArg::ImmOrOwnedObject(asset))
    .map_err(|e| Error::InvalidArgument(e.to_string()))?;

  ptb.command(Command::move_call(
    package,
    Identifier::from_str("asset").map_err(|e| Error::ParsingFailed(e.to_string()))?,
    Identifier::from_str("delete").map_err(|e| Error::ParsingFailed(e.to_string()))?,
    vec![T::move_type(package)],
    vec![asset],
  ));

  Ok(ptb.finish())
}
