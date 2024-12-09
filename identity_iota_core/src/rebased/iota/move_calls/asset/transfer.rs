// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::base_types::SequenceNumber;
use identity_iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use identity_iota_interaction::types::transaction::Command;
use identity_iota_interaction::types::transaction::ObjectArg;
use identity_iota_interaction::types::transaction::ProgrammableTransaction;
use identity_iota_interaction::types::TypeTag;
use move_core_types::ident_str;

use identity_iota_interaction::MoveType;
use crate::rebased::Error;

pub(crate) fn transfer<T: MoveType>(
  asset: ObjectRef,
  recipient: IotaAddress,
  package: ObjectID,
) -> Result<ProgrammableTransaction, Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let asset = ptb
    .obj(ObjectArg::ImmOrOwnedObject(asset))
    .map_err(|e| Error::InvalidArgument(e.to_string()))?;
  let recipient = ptb.pure(recipient).map_err(|e| Error::InvalidArgument(e.to_string()))?;

  ptb.command(Command::move_call(
    package,
    ident_str!("asset").into(),
    ident_str!("transfer").into(),
    vec![T::move_type(package)],
    vec![asset, recipient],
  ));

  Ok(ptb.finish())
}

fn make_tx(
  proposal: (ObjectID, SequenceNumber),
  cap: ObjectRef,
  asset: ObjectRef,
  asset_type_param: TypeTag,
  package: ObjectID,
  function_name: &'static str,
) -> Result<ProgrammableTransaction, Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let proposal = ptb
    .obj(ObjectArg::SharedObject {
      id: proposal.0,
      initial_shared_version: proposal.1,
      mutable: true,
    })
    .map_err(|e| Error::InvalidArgument(e.to_string()))?;
  let cap = ptb
    .obj(ObjectArg::ImmOrOwnedObject(cap))
    .map_err(|e| Error::InvalidArgument(e.to_string()))?;
  let asset = ptb
    .obj(ObjectArg::Receiving(asset))
    .map_err(|e| Error::InvalidArgument(e.to_string()))?;

  ptb.command(Command::move_call(
    package,
    ident_str!("asset").into(),
    ident_str!(function_name).into(),
    vec![asset_type_param],
    vec![proposal, cap, asset],
  ));

  Ok(ptb.finish())
}

pub(crate) fn accept_proposal(
  proposal: (ObjectID, SequenceNumber),
  recipient_cap: ObjectRef,
  asset: ObjectRef,
  asset_type_param: TypeTag,
  package: ObjectID,
) -> Result<ProgrammableTransaction, Error> {
  make_tx(proposal, recipient_cap, asset, asset_type_param, package, "accept")
}

pub(crate) fn conclude_or_cancel(
  proposal: (ObjectID, SequenceNumber),
  sender_cap: ObjectRef,
  asset: ObjectRef,
  asset_type_param: TypeTag,
  package: ObjectID,
) -> Result<ProgrammableTransaction, Error> {
  make_tx(
    proposal,
    sender_cap,
    asset,
    asset_type_param,
    package,
    "conclude_or_cancel",
  )
}
