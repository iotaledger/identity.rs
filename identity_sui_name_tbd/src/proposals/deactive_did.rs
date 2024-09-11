use std::str::FromStr;

use iota_sdk::rpc_types::IotaTransactionBlockEffects;
use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::Argument;
use iota_sdk::types::transaction::ProgrammableTransaction;
use iota_sdk::types::TypeTag;
use serde::Deserialize;
use serde::Serialize;

use crate::migration::OnChainIdentity;
use crate::migration::Proposal;
use crate::sui::move_calls;
use crate::utils::MoveType;
use crate::Error;

use super::ProposalT;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct DeactiveDid;

impl DeactiveDid {
  pub const fn new() -> Self {
    Self
  }
}

impl MoveType for DeactiveDid {
  fn move_type(package: ObjectID) -> TypeTag {
    TypeTag::from_str(&format!("{package}::identity::DeactivateDid")).expect("valid utf8")
  }
}

impl ProposalT for Proposal<DeactiveDid> {
  type Action = DeactiveDid;
  type Output = ();
  fn make_create_tx(
    _action: Self::Action,
    expiration: Option<u64>,
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    _identity_ref: OnChainIdentity,
    package: ObjectID,
  ) -> Result<(ProgrammableTransactionBuilder, Argument), Error> {
    move_calls::identity::propose_deactivation(identity, controller_cap, expiration, package)
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))
  }
  fn make_chained_execution_tx(
    ptb: ProgrammableTransactionBuilder,
    proposal_arg: Argument,
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    package: ObjectID,
  ) -> Result<ProgrammableTransaction, Error> {
    move_calls::identity::execute_deactivation(
      Some(ptb),
      Some(proposal_arg),
      identity,
      controller_cap,
      ObjectID::ZERO,
      package,
    )
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))
  }
  fn make_execute_tx(
    &self,
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    package: ObjectID,
  ) -> Result<ProgrammableTransaction, Error> {
    move_calls::identity::execute_deactivation(None, None, identity, controller_cap, self.id(), package)
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))
  }
  fn parse_tx_effects(_tx_effects: IotaTransactionBlockEffects) -> Result<Self::Output, Error> {
    Ok(())
  }
}
