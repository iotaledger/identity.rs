use std::collections::HashSet;
use std::str::FromStr;

use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::object::Owner;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::Argument;
use iota_sdk::types::transaction::ObjectArg;
use iota_sdk::types::transaction::ProgrammableTransaction;
use iota_sdk::types::Identifier;
use iota_sdk::types::TypeTag;

use super::super::utils;

#[allow(clippy::too_many_arguments)]
pub fn propose_config_change<I1, I2>(
  identity: OwnedObjectRef,
  controller_cap: ObjectRef,
  expiration: Option<u64>,
  threshold: Option<u64>,
  controllers_to_add: I1,
  controllers_to_remove: HashSet<ObjectID>,
  controllers_to_update: I2,
  package: ObjectID,
) -> anyhow::Result<(ProgrammableTransactionBuilder, Argument)>
where
  I1: IntoIterator<Item = (IotaAddress, u64)>,
  I2: IntoIterator<Item = (ObjectID, u64)>,
{
  let mut ptb = ProgrammableTransactionBuilder::new();

  let controllers_to_add = {
    let (addresses, vps): (Vec<IotaAddress>, Vec<u64>) = controllers_to_add.into_iter().unzip();
    let addresses = ptb.pure(addresses)?;
    let vps = ptb.pure(vps)?;

    ptb.programmable_move_call(
      package,
      Identifier::new("utils")?,
      Identifier::new("vec_map_from_keys_values")?,
      vec![TypeTag::Address, TypeTag::U64],
      vec![addresses, vps],
    )
  };
  let controllers_to_update = {
    let (ids, vps): (Vec<ObjectID>, Vec<u64>) = controllers_to_update.into_iter().unzip();
    let ids = ptb.pure(ids)?;
    let vps = ptb.pure(vps)?;

    ptb.programmable_move_call(
      package,
      Identifier::new("utils")?,
      Identifier::new("vec_map_from_keys_values")?,
      vec![TypeTag::from_str("0x2::object::ID").expect("valid utf8"), TypeTag::U64],
      vec![ids, vps],
    )
  };
  let identity = utils::owned_ref_to_shared_object_arg(identity, &mut ptb, true)?;
  let controller_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(controller_cap))?;
  let expiration = utils::option_to_move(expiration, &mut ptb, package)?;
  let threshold = utils::option_to_move(threshold, &mut ptb, package)?;
  let controllers_to_remove = ptb.pure(controllers_to_remove)?;

  let proposal_id = ptb.programmable_move_call(
    package,
    Identifier::new("identity").expect("valid utf8"),
    Identifier::new("propose_config_change").expect("valid utf8"),
    vec![],
    vec![
      identity,
      controller_cap,
      expiration,
      threshold,
      controllers_to_add,
      controllers_to_remove,
      controllers_to_update,
    ],
  );

  Ok((ptb, proposal_id))
}

pub fn execute_config_change(
  ptb: Option<ProgrammableTransactionBuilder>,
  proposal_id_arg: Option<Argument>,
  identity: OwnedObjectRef,
  controller_cap: ObjectRef,
  proposal_id: ObjectID,
  package: ObjectID,
) -> anyhow::Result<ProgrammableTransaction> {
  let mut ptb = ptb.unwrap_or_default();

  let Owner::Shared { initial_shared_version } = identity.owner else {
    anyhow::bail!("identity \"{}\" is a not shared object", identity.reference.object_id);
  };
  let identity = ptb.obj(ObjectArg::SharedObject {
    id: identity.reference.object_id,
    initial_shared_version,
    mutable: true,
  })?;
  let controller_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(controller_cap))?;
  let proposal_id = if let Some(proposal_id) = proposal_id_arg {
    proposal_id
  } else {
    ptb.pure(proposal_id)?
  };
  ptb.programmable_move_call(
    package,
    Identifier::new("identity").expect("valid utf8"),
    Identifier::new("execute_config_change").expect("valid_utf8"),
    vec![],
    vec![identity, controller_cap, proposal_id],
  );

  Ok(ptb.finish())
}
