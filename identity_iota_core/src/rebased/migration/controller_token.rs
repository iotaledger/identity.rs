// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_interaction::types::base_types::ObjectID;
use iota_interaction::types::id::UID;
use iota_interaction::types::TypeTag;
use iota_interaction::MoveType;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

/// An object that authenticates the actor presenting it
/// as a controller of shared object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerToken {
  #[serde(deserialize_with = "deserialize_from_uid")]
  id: ObjectID,
  controller_of: ObjectID,
}

fn deserialize_from_uid<'de, D>(deserializer: D) -> Result<ObjectID, D::Error>
where
  D: Deserializer<'de>,
{
  UID::deserialize(deserializer).map(|uid| *uid.object_id())
}

impl MoveType for ControllerToken {
  fn move_type(package: ObjectID) -> TypeTag {
    format!("{package}::controller::ControllerCap")
      .parse()
      .expect("valid Move type")
  }
}

impl ControllerToken {
  /// ID of this [ControllerToken].
  pub fn id(&self) -> ObjectID {
    self.id
  }

  /// ID of the object this token controls.
  pub fn controller_of(&self) -> ObjectID {
    self.controller_of
  }
}
