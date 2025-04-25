// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_interaction::types::base_types::IotaAddress;
use iota_interaction::types::base_types::ObjectID;
use iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_interaction::types::transaction::ProgrammableTransaction;
use iota_interaction::types::TypeTag;
use iota_interaction::types::IOTA_FRAMEWORK_PACKAGE_ID;
use iota_interaction::ident_str;

use crate::rebased::iota::move_calls::utils;
use crate::rebased::Error;

/// Build a transaction that creates a new on-chain Identity containing `did_doc`.
pub(crate) fn new(did_doc: &[u8], package_id: ObjectID) -> Result<ProgrammableTransaction, Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
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

  Ok(ptb.finish())
}

pub(crate) fn new_with_controllers<C>(
  did_doc: &[u8],
  controllers: C,
  threshold: u64,
  package_id: ObjectID,
) -> Result<ProgrammableTransaction, Error>
where
  C: IntoIterator<Item = (IotaAddress, u64)>,
{
  let mut ptb = ProgrammableTransactionBuilder::new();

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

  Ok(ptb.finish())
}
