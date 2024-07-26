use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::Command;
use iota_sdk::types::transaction::ProgrammableMoveCall;
use iota_sdk::types::transaction::ProgrammableTransaction;
use iota_sdk::types::TypeTag;
use iota_sdk::types::IOTA_FRAMEWORK_PACKAGE_ID;

use crate::sui::move_calls::utils;
use crate::utils::parse_identifier;
use crate::utils::ptb_pure;

use crate::Error;

pub fn new(did_doc: &[u8], package_id: ObjectID) -> Result<ProgrammableTransaction, Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let doc_arg = ptb_pure(&mut ptb, "did_doc", did_doc)?;

  // Create a new identity, sending its capability to the tx's sender.
  let identity_res = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
    package: package_id,
    module: parse_identifier("identity")?,
    function: parse_identifier("new")?,
    type_arguments: vec![],
    arguments: vec![doc_arg],
  })));

  // Share the resulting identity.
  ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
    package: IOTA_FRAMEWORK_PACKAGE_ID,
    module: parse_identifier("transfer")?,
    function: parse_identifier("public_share_object")?,
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
    .map(|controller_addr| ptb_pure(&mut ptb, "controller entry", controller_addr))
    .collect::<Result<Vec<_>, _>>()?;
  let controllers_move_vec = ptb.command(Command::MakeMoveVec(Some(TypeTag::Address), controller_args));

  // Make voting powers move vector.
  let vp_args = vps
    .into_iter()
    .map(|vp| ptb_pure(&mut ptb, "voting power entry", vp))
    .collect::<Result<Vec<_>, _>>()?;
  let vps_move_vec = ptb.command(Command::MakeMoveVec(Some(TypeTag::U64), vp_args));

  // Make controllers VecMap
  let controllers_vec_map = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
    package: IOTA_FRAMEWORK_PACKAGE_ID,
    module: parse_identifier("vec_map")?,
    function: parse_identifier("from_keys_values")?,
    type_arguments: vec![TypeTag::Address, TypeTag::U64],
    arguments: vec![controllers_move_vec, vps_move_vec],
  })));

  let doc_arg = ptb_pure(&mut ptb, "did_doc", did_doc)?;
  let threshold_arg = ptb_pure(&mut ptb, "threshold", threshold)?;

  // Create a new identity, sending its capabilities to the specified controllers.
  let identity_res = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
    package: package_id,
    module: parse_identifier("identity")?,
    function: parse_identifier("new_with_controllers")?,
    type_arguments: vec![],
    arguments: vec![doc_arg, controllers_vec_map, threshold_arg],
  })));

  // Share the resulting identity.
  ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
    package: IOTA_FRAMEWORK_PACKAGE_ID,
    module: parse_identifier("transfer")?,
    function: parse_identifier("public_share_object")?,
    type_arguments: vec![TypeTag::Struct(Box::new(utils::identity_tag(package_id)?))],
    arguments: vec![identity_res],
  })));

  Ok(ptb.finish())
}
