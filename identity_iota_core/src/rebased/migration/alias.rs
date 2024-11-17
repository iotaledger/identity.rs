// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::id::UID;
use iota_sdk::types::TypeTag;
use iota_sdk::types::STARDUST_PACKAGE_ID;
use serde;
use serde::Deserialize;
use serde::Serialize;

use crate::rebased::client::IdentityClientReadOnly;
use crate::rebased::utils::MoveType;
use crate::rebased::Error;

/// A legacy IOTA Stardust Output type, used to store DID Documents.
#[derive(Debug, Deserialize, Serialize)]
pub struct UnmigratedAlias {
  /// The ID of the Alias = hash of the Output ID that created the Alias Output in Stardust.
  /// This is the AliasID from Stardust.
  pub id: UID,

  /// The last State Controller address assigned before the migration.
  pub legacy_state_controller: Option<IotaAddress>,
  /// A counter increased by 1 every time the alias was state transitioned.
  pub state_index: u32,
  /// State metadata that can be used to store additional information.
  pub state_metadata: Option<Vec<u8>>,

  /// The sender feature.
  pub sender: Option<IotaAddress>,
  /// The metadata feature.  pub metadata: Option<Vec<u8>>,

  /// The immutable issuer feature.
  pub immutable_issuer: Option<IotaAddress>,
  /// The immutable metadata feature.
  pub immutable_metadata: Option<Vec<u8>>,
}

impl MoveType for UnmigratedAlias {
  fn move_type(_: ObjectID) -> TypeTag {
    format!("{STARDUST_PACKAGE_ID}::alias::Alias")
      .parse()
      .expect("valid move type")
  }
}

/// Resolves an [`UnmigratedAlias`] given its ID `object_id`.
pub async fn get_alias(client: &IdentityClientReadOnly, object_id: ObjectID) -> Result<Option<UnmigratedAlias>, Error> {
  match client.get_object_by_id(object_id).await {
    Ok(alias) => Ok(Some(alias)),
    Err(Error::ObjectLookup(err_msg)) if err_msg.contains("missing data") => Ok(None),
    Err(e) => Err(e),
  }
}
