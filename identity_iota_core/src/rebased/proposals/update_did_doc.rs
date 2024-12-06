// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;

use crate::rebased::client::IdentityClient;
use crate::rebased::client::IotaKeySignature;
use crate::rebased::iota::move_calls;
use crate::IotaDocument;
use async_trait::async_trait;
use iota_sdk::rpc_types::IotaTransactionBlockResponse;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::TypeTag;
use secret_storage::Signer;
use serde::Deserialize;
use serde::Serialize;

use crate::rebased::migration::OnChainIdentity;
use crate::rebased::migration::Proposal;
use crate::rebased::utils::MoveType;
use crate::rebased::Error;

use super::CreateProposalTx;
use super::ExecuteProposalTx;
use super::ProposalT;

/// Proposal's action for updating a DID Document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(into = "UpdateValue::<Vec<u8>>", from = "UpdateValue::<Vec<u8>>")]
pub struct UpdateDidDocument(Vec<u8>);

impl MoveType for UpdateDidDocument {
  fn move_type(package: ObjectID) -> TypeTag {
    use std::str::FromStr;

    TypeTag::from_str(&format!("{package}::update_value_proposal::UpdateValue<vector<u8>>")).expect("valid TypeTag")
  }
}

impl UpdateDidDocument {
  /// Creates a new [`UpdateDidDocument`] action.
  pub fn new(document: IotaDocument) -> Self {
    Self(document.pack().expect("a valid IotaDocument is packable"))
  }
}

#[async_trait]
impl ProposalT for Proposal<UpdateDidDocument> {
  type Action = UpdateDidDocument;
  type Output = ();

  async fn create<'i, S>(
    action: Self::Action,
    expiration: Option<u64>,
    identity: &'i mut OnChainIdentity,
    client: &IdentityClient<S>,
  ) -> Result<CreateProposalTx<'i, Self::Action>, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .expect("identity exists on-chain");
    let controller_cap_ref = identity.get_controller_cap(client).await?;
    let sender_vp = identity
      .controller_voting_power(controller_cap_ref.0)
      .expect("controller exists");
    let chained_execution = sender_vp >= identity.threshold();
    let tx = move_calls::identity::propose_update(
      identity_ref,
      controller_cap_ref,
      action.0,
      expiration,
      client.package_id(),
    )
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    Ok(CreateProposalTx {
      identity,
      tx,
      chained_execution,
      _action: PhantomData,
    })
  }

  async fn into_tx<'i, S>(
    self,
    identity: &'i mut OnChainIdentity,
    client: &IdentityClient<S>,
  ) -> Result<ExecuteProposalTx<'i, Self::Action>, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let proposal_id = self.id();
    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .expect("identity exists on-chain");
    let controller_cap_ref = identity.get_controller_cap(client).await?;

    let tx = move_calls::identity::execute_update(identity_ref, controller_cap_ref, proposal_id, client.package_id())
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    Ok(ExecuteProposalTx {
      identity,
      tx,
      _action: PhantomData,
    })
  }

  fn parse_tx_effects(_tx_response: &IotaTransactionBlockResponse) -> Result<Self::Output, Error> {
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
