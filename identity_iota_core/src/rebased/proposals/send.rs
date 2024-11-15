use std::marker::PhantomData;
use std::ops::Deref;

use async_trait::async_trait;
use iota_sdk::rpc_types::IotaTransactionBlockResponse;
use iota_sdk::types::base_types::{IotaAddress, ObjectID};
use iota_sdk::types::TypeTag;
use secret_storage::Signer;
use serde::{Deserialize, Serialize};

use crate::rebased::client::{IdentityClient, IotaKeySignature};
use crate::rebased::sui::move_calls;
use crate::rebased::utils::MoveType;
use crate::rebased::{migration::OnChainIdentity, Error};

use super::{CreateProposalTx, ExecuteProposalTx, Proposal, ProposalBuilder, ProposalT};

/// An action used to transfer [`crate::migration::OnChainIdentity`]-owned assets to other addresses.
#[derive(Debug, Clone, Deserialize, Default, Serialize)]
#[serde(from = "IotaSendAction", into = "IotaSendAction")]
pub struct SendAction(Vec<(ObjectID, IotaAddress)>);

impl MoveType for SendAction {
  fn move_type(package: ObjectID) -> TypeTag {
    use std::str::FromStr;

    TypeTag::from_str(&format!("{package}::transfer_proposal::Send")).expect("valid move type")
  }
}

impl Deref for SendAction {
  type Target = [(ObjectID, IotaAddress)];
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl SendAction {
  /// Adds to the list of object to send the object with ID `object_id` and send it to address `recipient`.
  pub fn send_object(&mut self, object_id: ObjectID, recipient: IotaAddress) {
    self.0.push((object_id, recipient));
  }

  /// Adds multiple objects to the list of objects to send.
  pub fn send_objects<I>(&mut self, objects: I)
  where
    I: IntoIterator<Item = (ObjectID, IotaAddress)>,
  {
    objects
      .into_iter()
      .for_each(|(obj_id, recp)| self.send_object(obj_id, recp));
  }
}

impl<'i> ProposalBuilder<'i, SendAction> {
  /// Adds one object to the list of objects to send.
  pub fn object(mut self, object_id: ObjectID, recipient: IotaAddress) -> Self {
    self.send_object(object_id, recipient);
    self
  }

  /// Adds multiple objects to the list of objects to send.
  pub fn objects<I>(mut self, objects: I) -> Self
  where
    I: IntoIterator<Item = (ObjectID, IotaAddress)>,
  {
    self.send_objects(objects);
    self
  }
}

#[async_trait]
impl ProposalT for Proposal<SendAction> {
  type Action = SendAction;
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
    let tx = move_calls::identity::propose_send(
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
      // Send proposals cannot be chain-executed as they have to be driven.
      chained_execution: false,
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

    // Construct a list of `(ObjectRef, TypeTag)` from the list of objects to send.
    let object_type_list = {
      let ids = self.into_action().0.into_iter().map(|(obj_id, _rcp)| obj_id);
      let mut object_and_type_list = vec![];
      for obj_id in ids {
        let ref_and_type = super::obj_ref_and_type_for_id(client, obj_id)
          .await
          .map_err(|e| Error::ObjectLookup(e.to_string()))?;
        object_and_type_list.push(ref_and_type);
      }
      object_and_type_list
    };

    let tx = move_calls::identity::execute_send(
      identity_ref,
      controller_cap_ref,
      proposal_id,
      object_type_list,
      client.package_id(),
    )
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

#[derive(Debug, Deserialize, Serialize)]
struct IotaSendAction {
  objects: Vec<ObjectID>,
  recipients: Vec<IotaAddress>,
}

impl From<IotaSendAction> for SendAction {
  fn from(value: IotaSendAction) -> Self {
    let IotaSendAction { objects, recipients } = value;
    let transfer_map = objects.into_iter().zip(recipients).collect();
    SendAction(transfer_map)
  }
}

impl From<SendAction> for IotaSendAction {
  fn from(action: SendAction) -> Self {
    let (objects, recipients) = action.0.into_iter().unzip();
    Self { objects, recipients }
  }
}
