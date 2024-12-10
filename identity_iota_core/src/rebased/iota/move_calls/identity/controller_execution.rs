// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::Argument;
use iota_sdk::types::transaction::ObjectArg;
use iota_sdk::types::transaction::ProgrammableTransaction;
use move_core_types::ident_str;

use crate::rebased::iota::move_calls::utils;
use crate::rebased::proposals::ControllerExecution;
use crate::rebased::utils::MoveType;

use super::ProposalContext;

fn controller_execution_impl(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  controller_cap_id: ObjectID,
  expiration: Option<u64>,
  package_id: ObjectID,
) -> anyhow::Result<ProposalContext> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, cap_arg, package_id);
  let identity_arg = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
  let controller_cap_id = ptb.pure(controller_cap_id)?;
  let exp_arg = utils::option_to_move(expiration, &mut ptb, package_id)?;

  let proposal_id = ptb.programmable_move_call(
    package_id,
    ident_str!("identity").into(),
    ident_str!("propose_controller_execution").into(),
    vec![],
    vec![identity_arg, delegation_token, controller_cap_id, exp_arg],
  );

  Ok(ProposalContext {
    ptb,
    controller_cap: cap_arg,
    delegation_token,
    borrow,
    identity: identity_arg,
    proposal_id,
  })
}

pub(crate) fn propose_controller_execution(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  controller_cap_id: ObjectID,
  expiration: Option<u64>,
  package_id: ObjectID,
) -> Result<ProgrammableTransaction, anyhow::Error> {
  let ProposalContext {
    mut ptb,
    controller_cap,
    delegation_token,
    borrow,
    ..
  } = controller_execution_impl(identity, capability, controller_cap_id, expiration, package_id)?;
  utils::put_back_delegation_token(&mut ptb, controller_cap, delegation_token, borrow, package_id);

  Ok(ptb.finish())
}

fn execute_controller_execution_impl<F>(
  ptb: &mut ProgrammableTransactionBuilder,
  identity: Argument,
  proposal_id: Argument,
  delegation_token: Argument,
  borrowing_controller_cap_ref: ObjectRef,
  intent_fn: F,
  package: ObjectID,
) -> anyhow::Result<()>
where
  F: FnOnce(&mut ProgrammableTransactionBuilder, &Argument),
{
  // Get the proposal's action as argument.
  let controller_execution_action = ptb.programmable_move_call(
    package,
    ident_str!("identity").into(),
    ident_str!("execute_proposal").into(),
    vec![ControllerExecution::move_type(package)],
    vec![identity, delegation_token, proposal_id],
  );

  // Borrow the controller cap into this transaction.
  let receiving = ptb.obj(ObjectArg::Receiving(borrowing_controller_cap_ref))?;
  let borrowed_controller_cap = ptb.programmable_move_call(
    package,
    ident_str!("identity").into(),
    ident_str!("borrow_controller_cap").into(),
    vec![],
    vec![identity, controller_execution_action, receiving],
  );

  // Apply the user-defined operation.
  intent_fn(ptb, &borrowed_controller_cap);

  // Put back the borrowed controller cap.
  ptb.programmable_move_call(
    package,
    ident_str!("controller_proposal").into(),
    ident_str!("put_back").into(),
    vec![],
    vec![controller_execution_action, borrowed_controller_cap],
  );

  Ok(())
}

pub(crate) fn execute_controller_execution<F>(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  proposal_id: ObjectID,
  borrowing_controller_cap_ref: ObjectRef,
  intent_fn: F,
  package: ObjectID,
) -> Result<ProgrammableTransaction, anyhow::Error>
where
  F: FnOnce(&mut ProgrammableTransactionBuilder, &Argument),
{
  let mut ptb = ProgrammableTransactionBuilder::new();
  let identity = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
  let controller_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, controller_cap, package);
  let proposal_id = ptb.pure(proposal_id)?;

  execute_controller_execution_impl(
    &mut ptb,
    identity,
    proposal_id,
    delegation_token,
    borrowing_controller_cap_ref,
    intent_fn,
    package,
  )?;

  utils::put_back_delegation_token(&mut ptb, controller_cap, delegation_token, borrow, package);

  Ok(ptb.finish())
}

pub(crate) fn create_and_execute_controller_execution<F>(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  expiration: Option<u64>,
  borrowing_controller_cap_ref: ObjectRef,
  intent_fn: F,
  package_id: ObjectID,
) -> anyhow::Result<ProgrammableTransaction>
where
  F: FnOnce(&mut ProgrammableTransactionBuilder, &Argument),
{
  let ProposalContext {
    mut ptb,
    controller_cap,
    delegation_token,
    borrow,
    proposal_id,
    identity,
  } = controller_execution_impl(
    identity,
    capability,
    borrowing_controller_cap_ref.0,
    expiration,
    package_id,
  )?;

  execute_controller_execution_impl(
    &mut ptb,
    identity,
    proposal_id,
    delegation_token,
    borrowing_controller_cap_ref,
    intent_fn,
    package_id,
  )?;

  utils::put_back_delegation_token(&mut ptb, controller_cap, delegation_token, borrow, package_id);

  Ok(ptb.finish())
}
