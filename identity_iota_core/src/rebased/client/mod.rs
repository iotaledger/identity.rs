// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod full_client;
mod read_only;

use anyhow::{anyhow, Context};
pub use full_client::*;
use identity_iota_interaction::IotaClientTrait;
use iota_sdk::rpc_types::{IotaData, IotaObjectDataOptions, OwnedObjectRef};
use iota_sdk::types::base_types::{IotaAddress, ObjectID};
use iota_sdk::types::crypto::PublicKey;
pub use read_only::*;

pub use identity_iota_interaction::IotaKeySignature;
use secret_storage::Signer;
use serde::de::DeserializeOwned;

use crate::iota_interaction_rust::IotaClientAdapter;
use crate::NetworkName;
use async_trait::async_trait;

#[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync", async_trait)]
/// A trait that defines the core read-only operations for core clients.
pub trait CoreClientReadOnly {
  /// Returns the package ID associated with the Client.
  ///
  /// The package ID uniquely identifies the deployed Contract on the
  /// IOTA network.
  fn package_id(&self) -> ObjectID;

  /// Returns the name of the network the client is connected to.
  ///
  /// This is typically a human-readable alias or identifier for the IOTA network.
  fn network_name(&self) -> &NetworkName;

  /// Returns the underlying [`IotaClientAdapter`] used by this client.
  ///
  /// This allows access to lower-level client operations if needed.
  fn client_adapter(&self) -> IotaClientAdapter;

  /// Retrieves a _Move_ Object by its ID.
  ///
  /// This function parses the object ID and returns the corresponding object
  ///
  /// # Arguments
  ///
  /// * `object_id` - The unique identifier of the object to retrieve.
  ///
  /// # Returns
  ///
  /// Returns `Ok(Some(T))` if the object is found, `Ok(None)` if not found,
  /// or an error if the operation fails.
  async fn get_object_by_id<T: DeserializeOwned>(&self, object_id: ObjectID) -> anyhow::Result<Option<T>> {
    self
      .client_adapter()
      .read_api()
      .get_object_with_options(object_id, IotaObjectDataOptions::new().with_content())
      .await
      .context("lookup request failed")
      .and_then(|res| res.data.context("missing data in response"))
      .and_then(|data| data.content.context("missing object content in data"))
      .and_then(|content| content.try_into_move().context("not a move object"))
      .and_then(|obj| {
        serde_json::from_value(obj.fields.to_json_value())
          .map_err(|err| anyhow!("failed to deserialize move object; {err}"))
      })
      .context("failed to get object by id")
  }

  /// Retrieves an object's [`OwnedObjectRef`], if any.
  ///
  /// # Arguments
  ///
  /// * `object_id` - The unique identifier of the object to retrieve.
  ///
  /// # Returns
  ///
  /// Returns `Ok(Some(OwnedObjectRef))` if the object is found, `Ok(None)` if not found,
  /// or an error if the operation fails.
  async fn get_object_ref_by_id(&self, object_id: ObjectID) -> anyhow::Result<Option<OwnedObjectRef>> {
    self
      .client_adapter()
      .read_api()
      .get_object_with_options(object_id, IotaObjectDataOptions::default().with_owner())
      .await
      .map(|response| {
        response.data.map(|obj_data| OwnedObjectRef {
          owner: obj_data.owner.expect("requested data"),
          reference: obj_data.object_ref().into(),
        })
      })
      .context("failed to get object ref by id")
  }


}

#[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync", async_trait)]
/// A trait that defines the core read-write operations for core clients.
pub trait CoreClient: CoreClientReadOnly {
  /// Returns the signer of the client.
  fn signer<S: Signer<IotaKeySignature>>(&self) -> &S;

  /// Returns this Client's sender address
  fn sender_address(&self) -> IotaAddress;

  /// Returns the bytes of the sender's public key.
  fn sender_public_key(&self) -> &PublicKey;
}
