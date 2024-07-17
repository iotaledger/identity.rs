use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::Command;
use iota_sdk::types::transaction::ProgrammableMoveCall;
use iota_sdk::types::transaction::ProgrammableTransaction;
use iota_sdk::types::Identifier;
use iota_sdk::types::TypeTag;
use iota_sdk::types::IOTA_FRAMEWORK_PACKAGE_ID;
use std::str::FromStr;

use crate::sui::move_calls::utils;
use crate::Error;

pub fn new(did_doc: &[u8], package_id: ObjectID) -> Result<ProgrammableTransaction, Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let doc_arg = utils::bytes_to_move_vec(did_doc, &mut ptb)?;

  // Create a new identity, sending its capability to the tx's sender.
  let identity_res = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
    package: package_id,
    module: Identifier::from_str("identity")
      .map_err(|err| Error::ParsingFailed(format!("\"document\" to identifier; {err}")))?,
    function: Identifier::from_str("new")
      .map_err(|err| Error::ParsingFailed(format!("\"new\" to identifier; {err}")))?,
    type_arguments: vec![],
    arguments: vec![doc_arg],
  })));

  // Share the resulting identity.
  ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
    package: IOTA_FRAMEWORK_PACKAGE_ID,
    module: Identifier::from_str("transfer")
      .map_err(|err| Error::ParsingFailed(format!("\"transfer\" to identifier; {err}")))?,
    function: Identifier::from_str("public_share_object")
      .map_err(|err| Error::ParsingFailed(format!("\"public_share_object\" to identifier; {err}")))?,
    type_arguments: vec![TypeTag::Struct(Box::new(utils::identity_tag(package_id)?))],
    arguments: vec![identity_res],
  })));

  Ok(ptb.finish())
}

pub fn new_with_controllers<C>(
  did_doc: &[u8],
  controllers: C,
  threshold: u64,
  package_id: ObjectID,
) -> Result<ProgrammableTransaction, Error>
where
  C: IntoIterator<Item = (IotaAddress, u64)>,
{
  let mut ptb = ProgrammableTransactionBuilder::new();
  let (controllers, vps): (Vec<IotaAddress>, Vec<u64>) = controllers.into_iter().unzip();
  // Make controllers move vector.
  let controller_args = controllers
    .into_iter()
    .map(|controller_addr| ptb.pure(controller_addr))
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| Error::InvalidArgument(e.to_string()))?;
  let controllers_move_vec = ptb.command(Command::MakeMoveVec(Some(TypeTag::Address), controller_args));

  // Make voting powers move vector.
  let vp_args = vps
    .into_iter()
    .map(|vp| ptb.pure(vp))
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| Error::InvalidArgument(e.to_string()))?;
  let vps_move_vec = ptb.command(Command::MakeMoveVec(Some(TypeTag::U64), vp_args));

  // Make controllers VecMap
  let controllers_vec_map = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
    package: IOTA_FRAMEWORK_PACKAGE_ID,
    module: Identifier::from_str("vec_map").map_err(|e| Error::ParsingFailed(e.to_string()))?,
    function: Identifier::from_str("from_keys_values").map_err(|e| Error::ParsingFailed(e.to_string()))?,
    type_arguments: vec![TypeTag::Address, TypeTag::U64],
    arguments: vec![controllers_move_vec, vps_move_vec],
  })));

  let doc_arg = utils::bytes_to_move_vec(did_doc, &mut ptb)?;
  let threshold_arg = ptb.pure(threshold).map_err(|e| Error::InvalidArgument(e.to_string()))?;

  // Create a new identity, sending its capabilities to the specified controllers.
  let identity_res = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
    package: package_id,
    module: Identifier::from_str("identity")
      .map_err(|err| Error::ParsingFailed(format!("\"identity\" to identifier; {err}")))?,
    function: Identifier::from_str("new_with_controllers")
      .map_err(|err| Error::ParsingFailed(format!("\"new_with_controllers\" to identifier; {err}")))?,
    type_arguments: vec![],
    arguments: vec![doc_arg, controllers_vec_map, threshold_arg],
  })));

  // Share the resulting identity.
  ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
    package: IOTA_FRAMEWORK_PACKAGE_ID,
    module: Identifier::from_str("transfer")
      .map_err(|err| Error::ParsingFailed(format!("\"transfer\" to identifier; {err}")))?,
    function: Identifier::from_str("public_share_object")
      .map_err(|err| Error::ParsingFailed(format!("\"public_share_object\" to identifier; {err}")))?,
    type_arguments: vec![TypeTag::Struct(Box::new(utils::identity_tag(package_id)?))],
    arguments: vec![identity_res],
  })));

  Ok(ptb.finish())
}
