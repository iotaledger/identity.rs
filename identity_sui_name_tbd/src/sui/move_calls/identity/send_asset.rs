use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::ObjectArg;
use iota_sdk::types::transaction::ProgrammableTransaction;
use iota_sdk::types::TypeTag;
use move_core_types::ident_str;

use crate::proposals::SendAction;
use crate::sui::move_calls;
use crate::utils::MoveType;

pub fn propose_send(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  transfer_map: Vec<(ObjectID, IotaAddress)>,
  expiration: Option<u64>,
  package_id: ObjectID,
) -> Result<ProgrammableTransaction, anyhow::Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let identity_arg = move_calls::utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
  let exp_arg = move_calls::utils::option_to_move(expiration, &mut ptb, package_id)?;
  let (objects, recipients) = {
    let (objects, recipients): (Vec<_>, Vec<_>) = transfer_map.into_iter().unzip();
    let objects = ptb.pure(objects)?;
    let recipients = ptb.pure(recipients)?;

    (objects, recipients)
  };

  let _proposal_id = ptb.programmable_move_call(
    package_id,
    ident_str!("identity").into(),
    ident_str!("propose_send").into(),
    vec![],
    vec![identity_arg, cap_arg, exp_arg, objects, recipients],
  );

  Ok(ptb.finish())
}

pub fn execute_send(
  identity: OwnedObjectRef,
  capability: ObjectRef,
  proposal_id: ObjectID,
  objects: Vec<(ObjectRef, TypeTag)>,
  package: ObjectID,
) -> Result<ProgrammableTransaction, anyhow::Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let identity = move_calls::utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
  let controller_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(capability))?;
  let proposal_id = ptb.pure(proposal_id)?;

  // Get the proposal's action as argument.
  let send_action = ptb.programmable_move_call(
    package,
    ident_str!("identity").into(),
    ident_str!("execute_proposal").into(),
    vec![SendAction::move_type(package)],
    vec![identity, controller_cap, proposal_id],
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

  Ok(ptb.finish())
}
