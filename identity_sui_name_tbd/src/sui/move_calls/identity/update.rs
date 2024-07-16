use std::str::FromStr;

use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::object::Owner;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::ObjectArg;
use iota_sdk::types::transaction::ProgrammableTransaction;
use iota_sdk::types::Identifier;
use iota_sdk::types::TypeTag;
use iota_sdk::types::MOVE_STDLIB_PACKAGE_ID;

use crate::sui::move_calls::utils;

pub fn propose_update(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  key: &str,
  did_doc: impl AsRef<[u8]>,
  expiration: Option<u64>,
  package_id: ObjectID,
) -> Result<ProgrammableTransaction, anyhow::Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let key_arg = utils::str_to_move_string(key, &mut ptb)?;
  let initial_shared_version = if let Owner::Shared { initial_shared_version } = identity.owner {
    initial_shared_version
  } else {
    panic!("nope");
  };
  let identity_arg = ptb.obj(ObjectArg::SharedObject {
    id: identity.reference.object_id,
    initial_shared_version,
    mutable: true,
  })?;
  let exp_arg = if let Some(exp) = expiration {
    let arg = ptb.pure(exp)?;
    ptb.programmable_move_call(
      MOVE_STDLIB_PACKAGE_ID,
      Identifier::from_str("option").unwrap(),
      Identifier::from_str("some").unwrap(),
      vec![TypeTag::U64],
      vec![arg],
    )
  } else {
    ptb.programmable_move_call(
      MOVE_STDLIB_PACKAGE_ID,
      Identifier::from_str("option").unwrap(),
      Identifier::from_str("none").unwrap(),
      vec![TypeTag::U64],
      vec![],
    )
  };
  let doc_arg = utils::bytes_to_move_vec(did_doc.as_ref(), &mut ptb)?;

  let _ = ptb.programmable_move_call(
    package_id,
    Identifier::from_str("identity")?,
    Identifier::from_str("propose_update")?,
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
) -> Result<ProgrammableTransaction, anyhow::Error> {
  let initial_shared_version = if let Owner::Shared { initial_shared_version } = identity.owner {
    initial_shared_version
  } else {
    panic!("nope");
  };
  let mut ptb = ProgrammableTransactionBuilder::new();
  let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let key_arg = utils::str_to_move_string(key, &mut ptb)?;
  let identity_arg = ptb.obj(ObjectArg::SharedObject {
    id: identity.reference.object_id,
    initial_shared_version,
    mutable: true,
  })?;

  let _ = ptb.programmable_move_call(
    package_id,
    Identifier::from_str("identity")?,
    Identifier::from_str("execute_update")?,
    vec![],
    vec![identity_arg, cap_arg, key_arg],
  );

  Ok(ptb.finish())
}
