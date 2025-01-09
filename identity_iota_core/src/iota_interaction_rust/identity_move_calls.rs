// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use std::str::FromStr;

use identity_iota_interaction::BorrowIntentFnT;
use identity_iota_interaction::ControllerIntentFnT;
use identity_iota_interaction::IdentityMoveCalls;
use identity_iota_interaction::ProgrammableTransactionBcs;
use identity_iota_interaction::TransactionBuilderT;

// ProgrammableTransactionBuilder can only be used here cause this is a platform specific file
use identity_iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder as Ptb;

use super::utils;
use super::TransactionBuilderAdapter;
use crate::rebased::proposals::BorrowAction;
use crate::rebased::proposals::ControllerExecution;
use crate::rebased::proposals::SendAction;
use crate::rebased::rebased_err;
use crate::rebased::Error;
use identity_iota_interaction::ident_str;
use identity_iota_interaction::rpc_types::IotaObjectData;
use identity_iota_interaction::rpc_types::OwnedObjectRef;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::base_types::ObjectType;
use identity_iota_interaction::types::transaction::ObjectArg;
use identity_iota_interaction::types::TypeTag;
use identity_iota_interaction::types::IOTA_FRAMEWORK_PACKAGE_ID;
use identity_iota_interaction::MoveType;

#[derive(Clone)]
pub(crate) struct IdentityMoveCallsRustSdk {}

impl IdentityMoveCalls for IdentityMoveCallsRustSdk {
  type Error = Error;
  type NativeTxBuilder = Ptb;

  fn propose_borrow(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    objects: Vec<ObjectID>,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let mut ptb = Ptb::new();
    let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability)).map_err(rebased_err)?;
    let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, cap_arg, package_id);
    let identity_arg = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true).map_err(rebased_err)?;
    let exp_arg = utils::option_to_move(expiration, &mut ptb, package_id).map_err(rebased_err)?;
    let objects_arg = ptb.pure(objects).map_err(rebased_err)?;

    let _proposal_id = ptb.programmable_move_call(
      package_id,
      ident_str!("identity").into(),
      ident_str!("propose_borrow").into(),
      vec![],
      vec![identity_arg, delegation_token, exp_arg, objects_arg],
    );

    utils::put_back_delegation_token(&mut ptb, cap_arg, delegation_token, borrow, package_id);

    Ok(bcs::to_bytes(&ptb.finish())?)
  }

  fn execute_borrow<F: BorrowIntentFnT<Self::Error, Self::NativeTxBuilder>>(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    objects: Vec<IotaObjectData>,
    intent_fn: F,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let mut ptb = Ptb::new();
    let identity = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true).map_err(rebased_err)?;
    let controller_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(capability)).map_err(rebased_err)?;
    let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, controller_cap, package);
    let proposal_id = ptb.pure(proposal_id).map_err(rebased_err)?;

    // Get the proposal's action as argument.
    let borrow_action = ptb.programmable_move_call(
      package,
      ident_str!("identity").into(),
      ident_str!("execute_proposal").into(),
      vec![BorrowAction::move_type(package)],
      vec![identity, delegation_token, proposal_id],
    );

    utils::put_back_delegation_token(&mut ptb, controller_cap, delegation_token, borrow, package);

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
      .collect::<anyhow::Result<_>>()
      .map_err(rebased_err)?;

    // Apply the user-defined operation.
    let mut ptb_adapter = TransactionBuilderAdapter::new(ptb);
    intent_fn(&mut ptb_adapter, &obj_arg_map);

    // Put back all the objects.
    let mut ptb = ptb_adapter.into_native_tx_builder();
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

    Ok(bcs::to_bytes(&ptb.finish())?)
  }

  fn propose_config_change<I1, I2>(
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    expiration: Option<u64>,
    threshold: Option<u64>,
    controllers_to_add: I1,
    controllers_to_remove: HashSet<ObjectID>,
    controllers_to_update: I2,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>
  where
    I1: IntoIterator<Item = (IotaAddress, u64)>,
    I2: IntoIterator<Item = (ObjectID, u64)>,
  {
    let mut ptb = Ptb::new();

    let controllers_to_add = {
      let (addresses, vps): (Vec<IotaAddress>, Vec<u64>) = controllers_to_add.into_iter().unzip();
      let addresses = ptb.pure(addresses).map_err(rebased_err)?;
      let vps = ptb.pure(vps).map_err(rebased_err)?;

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
      let ids = ptb.pure(ids).map_err(rebased_err)?;
      let vps = ptb.pure(vps).map_err(rebased_err)?;

      ptb.programmable_move_call(
        package,
        ident_str!("utils").into(),
        ident_str!("vec_map_from_keys_values").into(),
        vec![TypeTag::from_str("0x2::object::ID").expect("valid utf8"), TypeTag::U64],
        vec![ids, vps],
      )
    };
    let identity = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true).map_err(rebased_err)?;
    let controller_cap = ptb
      .obj(ObjectArg::ImmOrOwnedObject(controller_cap))
      .map_err(rebased_err)?;
    let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, controller_cap, package);
    let expiration = utils::option_to_move(expiration, &mut ptb, package).map_err(rebased_err)?;
    let threshold = utils::option_to_move(threshold, &mut ptb, package).map_err(rebased_err)?;
    let controllers_to_remove = ptb.pure(controllers_to_remove).map_err(rebased_err)?;

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

    Ok(bcs::to_bytes(&ptb.finish())?)
  }

  fn execute_config_change(
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    proposal_id: ObjectID,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let mut ptb = Ptb::new();

    let identity = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true).map_err(rebased_err)?;
    let controller_cap = ptb
      .obj(ObjectArg::ImmOrOwnedObject(controller_cap))
      .map_err(rebased_err)?;
    let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, controller_cap, package);
    let proposal_id = ptb.pure(proposal_id).map_err(rebased_err)?;
    ptb.programmable_move_call(
      package,
      ident_str!("identity").into(),
      ident_str!("execute_config_change").into(),
      vec![],
      vec![identity, delegation_token, proposal_id],
    );

    utils::put_back_delegation_token(&mut ptb, controller_cap, delegation_token, borrow, package);

    Ok(bcs::to_bytes(&ptb.finish())?)
  }

  fn propose_controller_execution(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    controller_cap_id: ObjectID,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let mut ptb = Ptb::new();
    let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability)).map_err(rebased_err)?;
    let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, cap_arg, package_id);
    let identity_arg = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true).map_err(rebased_err)?;
    let controller_cap_id = ptb.pure(controller_cap_id).map_err(rebased_err)?;
    let exp_arg = utils::option_to_move(expiration, &mut ptb, package_id).map_err(rebased_err)?;

    let _proposal_id = ptb.programmable_move_call(
      package_id,
      ident_str!("identity").into(),
      ident_str!("propose_controller_execution").into(),
      vec![],
      vec![identity_arg, delegation_token, controller_cap_id, exp_arg],
    );

    utils::put_back_delegation_token(&mut ptb, cap_arg, delegation_token, borrow, package_id);

    Ok(bcs::to_bytes(&ptb.finish())?)
  }

  fn execute_controller_execution<F: ControllerIntentFnT<Self::Error, Self::NativeTxBuilder>>(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    borrowing_controller_cap_ref: ObjectRef,
    intent_fn: F,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let mut ptb = Ptb::new();
    let identity = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true).map_err(rebased_err)?;
    let controller_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(capability)).map_err(rebased_err)?;
    let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, controller_cap, package);
    let proposal_id = ptb.pure(proposal_id).map_err(rebased_err)?;

    // Get the proposal's action as argument.
    let controller_execution_action = ptb.programmable_move_call(
      package,
      ident_str!("identity").into(),
      ident_str!("execute_proposal").into(),
      vec![ControllerExecution::move_type(package)],
      vec![identity, delegation_token, proposal_id],
    );

    utils::put_back_delegation_token(&mut ptb, controller_cap, delegation_token, borrow, package);

    // Borrow the controller cap into this transaction.
    let receiving = ptb
      .obj(ObjectArg::Receiving(borrowing_controller_cap_ref))
      .map_err(rebased_err)?;
    let borrowed_controller_cap = ptb.programmable_move_call(
      package,
      ident_str!("identity").into(),
      ident_str!("borrow_controller_cap").into(),
      vec![],
      vec![identity, controller_execution_action, receiving],
    );

    // Apply the user-defined operation.
    let mut ptb_adapter = TransactionBuilderAdapter::new(ptb);
    intent_fn(&mut ptb_adapter, &borrowed_controller_cap);

    // Put back the borrowed controller cap.
    let mut ptb = ptb_adapter.into_native_tx_builder();
    ptb.programmable_move_call(
      package,
      ident_str!("controller_proposal").into(),
      ident_str!("put_back").into(),
      vec![],
      vec![controller_execution_action, borrowed_controller_cap],
    );

    Ok(bcs::to_bytes(&ptb.finish())?)
  }

  fn new_identity(did_doc: &[u8], package_id: ObjectID) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let mut ptb = Ptb::new();
    let doc_arg = utils::ptb_pure(&mut ptb, "did_doc", did_doc)?;
    let clock = utils::get_clock_ref(&mut ptb);

    // Create a new identity, sending its capability to the tx's sender.
    let _identity_id = ptb.programmable_move_call(
      package_id,
      ident_str!("identity").into(),
      ident_str!("new").into(),
      vec![],
      vec![doc_arg, clock],
    );

    Ok(bcs::to_bytes(&ptb.finish())?)
  }

  fn new_with_controllers<C>(
    did_doc: &[u8],
    controllers: C,
    threshold: u64,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>
  where
    C: IntoIterator<Item = (IotaAddress, u64)>,
  {
    let mut ptb = Ptb::new();

    let controllers = {
      let (ids, vps): (Vec<IotaAddress>, Vec<u64>) = controllers.into_iter().unzip();
      let ids = ptb.pure(ids).map_err(|e| Error::InvalidArgument(e.to_string()))?;
      let vps = ptb.pure(vps).map_err(|e| Error::InvalidArgument(e.to_string()))?;
      ptb.programmable_move_call(
        package_id,
        ident_str!("utils").into(),
        ident_str!("vec_map_from_keys_values").into(),
        vec![TypeTag::Address, TypeTag::U64],
        vec![ids, vps],
      )
    };

    let controllers_that_can_delegate = ptb.programmable_move_call(
      IOTA_FRAMEWORK_PACKAGE_ID,
      ident_str!("vec_map").into(),
      ident_str!("empty").into(),
      vec![TypeTag::Address, TypeTag::U64],
      vec![],
    );
    let doc_arg = ptb.pure(did_doc).map_err(|e| Error::InvalidArgument(e.to_string()))?;
    let threshold_arg = ptb.pure(threshold).map_err(|e| Error::InvalidArgument(e.to_string()))?;
    let clock = utils::get_clock_ref(&mut ptb);

    // Create a new identity, sending its capabilities to the specified controllers.
    let _identity_id = ptb.programmable_move_call(
      package_id,
      ident_str!("identity").into(),
      ident_str!("new_with_controllers").into(),
      vec![],
      vec![
        doc_arg,
        controllers,
        controllers_that_can_delegate,
        threshold_arg,
        clock,
      ],
    );

    Ok(bcs::to_bytes(&ptb.finish())?)
  }

  fn propose_deactivation(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let mut ptb = Ptb::new();
    let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability)).map_err(rebased_err)?;
    let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, cap_arg, package_id);
    let identity_arg = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true).map_err(rebased_err)?;
    let exp_arg = utils::option_to_move(expiration, &mut ptb, package_id).map_err(rebased_err)?;
    let clock = utils::get_clock_ref(&mut ptb);

    let _proposal_id = ptb.programmable_move_call(
      package_id,
      ident_str!("identity").into(),
      ident_str!("propose_deactivation").into(),
      vec![],
      vec![identity_arg, delegation_token, exp_arg, clock],
    );

    utils::put_back_delegation_token(&mut ptb, cap_arg, delegation_token, borrow, package_id);

    Ok(bcs::to_bytes(&ptb.finish())?)
  }

  fn execute_deactivation(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let mut ptb = Ptb::new();
    let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability)).map_err(rebased_err)?;
    let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, cap_arg, package_id);
    let proposal_id = ptb.pure(proposal_id).map_err(rebased_err)?;
    let identity_arg = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true).map_err(rebased_err)?;
    let clock = utils::get_clock_ref(&mut ptb);

    let _ = ptb.programmable_move_call(
      package_id,
      ident_str!("identity").into(),
      ident_str!("execute_deactivation").into(),
      vec![],
      vec![identity_arg, delegation_token, proposal_id, clock],
    );

    utils::put_back_delegation_token(&mut ptb, cap_arg, delegation_token, borrow, package_id);

    Ok(bcs::to_bytes(&ptb.finish())?)
  }

  fn approve_proposal<T: MoveType>(
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    proposal_id: ObjectID,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let mut ptb = Ptb::new();
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

    Ok(bcs::to_bytes(&ptb.finish())?)
  }

  fn propose_send(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    transfer_map: Vec<(ObjectID, IotaAddress)>,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let mut ptb = Ptb::new();
    let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability)).map_err(rebased_err)?;
    let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, cap_arg, package_id);
    let identity_arg = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true).map_err(rebased_err)?;
    let exp_arg = utils::option_to_move(expiration, &mut ptb, package_id).map_err(rebased_err)?;
    let (objects, recipients) = {
      let (objects, recipients): (Vec<_>, Vec<_>) = transfer_map.into_iter().unzip();
      let objects = ptb.pure(objects).map_err(rebased_err)?;
      let recipients = ptb.pure(recipients).map_err(rebased_err)?;

      (objects, recipients)
    };

    let _proposal_id = ptb.programmable_move_call(
      package_id,
      ident_str!("identity").into(),
      ident_str!("propose_send").into(),
      vec![],
      vec![identity_arg, delegation_token, exp_arg, objects, recipients],
    );

    utils::put_back_delegation_token(&mut ptb, cap_arg, delegation_token, borrow, package_id);

    Ok(bcs::to_bytes(&ptb.finish())?)
  }

  fn execute_send(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    objects: Vec<(ObjectRef, TypeTag)>,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let mut ptb = Ptb::new();
    let identity = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true).map_err(rebased_err)?;
    let controller_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(capability)).map_err(rebased_err)?;
    let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, controller_cap, package);
    let proposal_id = ptb.pure(proposal_id).map_err(rebased_err)?;

    // Get the proposal's action as argument.
    let send_action = ptb.programmable_move_call(
      package,
      ident_str!("identity").into(),
      ident_str!("execute_proposal").into(),
      vec![SendAction::move_type(package)],
      vec![identity, delegation_token, proposal_id],
    );

    utils::put_back_delegation_token(&mut ptb, controller_cap, delegation_token, borrow, package);

    // Send each object in this send action.
    // Traversing the map in reverse reduces the number of operations on the move side.
    for (obj, obj_type) in objects.into_iter().rev() {
      let recv_obj = ptb.obj(ObjectArg::Receiving(obj)).map_err(rebased_err)?;

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

    Ok(bcs::to_bytes(&ptb.finish())?)
  }

  fn propose_update(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    did_doc: impl AsRef<[u8]>,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let mut ptb = Ptb::new();
    let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability)).map_err(rebased_err)?;
    let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, cap_arg, package_id);
    let identity_arg = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true).map_err(rebased_err)?;
    let exp_arg = utils::option_to_move(expiration, &mut ptb, package_id).map_err(rebased_err)?;
    let doc_arg = ptb.pure(did_doc.as_ref()).map_err(rebased_err)?;
    let clock = utils::get_clock_ref(&mut ptb);

    let _proposal_id = ptb.programmable_move_call(
      package_id,
      ident_str!("identity").into(),
      ident_str!("propose_update").into(),
      vec![],
      vec![identity_arg, delegation_token, doc_arg, exp_arg, clock],
    );

    utils::put_back_delegation_token(&mut ptb, cap_arg, delegation_token, borrow, package_id);

    Ok(bcs::to_bytes(&ptb.finish())?)
  }

  fn execute_update(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let mut ptb = Ptb::new();
    let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability)).map_err(rebased_err)?;
    let (delegation_token, borrow) = utils::get_controller_delegation(&mut ptb, cap_arg, package_id);
    let proposal_id = ptb.pure(proposal_id).map_err(rebased_err)?;
    let identity_arg = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true).map_err(rebased_err)?;
    let clock = utils::get_clock_ref(&mut ptb);

    let _ = ptb.programmable_move_call(
      package_id,
      ident_str!("identity").into(),
      ident_str!("execute_update").into(),
      vec![],
      vec![identity_arg, delegation_token, proposal_id, clock],
    );

    utils::put_back_delegation_token(&mut ptb, cap_arg, delegation_token, borrow, package_id);

    Ok(bcs::to_bytes(&ptb.finish())?)
  }

  fn propose_upgrade(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let mut ptb = Ptb::new();
    let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability)).map_err(rebased_err)?;
    let identity_arg = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true).map_err(rebased_err)?;
    let exp_arg = utils::option_to_move(expiration, &mut ptb, package_id).map_err(rebased_err)?;

    let _proposal_id = ptb.programmable_move_call(
      package_id,
      ident_str!("identity").into(),
      ident_str!("propose_upgrade").into(),
      vec![],
      vec![identity_arg, cap_arg, exp_arg],
    );

    Ok(bcs::to_bytes(&ptb.finish())?)
  }

  fn execute_upgrade(
    identity: OwnedObjectRef,
    capability: ObjectRef,
    proposal_id: ObjectID,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    let mut ptb = Ptb::new();
    let cap_arg = ptb.obj(ObjectArg::ImmOrOwnedObject(capability)).map_err(rebased_err)?;
    let proposal_id = ptb.pure(proposal_id).map_err(rebased_err)?;
    let identity_arg = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true).map_err(rebased_err)?;

    let _ = ptb.programmable_move_call(
      package_id,
      ident_str!("identity").into(),
      ident_str!("execute_upgrade").into(),
      vec![],
      vec![identity_arg, cap_arg, proposal_id],
    );

    Ok(bcs::to_bytes(&ptb.finish())?)
  }
}
