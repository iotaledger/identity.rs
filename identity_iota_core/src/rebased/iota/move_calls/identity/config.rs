// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use std::str::FromStr;

use identity_iota_interaction::rpc_types::OwnedObjectRef;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use identity_iota_interaction::types::transaction::ObjectArg;
use identity_iota_interaction::types::transaction::ProgrammableTransaction;
use identity_iota_interaction::types::TypeTag;
use identity_iota_interaction::ident_str;

use super::super::utils;

#[allow(clippy::too_many_arguments)]
pub(crate) fn propose_config_change<I1, I2>(
  identity: OwnedObjectRef,
  controller_cap: ObjectRef,
  expiration: Option<u64>,
  threshold: Option<u64>,
  controllers_to_add: I1,
  controllers_to_remove: HashSet<ObjectID>,
  controllers_to_update: I2,
  package: ObjectID,
) -> anyhow::Result<ProgrammableTransaction>
where
  I1: IntoIterator<Item = (IotaAddress, u64)>,
  I2: IntoIterator<Item = (ObjectID, u64)>,
{
  let mut ptb = ProgrammableTransactionBuilder::new();

  let controllers_to_add = {
    let (addresses, vps): (Vec<IotaAddress>, Vec<u64>) = controllers_to_add.into_iter().unzip();
    let addresses = ptb.pure(addresses)?;
    let vps = ptb.pure(vps)?;

    ptb.programmable_move_call(
      package,
      ident_str!("utils").into(),
      ident_str!("vec_map_from_keys_values").into(),
      vec![TypeTag::Address, TypeTag::U64],
      vec![addresses, vps],
    )
  };
  let controllers_to_update = {
    let (ids, vps): (Vec<ObjectID>, Vec<u64>) = controllers_to_update.into_iter().unzip();
    let ids = ptb.pure(ids)?;
    let vps = ptb.pure(vps)?;

    ptb.programmable_move_call(
      package,
      ident_str!("utils").into(),
      ident_str!("vec_map_from_keys_values").into(),
      vec![TypeTag::from_str("0x2::object::ID").expect("valid utf8"), TypeTag::U64],
      vec![ids, vps],
    )
  };
  let identity = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
  let controller_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(controller_cap))?;
  let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, controller_cap, package);
  let expiration = utils::option_to_move(expiration, &mut ptb, package)?;
  let threshold = utils::option_to_move(threshold, &mut ptb, package)?;
  let controllers_to_remove = ptb.pure(controllers_to_remove)?;

  let _proposal_id = ptb.programmable_move_call(
    package,
    ident_str!("identity").into(),
    ident_str!("propose_config_change").into(),
    vec![],
    vec![
      identity,
      delegation_token,
      expiration,
      threshold,
      controllers_to_add,
      controllers_to_remove,
      controllers_to_update,
    ],
  );

  utils::put_back_delegation_token(&mut ptb, controller_cap, delegation_token, borrow, package);

  Ok(ptb.finish())
}

pub(crate) fn execute_config_change(
  identity: OwnedObjectRef,
  controller_cap: ObjectRef,
  proposal_id: ObjectID,
  package: ObjectID,
) -> anyhow::Result<ProgrammableTransaction> {
  let mut ptb = ProgrammableTransactionBuilder::new();

  let identity = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
  let controller_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(controller_cap))?;
  let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, controller_cap, package);
  let proposal_id = ptb.pure(proposal_id)?;
  ptb.programmable_move_call(
    package,
    ident_str!("identity").into(),
    ident_str!("execute_config_change").into(),
    vec![],
    vec![identity, delegation_token, proposal_id],
  );

  utils::put_back_delegation_token(&mut ptb, controller_cap, delegation_token, borrow, package);

  Ok(ptb.finish())
}
