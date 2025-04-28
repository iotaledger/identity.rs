// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod full_client;
mod read_only;

use anyhow::anyhow;
use anyhow::Context;
pub use full_client::*;
use identity_iota_interaction::IotaClientTrait;
use identity_iota_interaction::MoveType;
use iota_sdk::rpc_types::IotaData;
use iota_sdk::rpc_types::IotaObjectData;
use iota_sdk::rpc_types::IotaObjectDataFilter;
use iota_sdk::rpc_types::IotaObjectDataOptions;
use iota_sdk::rpc_types::IotaObjectResponseQuery;
use iota_sdk::rpc_types::IotaParsedData;
use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::crypto::PublicKey;
use move_core_types::language_storage::StructTag;
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
  fn client_adapter(&self) -> &IotaClientAdapter;

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
  async fn get_object_by_id<T: DeserializeOwned>(&self, object_id: ObjectID) -> anyhow::Result<T> {
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

  /// Queries `address` owned objects, returning the first object for which `predicate` returns `true`.
  async fn find_object_for_address<T, P>(&self, address: IotaAddress, predicate: P) -> anyhow::Result<Option<T>>
  where
    T: MoveType + DeserializeOwned,
    P: Fn(&T) -> bool + Send,
  {
    let tag = T::move_type(self.package_id())
      .to_string()
      .parse()
      .expect("type tag is a valid struct tag");
    let filter = IotaObjectResponseQuery::new(
      Some(IotaObjectDataFilter::StructType(tag)),
      Some(IotaObjectDataOptions::default().with_content()),
    );
    let mut cursor = None;
    loop {
      let mut page = self
        .client_adapter()
        .read_api()
        .get_owned_objects(address, Some(filter.clone()), cursor, Some(25))
        .await?;
      let maybe_obj = std::mem::take(&mut page.data)
        .into_iter()
        .filter_map(|res| res.data)
        .filter_map(|data| data.content)
        .filter_map(|obj_data| {
          let IotaParsedData::MoveObject(move_object) = obj_data else {
            unreachable!()
          };
          serde_json::from_value(move_object.fields.to_json_value()).ok()
        })
        .find(&predicate);
      cursor = page.next_cursor;

      if maybe_obj.is_some() {
        return Ok(maybe_obj);
      }
      if !page.has_next_page {
        break;
      }
    }

    Ok(None)
  }

  /// Retrieves coins owned by the specified address with a balance of at least `balance`.
  ///
  /// # Arguments
  ///
  /// * `owner` - The address of the owner of the coins.
  /// * `balance` - The minimum balance required for the coins.
  ///
  /// # Returns
  ///
  /// Returns `Ok(Vec<ObjectRef>)` if the coins are found, or an error if the operation fails.
  async fn get_iota_coins_with_at_least_balance(
    &self,
    owner: IotaAddress,
    balance: u64,
  ) -> anyhow::Result<Vec<ObjectRef>> {
    let mut coins = self
      .client_adapter()
      .coin_read_api()
      .get_coins(owner, Some(String::from("0x2::iota::IOTA")), None, None)
      .await?
      .data;
    coins.sort_unstable_by_key(|coin| coin.balance);

    let mut needed_coins = vec![];
    let mut needed_coins_balance = 0;
    while let Some(coin) = coins.pop() {
      needed_coins_balance += coin.balance;
      needed_coins.push(coin.object_ref());

      if needed_coins_balance >= balance {
        return Ok(needed_coins);
      }
    }

    anyhow::bail!("address {owner} does not have enough coins to form a balance of {balance}");
  }

  /// Queries the object owned by this sender address and returns the first one
  /// that matches `tag` and for which `predicate` returns `true`.
  async fn find_owned_ref_for_address<P>(
    &self,
    address: IotaAddress,
    tag: StructTag,
    predicate: P,
  ) -> Result<Option<ObjectRef>, anyhow::Error>
  where
    P: Fn(&IotaObjectData) -> bool + Send,
  {
    let filter = IotaObjectResponseQuery::new_with_filter(IotaObjectDataFilter::StructType(tag));

    let mut cursor = None;
    loop {
      let mut page = self
        .client_adapter()
        .read_api()
        .get_owned_objects(address, Some(filter.clone()), cursor, None)
        .await?;
      let obj_ref = std::mem::take(&mut page.data)
        .into_iter()
        .filter_map(|res| res.data)
        .find(|obj| predicate(obj))
        .map(|obj_data| obj_data.object_ref());
      cursor = page.next_cursor;

      if obj_ref.is_some() {
        return Ok(obj_ref);
      }
      if !page.has_next_page {
        break;
      }
    }

    Ok(None)
  }
}

#[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync", async_trait)]
/// A trait that defines the core read-write operations for core clients.
pub trait CoreClient<S: Signer<IotaKeySignature>>: CoreClientReadOnly {
  /// Returns the signer of the client.
  fn signer(&self) -> &S;

  /// Returns this Client's sender address
  fn sender_address(&self) -> IotaAddress;

  /// Returns the bytes of the sender's public key.
  fn sender_public_key(&self) -> &PublicKey;
}
