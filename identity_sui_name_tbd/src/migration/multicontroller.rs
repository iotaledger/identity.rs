use std::collections::HashMap;
use std::collections::HashSet;

use crate::sui::types::Bag;
use crate::sui::types::Number;
use crate::iota_sdk_abstraction::types::base_types::ObjectID;
use crate::iota_sdk_abstraction::types::collection_types::Entry;
use crate::iota_sdk_abstraction::types::collection_types::VecMap;
use crate::iota_sdk_abstraction::types::collection_types::VecSet;
use crate::iota_sdk_abstraction::types::id::UID;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
  try_from = "IotaProposal::<T>",
  into = "IotaProposal::<T>",
  bound(serialize = "T: Serialize + Clone")
)]
pub struct Proposal<T> {
  id: UID,
  expiration_epoch: Option<u64>,
  votes: u64,
  voters: HashSet<ObjectID>,
  action: T,
}

impl<T> Proposal<T> {
  pub fn id(&self) -> ObjectID {
    *self.id.object_id()
  }
  pub fn votes(&self) -> u64 {
    self.votes
  }
  pub(crate) fn votes_mut(&mut self) -> &mut u64 {
    &mut self.votes
  }
  pub fn action(&self) -> &T {
    &self.action
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IotaProposal<T> {
  id: UID,
  expiration_epoch: Option<Number<u64>>,
  votes: Number<u64>,
  voters: VecSet<ObjectID>,
  action: T,
}

impl<T> TryFrom<IotaProposal<T>> for Proposal<T> {
  type Error = <u64 as TryFrom<Number<u64>>>::Error;
  fn try_from(proposal: IotaProposal<T>) -> Result<Self, Self::Error> {
    let IotaProposal {
      id,
      expiration_epoch,
      votes,
      voters,
      action,
    } = proposal;
    let expiration_epoch = expiration_epoch.map(TryInto::try_into).transpose()?;
    let votes = votes.try_into()?;
    let voters = voters.contents.into_iter().collect();

    Ok(Self {
      id,
      expiration_epoch,
      votes,
      voters,
      action,
    })
  }
}

impl<T> From<Proposal<T>> for IotaProposal<T> {
  fn from(value: Proposal<T>) -> Self {
    let Proposal {
      id,
      expiration_epoch,
      votes,
      voters,
      action,
    } = value;
    let contents = voters.into_iter().collect();
    IotaProposal {
      id,
      expiration_epoch: expiration_epoch.map(Into::into),
      votes: votes.into(),
      voters: VecSet { contents },
      action,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(try_from = "IotaMulticontroller::<T>")]
pub struct Multicontroller<T> {
  controlled_value: T,
  controllers: HashMap<ObjectID, u64>,
  threshold: u64,
  active_proposals: HashSet<ObjectID>,
  proposals: Bag,
}

impl<T> Multicontroller<T> {
  pub fn controlled_value(&self) -> &T {
    &self.controlled_value
  }
  pub fn threshold(&self) -> u64 {
    self.threshold
  }
  pub fn proposals(&self) -> &HashSet<ObjectID> {
    &self.active_proposals
  }
  pub fn proposals_bag_id(&self) -> ObjectID {
    *self.proposals.id.object_id()
  }
  pub fn controller_voting_power(&self, controller_cap_id: ObjectID) -> Option<u64> {
    self.controllers.get(&controller_cap_id).copied()
  }
  pub fn into_inner(self) -> T {
    self.controlled_value
  }
  pub(crate) fn controllers(&self) -> &HashMap<ObjectID, u64> {
    &self.controllers
  }
  pub fn has_member(&self, cap_id: ObjectID) -> bool {
    self.controllers.contains_key(&cap_id)
  }
}

impl<T> TryFrom<IotaMulticontroller<T>> for Multicontroller<T> {
  type Error = <u64 as TryFrom<Number<u64>>>::Error;
  fn try_from(value: IotaMulticontroller<T>) -> Result<Self, Self::Error> {
    let IotaMulticontroller {
      controlled_value,
      controllers,
      threshold,
      active_proposals,
      proposals,
    } = value;
    let controllers = controllers
      .contents
      .into_iter()
      .map(|Entry { key: id, value: vp }| (u64::try_from(vp).map(|vp| (id, vp))))
      .collect::<Result<_, _>>()?;

    Ok(Multicontroller {
      controlled_value,
      controllers,
      threshold: threshold.try_into()?,
      active_proposals,
      proposals,
    })
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct IotaMulticontroller<T> {
  controlled_value: T,
  controllers: VecMap<ObjectID, Number<u64>>,
  threshold: Number<u64>,
  active_proposals: HashSet<ObjectID>,
  proposals: Bag,
}
