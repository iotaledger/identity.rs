use std::collections::HashMap;

use crate::sui::types::Hashable;
use crate::sui::types::Number;
use crate::sui::types::VecMap;
use crate::sui::types::VecSet;
use serde::Deserialize;
use serde::Serialize;
use sui_sdk::types::id::ID;
use sui_sdk::types::id::UID;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "SuiProposal", into = "SuiProposal")]
pub(crate) struct Proposal {
  id: UID,
  expiration_epoch: Option<u64>,
  votes: u64,
  voters: VecSet<Hashable<ID>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SuiProposal {
  id: UID,
  expiration_epoch: Option<Number<u64>>,
  votes: Number<u64>,
  voters: VecSet<Hashable<ID>>,
}

impl TryFrom<SuiProposal> for Proposal {
  type Error = <u64 as TryFrom<Number<u64>>>::Error;
  fn try_from(proposal: SuiProposal) -> Result<Self, Self::Error> {
    let SuiProposal {
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

impl From<Proposal> for SuiProposal {
  fn from(value: Proposal) -> Self {
    let Proposal {
      id,
      expiration_epoch,
      votes,
      voters,
    } = value;
    SuiProposal {
      id,
      expiration_epoch: expiration_epoch.map(Into::into),
      votes: votes.into(),
      voters,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(try_from = "SuiMulticontroller::<T>")]
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
  pub fn into_inner(self) -> T {
    self.controlled_value
  }
}

impl<T> TryFrom<SuiMulticontroller<T>> for Multicontroller<T> {
  type Error = <u64 as TryFrom<Number<u64>>>::Error;
  fn try_from(value: SuiMulticontroller<T>) -> Result<Self, Self::Error> {
    let SuiMulticontroller {
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
struct SuiMulticontroller<T> {
  controlled_value: T,
  controllers: VecMap<Hashable<ID>, Number<u64>>,
  threshold: Number<u64>,
  proposals: VecMap<String, SuiProposal>,
}
