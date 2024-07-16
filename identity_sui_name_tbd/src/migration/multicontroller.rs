use std::collections::HashMap;

use crate::sui::types::Hashable;
use crate::sui::types::Number;
use crate::sui::types::VecMap;
use crate::sui::types::VecSet;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::id::ID;
use iota_sdk::types::id::UID;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "IotaProposal", into = "IotaProposal")]
pub struct Proposal {
  id: UID,
  expiration_epoch: Option<u64>,
  votes: u64,
  voters: VecSet<Hashable<ID>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IotaProposal {
  id: UID,
  expiration_epoch: Option<Number<u64>>,
  votes: Number<u64>,
  voters: VecSet<Hashable<ID>>,
}

impl TryFrom<IotaProposal> for Proposal {
  type Error = <u64 as TryFrom<Number<u64>>>::Error;
  fn try_from(proposal: IotaProposal) -> Result<Self, Self::Error> {
    let IotaProposal {
      id,
      expiration_epoch,
      votes,
      voters,
    } = proposal;
    let expiration_epoch = expiration_epoch.map(TryInto::try_into).transpose()?;
    let votes = votes.try_into()?;

    Ok(Self {
      id,
      expiration_epoch,
      votes,
      voters,
    })
  }
}

impl From<Proposal> for IotaProposal {
  fn from(value: Proposal) -> Self {
    let Proposal {
      id,
      expiration_epoch,
      votes,
      voters,
    } = value;
    IotaProposal {
      id,
      expiration_epoch: expiration_epoch.map(Into::into),
      votes: votes.into(),
      voters,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(try_from = "IotaMulticontroller::<T>")]
pub struct Multicontroller<T> {
  controlled_value: T,
  controllers: HashMap<Hashable<ID>, u64>,
  threshold: u64,
  proposals: HashMap<String, Proposal>,
}

impl<T> Multicontroller<T> {
  pub fn controlled_value(&self) -> &T {
    &self.controlled_value
  }
  pub fn threshold(&self) -> u64 {
    self.threshold
  }
  pub fn controller_voting_power(&self, controller_cap_id: ObjectID) -> Option<u64> {
    self.controllers.get(&Hashable(ID::new(controller_cap_id))).copied()
  }
  pub fn proposals(&self) -> &HashMap<String, Proposal> {
    &self.proposals
  }
  pub fn into_inner(self) -> T {
    self.controlled_value
  }
  pub(crate) fn controllers(&self) -> &HashMap<Hashable<ID>, u64> {
    &self.controllers
  }
  pub fn has_member(&self, cap_id: ObjectID) -> bool {
    let cap = Hashable(ID::new(cap_id));
    self.controllers.contains_key(&cap)
  }
}

impl<T> TryFrom<IotaMulticontroller<T>> for Multicontroller<T> {
  type Error = <u64 as TryFrom<Number<u64>>>::Error;
  fn try_from(value: IotaMulticontroller<T>) -> Result<Self, Self::Error> {
    let IotaMulticontroller {
      controlled_value,
      controllers,
      threshold,
      proposals,
    } = value;
    let controllers = controllers
      .into_inner_iter()
      .map(|(id, vp)| (u64::try_from(vp).map(|vp| (id, vp))))
      .collect::<Result<_, _>>()?;
    let proposals = proposals
      .into_inner_iter()
      .map(|(name, p)| Proposal::try_from(p).map(|p| (name, p)))
      .collect::<Result<_, _>>()?;

    Ok(Multicontroller {
      controlled_value,
      controllers,
      threshold: threshold.try_into()?,
      proposals,
    })
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct IotaMulticontroller<T> {
  controlled_value: T,
  controllers: VecMap<Hashable<ID>, Number<u64>>,
  threshold: Number<u64>,
  proposals: VecMap<String, IotaProposal>,
}
