// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::ObjectArg;
use iota_sdk::types::transaction::ProgrammableTransaction;
use move_core_types::ident_str;

use crate::rebased::iota::move_calls::utils;

pub(crate) fn propose_deactivation(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  expiration: Option<u64>,
  package_id: ObjectID,
) -> Result<ProgrammableTransaction, anyhow::Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, cap_arg, package_id);
  let identity_arg = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
  let exp_arg = utils::option_to_move(expiration, &mut ptb, package_id)?;
  let clock = utils::get_clock_ref(&mut ptb);

  let _proposal_id = ptb.programmable_move_call(
    package_id,
    ident_str!("identity").into(),
    ident_str!("propose_deactivation").into(),
    vec![],
    vec![identity_arg, delegation_token, exp_arg, clock],
  );

  utils::put_back_delegation_token(&mut ptb, cap_arg, delegation_token, borrow, package_id);

  Ok(ptb.finish())
}

pub(crate) fn execute_deactivation(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  proposal_id: ObjectID,
  package_id: ObjectID,
) -> Result<ProgrammableTransaction, anyhow::Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, cap_arg, package_id);
  let proposal_id = ptb.pure(proposal_id)?;
  let identity_arg = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
  let clock = utils::get_clock_ref(&mut ptb);

  let _ = ptb.programmable_move_call(
    package_id,
    ident_str!("identity").into(),
    ident_str!("execute_deactivation").into(),
    vec![],
    vec![identity_arg, delegation_token, proposal_id, clock],
  );

  utils::put_back_delegation_token(&mut ptb, cap_arg, delegation_token, borrow, package_id);

  Ok(ptb.finish())
}
