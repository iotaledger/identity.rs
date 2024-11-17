use super::utils;
use iota_sdk::{
  rpc_types::OwnedObjectRef,
  types::{
    base_types::{ObjectID, ObjectRef},
    programmable_transaction_builder::ProgrammableTransactionBuilder as Ptb,
    transaction::{ObjectArg, ProgrammableTransaction},
  },
};
use move_core_types::ident_str;

pub(crate) fn migrate_did_output(
  did_output: ObjectRef,
  migration_registry: OwnedObjectRef,
  package: ObjectID,
) -> anyhow::Result<ProgrammableTransaction> {
  let mut ptb = Ptb::new();
  let did_output = ptb.obj(ObjectArg::ImmOrOwnedObject(did_output))?;
  let migration_registry = utils::owned_ref_to_shared_object_arg(migration_registry, &mut ptb, true)?;

  ptb.programmable_move_call(
    package,
    ident_str!("migration").into(),
    ident_str!("migrate_alias_output").into(),
    vec![],
    vec![did_output, migration_registry],
  );

  Ok(ptb.finish())
}
