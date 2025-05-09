// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_interaction::rpc_types::OwnedObjectRef;
use iota_interaction::types::base_types::IotaAddress;
use iota_interaction::types::base_types::ObjectID;
use iota_interaction::types::base_types::ObjectRef;
use iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_interaction::types::transaction::Argument;
use iota_interaction::types::transaction::ObjectArg;
use iota_interaction::types::transaction::ProgrammableTransaction;
use iota_interaction::types::TypeTag;
use iota_interaction::ident_str;

use crate::rebased::iota::move_calls;
use crate::rebased::proposals::SendAction;
use iota_interaction::MoveType;

use self::move_calls::utils;
use super::ProposalContext;

fn send_proposal_impl(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  transfer_map: Vec<(ObjectID, IotaAddress)>,
  expiration: Option<u64>,
  package_id: ObjectID,
) -> anyhow::Result<ProposalContext> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let (delegation_token, borrow) = move_calls::utils::get_controller_delegation(&mut ptb, cap_arg, package_id);
  let identity_arg = move_calls::utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
  let exp_arg = move_calls::utils::option_to_move(expiration, &mut ptb, package_id)?;
  let (objects, recipients) = {
    let (objects, recipients): (Vec<_>, Vec<_>) = transfer_map.into_iter().unzip();
    let objects = ptb.pure(objects)?;
    let recipients = ptb.pure(recipients)?;

    (objects, recipients)
  };

  let proposal_id = ptb.programmable_move_call(
    package_id,
    ident_str!("identity").into(),
    ident_str!("propose_send").into(),
    vec![],
    vec![identity_arg, delegation_token, exp_arg, objects, recipients],
  );

  Ok(ProposalContext {
    ptb,
    identity: identity_arg,
    controller_cap: cap_arg,
    delegation_token,
    borrow,
    proposal_id,
  })
}

pub(crate) fn propose_send(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  transfer_map: Vec<(ObjectID, IotaAddress)>,
  expiration: Option<u64>,
  package_id: ObjectID,
) -> Result<ProgrammableTransaction, anyhow::Error> {
  let ProposalContext {
    mut ptb,
    controller_cap,
    delegation_token,
    borrow,
    ..
  } = send_proposal_impl(identity, capability, transfer_map, expiration, package_id)?;

  utils::put_back_delegation_token(&mut ptb, controller_cap, delegation_token, borrow, package_id);

  Ok(ptb.finish())
}

fn execute_send_impl(
  ptb: &mut ProgrammableTransactionBuilder,
  identity: Argument,
  delegation_token: Argument,
  proposal_id: Argument,
  objects: Vec<(ObjectRef, TypeTag)>,
  package: ObjectID,
) -> anyhow::Result<()> {
  // Get the proposal's action as argument.
  let send_action = ptb.programmable_move_call(
    package,
    ident_str!("identity").into(),
    ident_str!("execute_proposal").into(),
    vec![SendAction::move_type(package)],
    vec![identity, delegation_token, proposal_id],
  );

  // Send each object in this send action.
  // Traversing the map in reverse reduces the number of operations on the move side.
  for (obj, obj_type) in objects.into_iter().rev() {
    let recv_obj = ptb.obj(ObjectArg::Receiving(obj))?;

    ptb.programmable_move_call(
      package,
      ident_str!("identity").into(),
      ident_str!("execute_send").into(),
      vec![obj_type],
      vec![identity, send_action, recv_obj],
    );
  }

  // Consume the now empty send_action
  ptb.programmable_move_call(
    package,
    ident_str!("transfer_proposal").into(),
    ident_str!("complete_send").into(),
    vec![],
    vec![send_action],
  );

  Ok(())
}

pub(crate) fn execute_send(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  proposal_id: ObjectID,
  objects: Vec<(ObjectRef, TypeTag)>,
  package: ObjectID,
) -> Result<ProgrammableTransaction, anyhow::Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let identity = move_calls::utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
  let controller_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let (delegation_token, borrow) = move_calls::utils::get_controller_delegation(&mut ptb, controller_cap, package);
  let proposal_id = ptb.pure(proposal_id)?;

  execute_send_impl(&mut ptb, identity, delegation_token, proposal_id, objects, package)?;

  utils::put_back_delegation_token(&mut ptb, controller_cap, delegation_token, borrow, package);

  Ok(ptb.finish())
}

pub(crate) fn create_and_execute_send(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  transfer_map: Vec<(ObjectID, IotaAddress)>,
  expiration: Option<u64>,
  objects: Vec<(ObjectRef, TypeTag)>,
  package: ObjectID,
) -> anyhow::Result<ProgrammableTransaction> {
  let ProposalContext {
    mut ptb,
    identity,
    controller_cap,
    delegation_token,
    borrow,
    proposal_id,
  } = send_proposal_impl(identity, capability, transfer_map, expiration, package)?;

  execute_send_impl(&mut ptb, identity, delegation_token, proposal_id, objects, package)?;

  utils::put_back_delegation_token(&mut ptb, controller_cap, delegation_token, borrow, package);

  Ok(ptb.finish())
}
