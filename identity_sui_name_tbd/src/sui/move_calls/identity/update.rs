use std::str::FromStr;

use sui_sdk::types::base_types::ObjectID;
use sui_sdk::types::base_types::ObjectRef;
use sui_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_sdk::types::transaction::ObjectArg;
use sui_sdk::types::transaction::ProgrammableTransaction;
use sui_sdk::types::Identifier;
use sui_sdk::types::TypeTag;
use sui_sdk::types::MOVE_STDLIB_PACKAGE_ID;

use crate::sui::move_calls::utils;

pub fn propose_update(
  identity: ObjectRef,
  capability: ObjectRef,
  key: &str,
  did_doc: impl AsRef<[u8]>,
  expiration: Option<u64>,
  package_id: ObjectID,
) -> Result<ProgrammableTransaction, anyhow::Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let key_arg = utils::str_to_move_string(key, &mut ptb)?;
  let identity_arg = ptb.obj(ObjectArg::SharedObject {
    id: identity.0,
    initial_shared_version: identity.1,
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
  identity: ObjectRef,
  capability: ObjectRef,
  key: &str,
  package_id: ObjectID,
) -> Result<ProgrammableTransaction, anyhow::Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let key_arg = utils::str_to_move_string(key, &mut ptb)?;
  let identity_arg = ptb.obj(ObjectArg::SharedObject {
    id: identity.0,
    initial_shared_version: identity.1,
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
