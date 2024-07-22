use std::str::FromStr;

use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::ProgrammableTransaction;
use iota_sdk::types::Identifier;

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
  let cap_arg = ptb.pure(capability)?;
  let key_arg = ptb.pure(key)?;
  let identity_arg = ptb.pure(identity)?;
  let exp_arg = ptb.pure(expiration)?;
  let doc_arg = ptb.pure(did_doc.as_ref())?;

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
  let mut ptb = ProgrammableTransactionBuilder::new();
  let cap_arg = ptb.pure(capability)?;
  let key_arg = ptb.pure(key)?;
  let identity_arg = ptb.pure(identity)?;

  let _ = ptb.programmable_move_call(
    package_id,
    Identifier::from_str("identity")?,
    Identifier::from_str("execute_update")?,
    vec![],
    vec![identity_arg, cap_arg, key_arg],
  );

  Ok(ptb.finish())
}
