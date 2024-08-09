use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::Command;
use iota_sdk::types::transaction::ProgrammableMoveCall;
use iota_sdk::types::transaction::ProgrammableTransaction;
use iota_sdk::types::Identifier;
use iota_sdk::types::TypeTag;
use iota_sdk::types::IOTA_FRAMEWORK_PACKAGE_ID;

use crate::sui::move_calls::utils;
use crate::utils::parse_identifier;

use crate::Error;

pub fn new(did_doc: &[u8], package_id: ObjectID) -> Result<ProgrammableTransaction, Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let doc_arg = utils::ptb_pure(&mut ptb, "did_doc", did_doc)?;

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

  let controllers = {
    let (ids, vps): (Vec<IotaAddress>, Vec<u64>) = controllers.into_iter().unzip();
    let ids = ptb.pure(ids).map_err(|e| Error::InvalidArgument(e.to_string()))?;
    let vps = ptb.pure(vps).map_err(|e| Error::InvalidArgument(e.to_string()))?;
    ptb.programmable_move_call(
      package_id,
      Identifier::new("utils").expect("valid utf8"),
      Identifier::new("vec_map_from_keys_values").expect("valid utf8"),
      vec![TypeTag::Address, TypeTag::U64],
      vec![ids, vps],
    )
  };
  let doc_arg = ptb.pure(did_doc).map_err(|e| Error::InvalidArgument(e.to_string()))?;
  let threshold_arg = ptb.pure(threshold).map_err(|e| Error::InvalidArgument(e.to_string()))?;

  // Create a new identity, sending its capabilities to the specified controllers.
  let identity_res = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
    package: package_id,
    module: parse_identifier("identity")?,
    function: parse_identifier("new_with_controllers")?,
    type_arguments: vec![],
    arguments: vec![doc_arg, controllers, threshold_arg],
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
