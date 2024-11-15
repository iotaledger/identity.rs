use std::collections::HashMap;
use std::marker::PhantomData;

use crate::client::IdentityClient;
use crate::client::IotaKeySignature;
use crate::migration::Proposal;
use crate::sui::move_calls;
use crate::utils::MoveType;
use crate::Error;
use async_trait::async_trait;
use iota_sdk::rpc_types::IotaObjectData;
use iota_sdk::rpc_types::IotaTransactionBlockResponse;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder as Ptb;
use iota_sdk::types::transaction::Argument;
use iota_sdk::types::TypeTag;
use secret_storage::Signer;
use serde::Deserialize;
use serde::Serialize;

use super::CreateProposalTx;
use super::ExecuteProposalTx;
use super::OnChainIdentity;
use super::ProposalBuilder;
use super::ProposalT;

pub(crate) type IntentFn = Box<dyn FnOnce(&mut Ptb, &HashMap<ObjectID, (Argument, IotaObjectData)>) + Send>;

/// Action used to borrow in transaction [OnChainIdentity]'s assets.
#[derive(Default, Deserialize, Serialize)]
pub struct BorrowAction {
  objects: Vec<ObjectID>,
  #[serde(skip)]
  intent: Option<IntentFn>,
}

impl MoveType for BorrowAction {
  fn move_type(package: ObjectID) -> TypeTag {
    use std::str::FromStr;

    TypeTag::from_str(&format!("{package}::borrow_proposal::Borrow")).expect("valid move type")
  }
}

impl BorrowAction {
  /// Adds an object to the lists of objects that will be borrowed when executing
  /// this action in a proposal.
  pub fn borrow_object(&mut self, object_id: ObjectID) {
    self.objects.push(object_id);
  }

  /// Adds many objects. See [`BorrowAction::borrow_object`] for more details.
  pub fn borrow_objects<I>(&mut self, objects: I)
  where
    I: IntoIterator<Item = ObjectID>,
  {
    objects.into_iter().for_each(|obj_id| self.borrow_object(obj_id));
  }
}

impl<'i> ProposalBuilder<'i, BorrowAction> {
  /// Adds an object to the list of objects that will be borrowed when executing this action.
  pub fn borrow(mut self, object_id: ObjectID) -> Self {
    self.borrow_object(object_id);
    self
  }
  /// Adds many objects. See [`BorrowAction::borrow_object`] for more details.
  pub fn borrow_objects<I>(self, objects: I) -> Self
  where
    I: IntoIterator<Item = ObjectID>,
  {
    objects.into_iter().fold(self, |builder, obj| builder.borrow(obj))
  }
}

impl Proposal<BorrowAction> {
  /// Defines how the borrowed assets should be used.
  pub fn with_intent<F>(mut self, intent_fn: F) -> Self
  where
    F: FnOnce(&mut Ptb, &HashMap<ObjectID, (Argument, IotaObjectData)>) + Send + 'static,
  {
    self.action.intent = Some(Box::new(intent_fn));
    self
  }
}

#[async_trait]
impl ProposalT for Proposal<BorrowAction> {
  type Action = BorrowAction;
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
    let tx = move_calls::identity::propose_borrow(
      identity_ref,
      controller_cap_ref,
      action.objects,
      expiration,
      client.package_id(),
    )
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    Ok(CreateProposalTx {
      identity,
      tx,
      // Borrow proposals cannot be chain-executed as they have to be driven.
      chained_execution: false,
      _action: PhantomData,
    })
  }

  async fn execute<'i, S>(
    self,
    identity: &'i mut OnChainIdentity,
    client: &IdentityClient<S>,
  ) -> Result<ExecuteProposalTx<'i, Self::Action>, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let proposal_id = self.id();
    let action = self.into_action();
    let intent_fn = action.intent.ok_or_else(|| {
      Error::TransactionBuildingFailed(
        "missing intent; make sure to provide one by calling `Proposal::<BorrowAction>::with_intent`".to_string(),
      )
    })?;
    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .expect("identity exists on-chain");
    let controller_cap_ref = identity.get_controller_cap(client).await?;

    // Construct a list of `(ObjectRef, TypeTag)` from the list of objects to send.
    let object_data_list = {
      let mut object_data_list = vec![];
      for obj_id in action.objects {
        let object_data = super::obj_data_for_id(client, obj_id)
          .await
          .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
        object_data_list.push(object_data);
      }
      object_data_list
    };

    let tx = move_calls::identity::execute_borrow(
      identity_ref,
      controller_cap_ref,
      proposal_id,
      object_data_list,
      intent_fn,
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
