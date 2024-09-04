use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::DerefMut as _;
use std::str::FromStr as _;

use iota_sdk::rpc_types::IotaTransactionBlockEffects;
use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::collection_types::Entry;
use iota_sdk::types::collection_types::VecMap;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::Argument;
use iota_sdk::types::transaction::ProgrammableTransaction;
use iota_sdk::types::TypeTag;
use serde::Deserialize;
use serde::Serialize;

use crate::migration::OnChainIdentity;
use crate::sui::move_calls;
use crate::sui::types::Number;
use crate::utils::MoveType;
use crate::Error;

use super::Proposal;
use super::ProposalBuilder;
use super::ProposalT;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(try_from = "Modify")]
pub struct ConfigChange {
  threshold: Option<u64>,
  controllers_to_add: HashMap<IotaAddress, u64>,
  controllers_to_remove: HashSet<ObjectID>,
  controllers_voting_power: HashMap<ObjectID, u64>,
}

impl MoveType for ConfigChange {
  fn move_type(package: ObjectID) -> iota_sdk::types::TypeTag {
    TypeTag::from_str(&format!("{package}::config_proposal::Modify")).expect("valid type tag")
  }
}

impl ProposalT for Proposal<ConfigChange> {
  type Action = ConfigChange;
  type Output = ();

  fn make_create_tx(
    action: Self::Action,
    expiration: Option<u64>,
    identity_ref: OwnedObjectRef,
    controller_cap: ObjectRef,
    identity: &OnChainIdentity,
    package: ObjectID,
  ) -> Result<(ProgrammableTransactionBuilder, Argument), Error> {
    action.validate(identity)?;
    move_calls::identity::propose_config_change(
      identity_ref,
      controller_cap,
      expiration,
      action.threshold,
      action.controllers_to_add,
      action.controllers_to_remove,
      action.controllers_voting_power,
      package,
    )
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))
  }

  fn make_chained_execution_tx(
    ptb: ProgrammableTransactionBuilder,
    proposal_arg: Argument,
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    package: ObjectID,
  ) -> Result<ProgrammableTransaction, Error> {
    move_calls::identity::execute_config_change(
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
    move_calls::identity::execute_config_change(None, None, identity, controller_cap, self.id(), package)
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))
  }

  fn parse_tx_effects(_effects: IotaTransactionBlockEffects) -> Result<Self::Output, Error> {
    Ok(())
  }
}

impl<'i> ProposalBuilder<'i, ConfigChange> {
  pub fn threshold(mut self, threshold: u64) -> Self {
    self.set_threshold(threshold);
    self
  }

  pub fn add_controller(mut self, address: IotaAddress, voting_power: u64) -> Self {
    self.deref_mut().add_controller(address, voting_power);
    self
  }

  pub fn add_multiple_controllers<I>(mut self, controllers: I) -> Self
  where
    I: IntoIterator<Item = (IotaAddress, u64)>,
  {
    self.deref_mut().add_multiple_controllers(controllers);
    self
  }

  pub fn remove_controller(mut self, controller_id: ObjectID) -> Self {
    self.deref_mut().remove_controller(controller_id);
    self
  }

  pub fn remove_multiple_controllers<I>(mut self, controllers: I) -> Self
  where
    I: IntoIterator<Item = ObjectID>,
  {
    self.deref_mut().remove_multiple_controllers(controllers);
    self
  }
}

impl ConfigChange {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn set_threshold(&mut self, new_threshold: u64) {
    self.threshold = Some(new_threshold);
  }

  pub fn add_controller(&mut self, address: IotaAddress, voting_power: u64) {
    self.controllers_to_add.insert(address, voting_power);
  }

  pub fn add_multiple_controllers<I>(&mut self, controllers: I)
  where
    I: IntoIterator<Item = (IotaAddress, u64)>,
  {
    for (addr, vp) in controllers {
      self.add_controller(addr, vp)
    }
  }

  pub fn remove_controller(&mut self, controller_id: ObjectID) {
    self.controllers_to_remove.insert(controller_id);
  }

  pub fn remove_multiple_controllers<I>(&mut self, controllers: I)
  where
    I: IntoIterator<Item = ObjectID>,
  {
    for controller in controllers {
      self.remove_controller(controller)
    }
  }

  fn validate(&self, identity: &OnChainIdentity) -> Result<(), Error> {
    let new_threshold = self.threshold.unwrap_or(identity.threshold());
    let mut controllers = identity.controllers().clone();
    // check if update voting powers is valid
    for (controller, new_vp) in &self.controllers_voting_power {
      match controllers.get_mut(controller) {
        Some(vp) => *vp = *new_vp,
        None => {
          return Err(Error::InvalidConfig(format!(
            "object \"{controller}\" is not among identity \"{}\"'s controllers",
            identity.id()
          )))
        }
      }
    }
    // check if deleting controllers is valid
    for controller in &self.controllers_to_remove {
      if controllers.remove(controller).is_none() {
        return Err(Error::InvalidConfig(format!(
          "object \"{controller}\" is not among identity \"{}\"'s controllers",
          identity.id()
        )));
      }
    }
    // check if adding controllers is valid
    for (controller, vp) in &self.controllers_to_add {
      if controllers.insert((*controller).into(), *vp).is_some() {
        return Err(Error::InvalidConfig(format!(
          "object \"{controller}\" is already among identity \"{}\"'s controllers",
          identity.id()
        )));
      }
    }
    // check whether the new threshold allows to interact with the identity
    if new_threshold > controllers.values().sum() {
      return Err(Error::InvalidConfig(
        "the resulting configuration will result in an unaccessible identity".to_string(),
      ));
    }
    Ok(())
  }
}

#[derive(Debug, Deserialize)]
struct Modify {
  threshold: Option<Number<u64>>,
  controllers_to_add: VecMap<IotaAddress, Number<u64>>,
  controllers_to_remove: HashSet<ObjectID>,
  controllers_to_update: VecMap<ObjectID, Number<u64>>,
}

impl TryFrom<Modify> for ConfigChange {
  type Error = <u64 as TryFrom<Number<u64>>>::Error;
  fn try_from(value: Modify) -> Result<Self, Self::Error> {
    let Modify {
      threshold,
      controllers_to_add,
      controllers_to_remove,
      controllers_to_update,
    } = value;
    let threshold = threshold.map(|num| num.try_into()).transpose()?;
    let controllers_to_add = controllers_to_add
      .contents
      .into_iter()
      .map(|Entry { key, value }| value.try_into().map(|n| (key, n)))
      .collect::<Result<_, _>>()?;
    let controllers_to_update = controllers_to_update
      .contents
      .into_iter()
      .map(|Entry { key, value }| value.try_into().map(|n| (key, n)))
      .collect::<Result<_, _>>()?;
    Ok(Self {
      threshold,
      controllers_to_add,
      controllers_to_remove,
      controllers_voting_power: controllers_to_update,
    })
  }
}
