// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::IntoIterator;

use async_trait::async_trait;
use serde::Serialize;

use crate::rpc_types::IotaObjectData;
use crate::rpc_types::OwnedObjectRef;
use crate::types::base_types::IotaAddress;
use crate::types::base_types::ObjectID;
use crate::types::base_types::ObjectRef;
use crate::types::base_types::SequenceNumber;
use crate::types::transaction::Argument;
use crate::types::TypeTag;
use crate::MoveType;
use crate::OptionalSend;
use crate::ProgrammableTransactionBcs;

#[derive(Debug, Clone, Copy)]
pub enum ControllerTokenRef {
  Controller(ObjectRef),
  Delegate(ObjectRef),
}

impl ControllerTokenRef {
  pub fn is_controller_cap(&self) -> bool {
    matches!(self, ControllerTokenRef::Controller(_))
  }

  pub fn object_ref(&self) -> ObjectRef {
    match self {
      Self::Controller(obj_ref) => *obj_ref,
      Self::Delegate(obj_ref) => *obj_ref,
    }
  }
}

pub trait AssetMoveCalls {
  type Error;

  fn new_asset<T: Serialize + MoveType>(
    inner: &T,
    mutable: bool,
    transferable: bool,
    deletable: bool,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  fn delete<T: MoveType>(asset: ObjectRef, package: ObjectID) -> Result<ProgrammableTransactionBcs, Self::Error>;

  fn transfer<T: MoveType>(
    asset: ObjectRef,
    recipient: IotaAddress,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  fn make_tx(
    proposal: (ObjectID, SequenceNumber),
    cap: ObjectRef,
    asset: ObjectRef,
    asset_type_param: TypeTag,
    package: ObjectID,
    function_name: &'static str,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  fn accept_proposal(
    proposal: (ObjectID, SequenceNumber),
    recipient_cap: ObjectRef,
    asset: ObjectRef,
    asset_type_param: TypeTag,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  fn conclude_or_cancel(
    proposal: (ObjectID, SequenceNumber),
    sender_cap: ObjectRef,
    asset: ObjectRef,
    asset_type_param: TypeTag,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  fn update<T: MoveType + Serialize>(
    asset: ObjectRef,
    new_content: &T,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;
}

pub trait MigrationMoveCalls {
  type Error;

  fn migrate_did_output(
    did_output: ObjectRef,
    creation_timestamp: Option<u64>,
    migration_registry: OwnedObjectRef,
    package: ObjectID,
  ) -> anyhow::Result<ProgrammableTransactionBcs, Self::Error>;
}

pub trait BorrowIntentFnInternalT<B>: FnOnce(&mut B, &HashMap<ObjectID, (Argument, IotaObjectData)>) {}
impl<T, B> BorrowIntentFnInternalT<B> for T where T: FnOnce(&mut B, &HashMap<ObjectID, (Argument, IotaObjectData)>) {}

pub trait ControllerIntentFnInternalT<B>: FnOnce(&mut B, &Argument) {}
impl<T, B> ControllerIntentFnInternalT<B> for T where T: FnOnce(&mut B, &Argument) {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait IdentityMoveCalls {
  type Error;
  type NativeTxBuilder;

  fn propose_borrow(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
    objects: Vec<ObjectID>,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  fn execute_borrow<F: BorrowIntentFnInternalT<Self::NativeTxBuilder>>(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
    proposal_id: ObjectID,
    objects: Vec<IotaObjectData>,
    intent_fn: F,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  fn create_and_execute_borrow<F>(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
    objects: Vec<IotaObjectData>,
    intent_fn: F,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> anyhow::Result<ProgrammableTransactionBcs, Self::Error>
  where
    F: BorrowIntentFnInternalT<Self::NativeTxBuilder>;

  // We allow clippy::too_many_arguments here because splitting this trait function into multiple
  // other functions or creating an options struct gathering multiple function arguments has lower
  // priority at the moment.
  // TODO: remove clippy::too_many_arguments allowance here
  #[allow(clippy::too_many_arguments)]
  fn propose_config_change<I1, I2>(
    identity: OwnedObjectRef,
    controller_cap: ControllerTokenRef,
    expiration: Option<u64>,
    threshold: Option<u64>,
    controllers_to_add: I1,
    controllers_to_remove: HashSet<ObjectID>,
    controllers_to_update: I2,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>
  where
    I1: IntoIterator<Item = (IotaAddress, u64)>,
    I2: IntoIterator<Item = (ObjectID, u64)>;

  fn execute_config_change(
    identity: OwnedObjectRef,
    controller_cap: ControllerTokenRef,
    proposal_id: ObjectID,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  fn propose_controller_execution(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
    controller_cap_id: ObjectID,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  fn execute_controller_execution<F: ControllerIntentFnInternalT<Self::NativeTxBuilder>>(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
    proposal_id: ObjectID,
    borrowing_controller_cap_ref: ObjectRef,
    intent_fn: F,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  fn create_and_execute_controller_execution<F>(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
    expiration: Option<u64>,
    borrowing_controller_cap_ref: ObjectRef,
    intent_fn: F,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>
  where
    F: ControllerIntentFnInternalT<Self::NativeTxBuilder>;

  async fn new_identity(
    did_doc: Option<&[u8]>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  async fn new_with_controllers<C: IntoIterator<Item = (IotaAddress, u64, bool)> + OptionalSend>(
    did_doc: Option<&[u8]>,
    controllers: C,
    threshold: u64,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  fn approve_proposal<T: MoveType>(
    identity: OwnedObjectRef,
    controller_cap: ControllerTokenRef,
    proposal_id: ObjectID,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  fn propose_send(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
    transfer_map: Vec<(ObjectID, IotaAddress)>,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  fn execute_send(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
    proposal_id: ObjectID,
    objects: Vec<(ObjectRef, TypeTag)>,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  async fn propose_update(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
    did_doc: Option<&[u8]>,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  async fn execute_update(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
    proposal_id: ObjectID,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  fn create_and_execute_send(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
    transfer_map: Vec<(ObjectID, IotaAddress)>,
    expiration: Option<u64>,
    objects: Vec<(ObjectRef, TypeTag)>,
    package: ObjectID,
  ) -> anyhow::Result<ProgrammableTransactionBcs, Self::Error>;

  fn propose_upgrade(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
    expiration: Option<u64>,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  fn execute_upgrade(
    identity: OwnedObjectRef,
    capability: ControllerTokenRef,
    proposal_id: ObjectID,
    package_id: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  async fn delegate_controller_cap(
    controller_cap: ObjectRef,
    recipient: IotaAddress,
    permissions: u32,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  async fn revoke_delegation_token(
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    delegation_token_id: ObjectID,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  async fn unrevoke_delegation_token(
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    delegation_token_id: ObjectID,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;

  async fn destroy_delegation_token(
    identity: OwnedObjectRef,
    delegation_token: ObjectRef,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error>;
}
