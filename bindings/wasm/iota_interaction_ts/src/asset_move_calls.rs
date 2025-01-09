// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use crate::error::TsSdkError;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::base_types::SequenceNumber;
use identity_iota_interaction::types::TypeTag;
use identity_iota_interaction::AssetMoveCalls;
use identity_iota_interaction::MoveType;
use identity_iota_interaction::ProgrammableTransactionBcs;

pub struct AssetMoveCallsTsSdk {}

impl AssetMoveCalls for AssetMoveCallsTsSdk {
  type Error = TsSdkError;

  fn new_asset<T: Serialize + MoveType>(
    inner: T,
    mutable: bool,
    transferable: bool,
    deletable: bool,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }

  fn delete<T: MoveType>(asset: ObjectRef, package: ObjectID) -> Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }

  fn transfer<T: MoveType>(
    asset: ObjectRef,
    recipient: IotaAddress,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }

  fn make_tx(
    proposal: (ObjectID, SequenceNumber),
    cap: ObjectRef,
    asset: ObjectRef,
    asset_type_param: TypeTag,
    package: ObjectID,
    function_name: &'static str,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }

  fn accept_proposal(
    proposal: (ObjectID, SequenceNumber),
    recipient_cap: ObjectRef,
    asset: ObjectRef,
    asset_type_param: TypeTag,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }

  fn conclude_or_cancel(
    proposal: (ObjectID, SequenceNumber),
    sender_cap: ObjectRef,
    asset: ObjectRef,
    asset_type_param: TypeTag,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }

  fn update<T: MoveType + Serialize>(
    asset: ObjectRef,
    new_content: T,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Self::Error> {
    unimplemented!();
  }
}
