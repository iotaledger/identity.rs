// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::ObjectArg;
use iota_sdk::types::transaction::ProgrammableTransaction;
use move_core_types::ident_str;

use crate::sui::move_calls::utils;

pub fn propose_update(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  did_doc: impl AsRef<[u8]>,
  expiration: Option<u64>,
  package_id: ObjectID,
) -> Result<ProgrammableTransaction, anyhow::Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let identity_arg = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
  let exp_arg = utils::option_to_move(expiration, &mut ptb, package_id)?;
  let doc_arg = ptb.pure(did_doc.as_ref())?;

  let _proposal_id = ptb.programmable_move_call(
    package_id,
    ident_str!("identity").into(),
    ident_str!("propose_update").into(),
    vec![],
    vec![identity_arg, cap_arg, doc_arg, exp_arg],
  );

  Ok(ptb.finish())
}

pub fn execute_update(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  proposal_id: ObjectID,
  package_id: ObjectID,
) -> Result<ProgrammableTransaction, anyhow::Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let proposal_id = ptb.pure(proposal_id)?;
  let identity_arg = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;

  let _ = ptb.programmable_move_call(
    package_id,
    ident_str!("identity").into(),
    ident_str!("execute_update").into(),
    vec![],
    vec![identity_arg, cap_arg, proposal_id],
  );

  Ok(ptb.finish())
}
