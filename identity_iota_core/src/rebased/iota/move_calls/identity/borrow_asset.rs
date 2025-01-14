// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use iota_sdk::rpc_types::IotaObjectData;
use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::base_types::ObjectType;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::Argument;
use iota_sdk::types::transaction::ObjectArg;
use iota_sdk::types::transaction::ProgrammableTransaction;
use itertools::Itertools;
use move_core_types::ident_str;

use crate::rebased::iota::move_calls::utils;
use crate::rebased::proposals::BorrowAction;
use crate::rebased::utils::MoveType;

use super::ProposalContext;

fn borrow_proposal_impl(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  objects: Vec<ObjectID>,
  expiration: Option<u64>,
  package_id: ObjectID,
) -> anyhow::Result<ProposalContext> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, cap_arg, package_id);
  let identity_arg = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
  let exp_arg = utils::option_to_move(expiration, &mut ptb, package_id)?;
  let objects_arg = ptb.pure(objects)?;

  let proposal_id = ptb.programmable_move_call(
    package_id,
    ident_str!("identity").into(),
    ident_str!("propose_borrow").into(),
    vec![],
    vec![identity_arg, delegation_token, exp_arg, objects_arg],
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

pub(crate) fn propose_borrow(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  objects: Vec<ObjectID>,
  expiration: Option<u64>,
  package_id: ObjectID,
) -> Result<ProgrammableTransaction, anyhow::Error> {
  let ProposalContext {
    mut ptb,
    controller_cap,
    delegation_token,
    borrow,
    ..
  } = borrow_proposal_impl(identity, capability, objects, expiration, package_id)?;

  utils::put_back_delegation_token(&mut ptb, controller_cap, delegation_token, borrow, package_id);

  Ok(ptb.finish())
}

fn execute_borrow_impl<F>(
  ptb: &mut ProgrammableTransactionBuilder,
  identity: Argument,
  delegation_token: Argument,
  proposal_id: Argument,
  objects: Vec<IotaObjectData>,
  intent_fn: F,
  package: ObjectID,
) -> anyhow::Result<()>
where
  F: FnOnce(&mut ProgrammableTransactionBuilder, &HashMap<ObjectID, (Argument, IotaObjectData)>),
{
  // Get the proposal's action as argument.
  let borrow_action = ptb.programmable_move_call(
    package,
    ident_str!("identity").into(),
    ident_str!("execute_proposal").into(),
    vec![BorrowAction::move_type(package)],
    vec![identity, delegation_token, proposal_id],
  );

  // Borrow all the objects specified in the action.
  let obj_arg_map = objects
    .into_iter()
    .map(|obj_data| {
      let obj_ref = obj_data.object_ref();
      let ObjectType::Struct(obj_type) = obj_data.object_type()? else {
        unreachable!("move packages cannot be borrowed to begin with");
      };
      let recv_obj = ptb.obj(ObjectArg::Receiving(obj_ref))?;

      let obj_arg = ptb.programmable_move_call(
        package,
        ident_str!("identity").into(),
        ident_str!("execute_borrow").into(),
        vec![obj_type.into()],
        vec![identity, borrow_action, recv_obj],
      );

      Ok((obj_ref.0, (obj_arg, obj_data)))
    })
    .collect::<anyhow::Result<_>>()?;

  // Apply the user-defined operation.
  intent_fn(ptb, &obj_arg_map);

  // Put back all the objects.
  obj_arg_map.into_values().for_each(|(obj_arg, obj_data)| {
    let ObjectType::Struct(obj_type) = obj_data.object_type().expect("checked above") else {
      unreachable!("move packages cannot be borrowed to begin with");
    };
    ptb.programmable_move_call(
      package,
      ident_str!("borrow_proposal").into(),
      ident_str!("put_back").into(),
      vec![obj_type.into()],
      vec![borrow_action, obj_arg],
    );
  });

  // Consume the now empty borrow_action
  ptb.programmable_move_call(
    package,
    ident_str!("borrow_proposal").into(),
    ident_str!("conclude_borrow").into(),
    vec![],
    vec![borrow_action],
  );

  Ok(())
}

pub(crate) fn execute_borrow<F>(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  proposal_id: ObjectID,
  objects: Vec<IotaObjectData>,
  intent_fn: F,
  package: ObjectID,
) -> Result<ProgrammableTransaction, anyhow::Error>
where
  F: FnOnce(&mut ProgrammableTransactionBuilder, &HashMap<ObjectID, (Argument, IotaObjectData)>),
{
  let mut ptb = ProgrammableTransactionBuilder::new();
  let identity = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
  let controller_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, controller_cap, package);
  let proposal_id = ptb.pure(proposal_id)?;

  execute_borrow_impl(
    &mut ptb,
    identity,
    delegation_token,
    proposal_id,
    objects,
    intent_fn,
    package,
  )?;

  utils::put_back_delegation_token(&mut ptb, controller_cap, delegation_token, borrow, package);

  Ok(ptb.finish())
}

pub(crate) fn create_and_execute_borrow<F>(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  objects: Vec<IotaObjectData>,
  intent_fn: F,
  expiration: Option<u64>,
  package_id: ObjectID,
) -> anyhow::Result<ProgrammableTransaction>
where
  F: FnOnce(&mut ProgrammableTransactionBuilder, &HashMap<ObjectID, (Argument, IotaObjectData)>),
{
  let ProposalContext {
    mut ptb,
    controller_cap,
    delegation_token,
    borrow,
    identity,
    proposal_id,
  } = borrow_proposal_impl(
    identity,
    capability,
    objects.iter().map(|obj_data| obj_data.object_id).collect_vec(),
    expiration,
    package_id,
  )?;

  execute_borrow_impl(
    &mut ptb,
    identity,
    delegation_token,
    proposal_id,
    objects,
    intent_fn,
    package_id,
  )?;

  utils::put_back_delegation_token(&mut ptb, controller_cap, delegation_token, borrow, package_id);

  Ok(ptb.finish())
}
