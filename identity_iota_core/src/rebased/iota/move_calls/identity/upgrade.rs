// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_interaction::rpc_types::OwnedObjectRef;
use iota_interaction::types::base_types::ObjectID;
use iota_interaction::types::base_types::ObjectRef;
use iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_interaction::types::transaction::ObjectArg;
use iota_interaction::types::transaction::ProgrammableTransaction;
use iota_interaction::ident_str;

use crate::rebased::iota::move_calls::utils;

pub(crate) fn propose_upgrade(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  expiration: Option<u64>,
  package_id: ObjectID,
) -> Result<ProgrammableTransaction, anyhow::Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let identity_arg = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
  let exp_arg = utils::option_to_move(expiration, &mut ptb, package_id)?;

  let _proposal_id = ptb.programmable_move_call(
    package_id,
    ident_str!("identity").into(),
    ident_str!("propose_upgrade").into(),
    vec![],
    vec![identity_arg, cap_arg, exp_arg],
  );

  Ok(ptb.finish())
}

pub(crate) fn execute_upgrade(
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
    ident_str!("execute_upgrade").into(),
    vec![],
    vec![identity_arg, cap_arg, proposal_id],
  );

  Ok(ptb.finish())
}
