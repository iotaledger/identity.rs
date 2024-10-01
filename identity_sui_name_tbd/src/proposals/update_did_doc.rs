use std::str::FromStr;

use identity_iota_core::IotaDocument;
use crate::iota_sdk_abstraction::IotaTransactionBlockResponseT;
use crate::iota_sdk_abstraction::rpc_types::OwnedObjectRef;
use crate::iota_sdk_abstraction::types::base_types::ObjectID;
use crate::iota_sdk_abstraction::types::base_types::ObjectRef;
use crate::iota_sdk_abstraction::types::transaction::Argument;
use crate::iota_sdk_abstraction::ProgrammableTransactionBcs;
use crate::iota_sdk_abstraction::types::TypeTag;
use serde::Deserialize;
use serde::Serialize;

use crate::migration::OnChainIdentity;
use crate::migration::Proposal;
use crate::sui::iota_sdk_adapter::IdentityMoveCallsAdapter;
use crate::iota_sdk_abstraction::IdentityMoveCalls;
use crate::utils::MoveType;
use crate::Error;

use super::ProposalT;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(into = "UpdateValue::<Vec<u8>>", from = "UpdateValue::<Vec<u8>>")]
pub struct UpdateDidDocument(Vec<u8>);

impl MoveType for UpdateDidDocument {
  fn move_type(package: ObjectID) -> TypeTag {
    TypeTag::from_str(&format!("{package}::update_value_proposal::UpdateValue<vector<u8>>")).expect("valid TypeTag")
  }
}

impl UpdateDidDocument {
  pub fn new(document: IotaDocument) -> Self {
    Self(document.pack().expect("a valid IotaDocument is packable"))
  }
}

impl ProposalT for Proposal<UpdateDidDocument> {
  type Action = UpdateDidDocument;
  type Output = ();

  fn make_create_tx(
    action: Self::Action,
    expiration: Option<u64>,
    identity_ref: OwnedObjectRef,
    controller_cap: ObjectRef,
    _identity: OnChainIdentity,
    package: ObjectID,
  ) -> Result<(<IdentityMoveCallsAdapter as IdentityMoveCalls>::TxBuilder, Argument), Error> {
    IdentityMoveCallsAdapter::propose_update(identity_ref, controller_cap, &action.0, expiration, package)
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))
  }

  fn make_chained_execution_tx(
    ptb: <IdentityMoveCallsAdapter as IdentityMoveCalls>::TxBuilder,
    proposal_arg: Argument,
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Error> {
    IdentityMoveCallsAdapter::execute_update(
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
  ) -> Result<ProgrammableTransactionBcs, Error> {
    IdentityMoveCallsAdapter::execute_update(None, None, identity, controller_cap, self.id(), package)
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))
  }

  fn parse_tx_effects(_tx_response: &dyn IotaTransactionBlockResponseT<Error = Error>) -> Result<Self::Output, Error> {
    Ok(())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdateValue<V> {
  new_value: V,
}

impl From<UpdateDidDocument> for UpdateValue<Vec<u8>> {
  fn from(value: UpdateDidDocument) -> Self {
    Self { new_value: value.0 }
  }
}

impl From<UpdateValue<Vec<u8>>> for UpdateDidDocument {
  fn from(value: UpdateValue<Vec<u8>>) -> Self {
    UpdateDidDocument(value.new_value)
  }
}
