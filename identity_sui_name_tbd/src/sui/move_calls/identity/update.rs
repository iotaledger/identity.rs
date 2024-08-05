use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::object::Owner;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::ObjectArg;
use iota_sdk::types::transaction::ProgrammableTransaction;
use iota_sdk::types::TypeTag;
use iota_sdk::types::MOVE_STDLIB_PACKAGE_ID;

use crate::utils::parse_identifier;
use crate::utils::ptb_obj;
use crate::utils::ptb_pure;
use crate::Error;

pub fn propose_update(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  key: &str,
  did_doc: impl AsRef<[u8]>,
  expiration: Option<u64>,
  package_id: ObjectID,
) -> Result<ProgrammableTransaction, Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let cap_arg = ptb_obj(&mut ptb, "capability", ObjectArg::ImmOrOwnedObject(capability))?;
  let key_arg = ptb_pure(&mut ptb, "key", key)?;
  let initial_shared_version = if let Owner::Shared { initial_shared_version } = identity.owner {
    initial_shared_version
  } else {
    return Err(Error::InvalidArgument(format!(
      "given identity {} is not a shared object",
      identity.reference.object_id
    )));
  };
  let identity_arg = ptb_obj(
    &mut ptb,
    "identity",
    ObjectArg::SharedObject {
      id: identity.reference.object_id,
      initial_shared_version,
      mutable: true,
    },
  )?;
  let exp_arg = if let Some(exp) = expiration {
    let arg = ptb_pure(&mut ptb, "exp", exp)?;
    ptb.programmable_move_call(
      MOVE_STDLIB_PACKAGE_ID,
      parse_identifier("option")?,
      parse_identifier("some")?,
      vec![TypeTag::U64],
      vec![arg],
    )
  } else {
    ptb.programmable_move_call(
      MOVE_STDLIB_PACKAGE_ID,
      parse_identifier("option")?,
      parse_identifier("none")?,
      vec![TypeTag::U64],
      vec![],
    )
  };
  let doc_arg = ptb_pure(&mut ptb, "did_doc", did_doc.as_ref().to_vec())?;

  let _ = ptb.programmable_move_call(
    package_id,
    parse_identifier("identity")?,
    parse_identifier("propose_update")?,
    vec![],
    vec![identity_arg, cap_arg, key_arg, doc_arg, exp_arg],
  );

  Ok(ptb.finish())
}

pub fn execute_update(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  key: &str,
  package_id: ObjectID,
) -> Result<ProgrammableTransaction, Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let cap_arg = ptb_obj(&mut ptb, "capability", ObjectArg::ImmOrOwnedObject(capability))?;
  let key_arg = ptb_pure(&mut ptb, "key", key)?;
  let initial_shared_version = if let Owner::Shared { initial_shared_version } = identity.owner {
    initial_shared_version
  } else {
    return Err(Error::Identity(format!(
      "given identity {} is not a shared object",
      identity.reference.object_id
    )));
  };
  let identity_arg = ptb_obj(
    &mut ptb,
    "identity",
    ObjectArg::SharedObject {
      id: identity.reference.object_id,
      initial_shared_version,
      mutable: true,
    },
  )?;

  let _ = ptb.programmable_move_call(
    package_id,
    parse_identifier("identity")?,
    parse_identifier("execute_update")?,
    vec![],
    vec![identity_arg, cap_arg, key_arg],
  );

  Ok(ptb.finish())
}
