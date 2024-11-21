// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::utils;
use iota_sdk::{
  rpc_types::OwnedObjectRef,
  types::{
    base_types::{ObjectID, ObjectRef},
    programmable_transaction_builder::ProgrammableTransactionBuilder as Ptb,
    transaction::{ObjectArg, ProgrammableTransaction},
    IOTA_FRAMEWORK_PACKAGE_ID,
  },
};
use move_core_types::ident_str;

pub(crate) fn migrate_did_output(
  did_output: ObjectRef,
  creation_timestamp: Option<u64>,
  migration_registry: OwnedObjectRef,
  package: ObjectID,
) -> anyhow::Result<ProgrammableTransaction> {
  let mut ptb = Ptb::new();
  let did_output = ptb.obj(ObjectArg::ImmOrOwnedObject(did_output))?;
  let migration_registry = utils::owned_ref_to_shared_object_arg(migration_registry, &mut ptb, true)?;
  let clock = utils::get_clock_ref(&mut ptb);

  let creation_timestamp = match creation_timestamp {
    Some(timestamp) => ptb.pure(timestamp)?,
    _ => ptb.programmable_move_call(
      IOTA_FRAMEWORK_PACKAGE_ID,
      ident_str!("clock").into(),
      ident_str!("timestamp_ms").into(),
      vec![],
      vec![clock],
    ),
  };

  ptb.programmable_move_call(
    package,
    ident_str!("migration").into(),
    ident_str!("migrate_alias_output").into(),
    vec![],
    vec![did_output, migration_registry, creation_timestamp, clock],
  );

  Ok(ptb.finish())
}
