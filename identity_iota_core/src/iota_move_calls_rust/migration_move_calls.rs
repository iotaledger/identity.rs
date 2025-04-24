// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder as Ptb;

use identity_iota_interaction::ident_str;
use identity_iota_interaction::rpc_types::OwnedObjectRef;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::transaction::ObjectArg;
use identity_iota_interaction::types::IOTA_FRAMEWORK_PACKAGE_ID;
use identity_iota_interaction::MigrationMoveCalls;
use identity_iota_interaction::ProgrammableTransactionBcs;

use crate::rebased::Error;

use super::utils;

pub(crate) struct MigrationMoveCallsRustSdk {}

impl MigrationMoveCalls for MigrationMoveCallsRustSdk {
  type Error = Error;

  fn migrate_did_output(
    did_output: ObjectRef,
    creation_timestamp: Option<u64>,
    migration_registry: OwnedObjectRef,
    package: ObjectID,
  ) -> anyhow::Result<ProgrammableTransactionBcs, Self::Error> {
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

    Ok(bcs::to_bytes(&ptb.finish())?)
  }
}
