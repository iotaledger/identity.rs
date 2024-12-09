// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::rebased::iota::move_calls::utils;
use identity_iota_interaction::MoveType;
use crate::rebased::Error;
use identity_iota_interaction::rpc_types::OwnedObjectRef;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use identity_iota_interaction::types::transaction::ObjectArg;
use identity_iota_interaction::types::transaction::ProgrammableTransaction;
use move_core_types::ident_str;

pub(crate) fn approve<T: MoveType>(
  identity: OwnedObjectRef,
  controller_cap: ObjectRef,
  proposal_id: ObjectID,
  package: ObjectID,
) -> Result<ProgrammableTransaction, Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let identity = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
  let controller_cap = ptb
    .obj(ObjectArg::ImmOrOwnedObject(controller_cap))
    .map_err(|e| Error::InvalidArgument(e.to_string()))?;
  let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, controller_cap, package);
  let proposal_id = ptb
    .pure(proposal_id)
    .map_err(|e| Error::InvalidArgument(e.to_string()))?;

  ptb.programmable_move_call(
    package,
    ident_str!("identity").into(),
    ident_str!("approve_proposal").into(),
    vec![T::move_type(package)],
    vec![identity, delegation_token, proposal_id],
  );

  utils::put_back_delegation_token(&mut ptb, controller_cap, delegation_token, borrow, package);

  Ok(ptb.finish())
}
