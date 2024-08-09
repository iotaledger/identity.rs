use crate::utils::MoveType;
use crate::Error;
use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::object::Owner;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::ObjectArg;
use iota_sdk::types::transaction::ProgrammableTransaction;
use iota_sdk::types::Identifier;

pub fn approve<T: MoveType>(
  identity: OwnedObjectRef,
  controller_cap: ObjectRef,
  proposal_id: ObjectID,
  package: ObjectID,
) -> Result<ProgrammableTransaction, Error> {
  let mut ptb = ProgrammableTransactionBuilder::new();
  let Owner::Shared { initial_shared_version } = identity.owner else {
    return Err(Error::TransactionBuildingFailed(format!(
      "Identity \"{}\" is not a shared object",
      identity.object_id()
    )));
  };
  let identity = ptb
    .obj(ObjectArg::SharedObject {
      id: identity.object_id(),
      initial_shared_version,
      mutable: true,
    })
    .map_err(|e| Error::InvalidArgument(e.to_string()))?;
  let controller_cap = ptb
    .obj(ObjectArg::ImmOrOwnedObject(controller_cap))
    .map_err(|e| Error::InvalidArgument(e.to_string()))?;
  let proposal_id = ptb
    .pure(proposal_id)
    .map_err(|e| Error::InvalidArgument(e.to_string()))?;

  ptb.programmable_move_call(
    package,
    Identifier::new("identity").expect("valid utf8"),
    Identifier::new("approve_proposal").expect("valid utf8"),
    vec![T::move_type(package)],
    vec![identity, controller_cap, proposal_id],
  );

  Ok(ptb.finish())
}
