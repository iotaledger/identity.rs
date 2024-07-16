use crate::Error;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder as Ptb;
use iota_sdk::types::transaction::Argument;
use iota_sdk::types::transaction::Command;
use iota_sdk::types::Identifier;
use iota_sdk::types::MOVE_STDLIB_PACKAGE_ID;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::StructTag;
use std::str::FromStr;

pub fn bytes_to_move_vec<'b, B>(bytes: B, ptb: &mut Ptb) -> Result<Argument, Error>
where
  B: IntoIterator<Item = &'b u8>,
{
  let args = bytes
    .into_iter()
    .map(|b| ptb.pure(b))
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| Error::InvalidArgument(format!("could not convert given document to move vector; {e}")))?;

  Ok(ptb.command(Command::MakeMoveVec(Some(iota_sdk::types::TypeTag::U8), args)))
}

pub fn identity_tag(package_id: ObjectID) -> Result<StructTag, Error> {
  Ok(StructTag {
    address: AccountAddress::from_str(&package_id.to_string())
      .map_err(|err| Error::ParsingFailed(format!("package id\"{package_id}\" to account address; {err}")))?,
    module: Identifier::from_str("identity")
      .map_err(|err| Error::ParsingFailed(format!("\"identity\" to identifier; {err}")))?,
    name: Identifier::from_str("Identity")
      .map_err(|err| Error::ParsingFailed(format!("\"Identity\" to identifier; {err}")))?,
    type_params: vec![],
  })
}

pub fn str_to_move_string(s: impl AsRef<str>, ptb: &mut Ptb) -> Result<Argument, anyhow::Error> {
  let str_bytes = bytes_to_move_vec(s.as_ref().as_bytes(), ptb)?;
  Ok(ptb.programmable_move_call(
    MOVE_STDLIB_PACKAGE_ID,
    Identifier::from_str("string")?,
    Identifier::from_str("utf8")?,
    vec![],
    vec![str_bytes],
  ))
}
