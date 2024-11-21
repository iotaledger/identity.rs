use crate::rebased::utils::MoveType;
use crate::rebased::Error;
use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::STD_OPTION_MODULE_NAME;
use iota_sdk::types::object::Owner;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder as Ptb;
use iota_sdk::types::transaction::Argument;
use iota_sdk::types::transaction::ObjectArg;
use iota_sdk::types::IOTA_CLOCK_OBJECT_ID;
use iota_sdk::types::IOTA_CLOCK_OBJECT_SHARED_VERSION;
use iota_sdk::types::MOVE_STDLIB_PACKAGE_ID;
use move_core_types::ident_str;
use serde::Serialize;

/// Adds a reference to the on-chain clock to `ptb`'s arguments.
pub(crate) fn get_clock_ref(ptb: &mut Ptb) -> Argument {
  ptb
    .obj(ObjectArg::SharedObject {
      id: IOTA_CLOCK_OBJECT_ID,
      initial_shared_version: IOTA_CLOCK_OBJECT_SHARED_VERSION,
      mutable: false,
    })
    .expect("network has a singleton clock instantiated")
}

pub(crate) fn owned_ref_to_shared_object_arg(
  owned_ref: OwnedObjectRef,
  ptb: &mut Ptb,
  mutable: bool,
) -> anyhow::Result<Argument> {
  let Owner::Shared { initial_shared_version } = owned_ref.owner else {
    anyhow::bail!("Identity \"{}\" is not a shared object", owned_ref.object_id());
  };
  ptb.obj(ObjectArg::SharedObject {
    id: owned_ref.object_id(),
    initial_shared_version,
    mutable,
  })
}

pub(crate) fn option_to_move<T: MoveType + Serialize>(
  option: Option<T>,
  ptb: &mut Ptb,
  package: ObjectID,
) -> Result<Argument, anyhow::Error> {
  let arg = if let Some(t) = option {
    let t = ptb.pure(t)?;
    ptb.programmable_move_call(
      MOVE_STDLIB_PACKAGE_ID,
      STD_OPTION_MODULE_NAME.into(),
      ident_str!("some").into(),
      vec![T::move_type(package)],
      vec![t],
    )
  } else {
    ptb.programmable_move_call(
      MOVE_STDLIB_PACKAGE_ID,
      STD_OPTION_MODULE_NAME.into(),
      ident_str!("none").into(),
      vec![T::move_type(package)],
      vec![],
    )
  };

  Ok(arg)
}

pub(crate) fn ptb_pure<T>(ptb: &mut Ptb, name: &str, value: T) -> Result<Argument, Error>
where
  T: Serialize + core::fmt::Debug,
{
  ptb.pure(&value).map_err(|err| {
    Error::InvalidArgument(format!(
      r"could not serialize pure value {name} with value {value:?}; {err}"
    ))
  })
}

#[allow(dead_code)]
pub(crate) fn ptb_obj(ptb: &mut Ptb, name: &str, value: ObjectArg) -> Result<Argument, Error> {
  ptb
    .obj(value)
    .map_err(|err| Error::InvalidArgument(format!("could not serialize object {name} {value:?}; {err}")))
}
