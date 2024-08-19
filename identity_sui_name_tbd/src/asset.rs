use std::str::FromStr as _;

use crate::client::IdentityClient;
use crate::client::IotaKeySignature;
use crate::sui::move_calls;
use crate::utils::MoveType;
use crate::Error;
use anyhow::anyhow;
use anyhow::Context;
use iota_sdk::rpc_types::IotaData as _;
use iota_sdk::rpc_types::IotaObjectDataOptions;
use iota_sdk::rpc_types::IotaTransactionBlockEffectsAPI as _;
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::base_types::SequenceNumber;
use iota_sdk::types::id::UID;
use iota_sdk::types::object::Owner;
use iota_sdk::types::Identifier;
use iota_sdk::types::TypeTag;
use iota_sdk::IotaClient;
use move_core_types::language_storage::StructTag;
use secret_storage::Signer;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticatedAsset<T> {
  id: UID,
  #[serde(
    deserialize_with = "deserialize_inner",
    bound(deserialize = "T: for<'a> Deserialize<'a>")
  )]
  inner: T,
  owner: IotaAddress,
  origin: IotaAddress,
  mutable: bool,
  transferable: bool,
  deletable: bool,
}

fn deserialize_inner<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  D: Deserializer<'de>,
  T: for<'a> Deserialize<'a>,
{
  use serde::de::Error as _;

  match std::any::type_name::<T>() {
    "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => {
      String::deserialize(deserializer).and_then(|s| serde_json::from_str(&s).map_err(D::Error::custom))
    }
    _ => T::deserialize(deserializer),
  }
}

impl<T> AuthenticatedAsset<T>
where
  T: for<'de> Deserialize<'de>,
{
  pub async fn get_by_id(id: ObjectID, client: &IotaClient) -> Result<Self, Error> {
    let res = client
      .read_api()
      .get_object_with_options(id, IotaObjectDataOptions::new().with_content())
      .await?;
    let Some(data) = res.data else {
      return Err(Error::ObjectLookup(res.error.map_or(String::new(), |e| e.to_string())));
    };
    data
      .content
      .ok_or_else(|| anyhow!("No content for object with ID {id}"))
      .and_then(|content| content.try_into_move().context("not a Move object"))
      .and_then(|obj_data| {
        serde_json::from_value(obj_data.fields.to_json_value()).context("failed to deserialize move object")
      })
      .map_err(|e| Error::ObjectLookup(e.to_string()))
  }
}

impl<T> AuthenticatedAsset<T> {
  async fn object_ref(&self, client: &IotaClient) -> Result<ObjectRef, Error> {
    client
      .read_api()
      .get_object_with_options(self.id(), IotaObjectDataOptions::default())
      .await?
      .object_ref_if_exists()
      .ok_or_else(|| Error::ObjectLookup("missing object reference in response".to_owned()))
  }

  pub fn id(&self) -> ObjectID {
    *self.id.object_id()
  }

  pub fn content(&self) -> &T {
    &self.inner
  }
}

impl<T: MoveType> AuthenticatedAsset<T> {
  pub async fn transfer<S>(
    self,
    recipient: IotaAddress,
    gas_budget: u64,
    client: &IdentityClient<S>,
  ) -> Result<TransferProposal, Error>
  where
    S: Signer<IotaKeySignature>,
  {
    if !self.transferable {
      return Err(Error::InvalidConfig(format!(
        "`AuthenticatedAsset` {} is not transferable",
        self.id()
      )));
    }
    let tx = move_calls::asset::transfer::<T>(self.object_ref(client).await?, recipient, client.package_id())?;
    for id in client
      .execute_transaction(tx, gas_budget)
      .await?
      .effects
      .ok_or_else(|| Error::TransactionUnexpectedResponse("could not find effects in transaction response".to_owned()))?
      .created()
      .iter()
      .map(|obj| obj.reference.object_id)
    {
      let object_type = client
        .read_api()
        .get_object_with_options(id, IotaObjectDataOptions::new().with_type())
        .await?
        .data
        .context("no data in response")
        .and_then(|data| Ok(data.object_type()?.to_string()))
        .map_err(|e| Error::ObjectLookup(e.to_string()))?;

      if object_type == TransferProposal::move_type(client.package_id()).to_string() {
        return TransferProposal::get_by_id(id, client).await;
      }
    }

    Err(Error::TransactionUnexpectedResponse(
      "no proposal was created in this transaction".to_owned(),
    ))
  }

  pub async fn delete<S>(self, gas_budget: u64, client: &IdentityClient<S>) -> Result<(), Error>
  where
    S: Signer<IotaKeySignature>,
  {
    if !self.deletable {
      return Err(Error::InvalidConfig(format!(
        "`AuthenticatedAsset` {} is cannot be deleted",
        self.id()
      )));
    }

    let tx = move_calls::asset::delete::<T>(self.object_ref(client).await?, client.package_id())?;
    let response = client.execute_transaction(tx, gas_budget).await?;

    if response.errors.is_empty() {
      Ok(())
    } else {
      let err_str = response.errors.join("; ");
      Err(Error::TransactionUnexpectedResponse(err_str))
    }
  }
}

impl<T: MoveType + Serialize + Clone> AuthenticatedAsset<T> {
  pub async fn set_content<S>(
    &mut self,
    new_content: T,
    gas_budget: u64,
    client: &IdentityClient<S>,
  ) -> Result<(), Error>
  where
    S: Signer<IotaKeySignature>,
  {
    if !self.mutable {
      return Err(Error::InvalidConfig(format!(
        "`AuthenticatedAsset` {} is immutable",
        self.id()
      )));
    }

    let tx = move_calls::asset::update(self.object_ref(client).await?, new_content.clone(), client.package_id())?;
    let response = client.execute_transaction(tx, gas_budget).await?;

    if response.errors.is_empty() {
      self.inner = new_content;
      Ok(())
    } else {
      let err_str = response.errors.join("; ");
      Err(Error::TransactionUnexpectedResponse(err_str))
    }
  }
}

#[derive(Debug)]
pub struct AuthenticatedAssetBuilder<T> {
  inner: T,
  mutable: bool,
  transferable: bool,
  deletable: bool,
  gas_budget: Option<u64>,
}

impl<T: MoveType> MoveType for AuthenticatedAsset<T> {
  fn move_type(package: ObjectID) -> TypeTag {
    TypeTag::Struct(Box::new(StructTag {
      address: package.into(),
      module: Identifier::new("asset").expect("valid utf8"),
      name: Identifier::new("AuthenticatedAsset").expect("valid utf8"),
      type_params: vec![T::move_type(package)],
    }))
  }
}

impl<T> AuthenticatedAssetBuilder<T> {
  pub fn new(content: T) -> Self {
    Self {
      inner: content,
      mutable: false,
      transferable: false,
      deletable: false,
      gas_budget: None,
    }
  }

  pub fn mutable(mut self, mutable: bool) -> Self {
    self.mutable = mutable;
    self
  }

  pub fn transferable(mut self, transferable: bool) -> Self {
    self.transferable = transferable;
    self
  }

  pub fn deletable(mut self, deletable: bool) -> Self {
    self.deletable = deletable;
    self
  }

  pub fn gas_budget(mut self, budget: u64) -> Self {
    self.gas_budget = Some(budget);
    self
  }
}

impl<T> AuthenticatedAssetBuilder<T>
where
  T: MoveType + Serialize + for<'de> Deserialize<'de>,
{
  pub async fn finish<S>(self, client: &IdentityClient<S>) -> Result<AuthenticatedAsset<T>, Error>
  where
    S: Signer<IotaKeySignature>,
  {
    let AuthenticatedAssetBuilder {
      inner,
      mutable,
      transferable,
      deletable,
      gas_budget,
    } = self;
    let tx = move_calls::asset::new(inner, mutable, transferable, deletable, client.package_id())?;

    let gas_budget = gas_budget.ok_or_else(|| Error::GasIssue("missing gas budget".to_owned()))?;
    let created_asset_id = client
      .execute_transaction(tx, gas_budget)
      .await?
      .effects
      .ok_or_else(|| Error::TransactionUnexpectedResponse("could not find effects in transaction response".to_owned()))?
      .created()
      .first()
      .ok_or_else(|| Error::TransactionUnexpectedResponse("no object was created in this transaction".to_owned()))?
      .object_id();

    AuthenticatedAsset::get_by_id(created_asset_id, client).await
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferProposal {
  id: UID,
  asset_id: ObjectID,
  sender_cap_id: ObjectID,
  sender_address: IotaAddress,
  recipient_cap_id: ObjectID,
  recipient_address: IotaAddress,
  done: bool,
}

impl MoveType for TransferProposal {
  fn move_type(package: ObjectID) -> TypeTag {
    TypeTag::Struct(Box::new(StructTag {
      address: package.into(),
      module: Identifier::new("asset").expect("valid identifier"),
      name: Identifier::new("TransferProposal").expect("valid identifier"),
      type_params: vec![],
    }))
  }
}

impl TransferProposal {
  pub async fn get_by_id(id: ObjectID, client: &IotaClient) -> Result<Self, Error> {
    let res = client
      .read_api()
      .get_object_with_options(id, IotaObjectDataOptions::new().with_content())
      .await?;
    let Some(data) = res.data else {
      return Err(Error::ObjectLookup(res.error.map_or(String::new(), |e| e.to_string())));
    };
    data
      .content
      .ok_or_else(|| anyhow!("No content for object with ID {id}"))
      .and_then(|content| content.try_into_move().context("not a Move object"))
      .and_then(|obj_data| {
        serde_json::from_value(obj_data.fields.to_json_value()).context("failed to deserialize move object")
      })
      .map_err(|e| Error::ObjectLookup(e.to_string()))
  }

  async fn get_cap<S>(&self, cap_type: &str, client: &IdentityClient<S>) -> Result<ObjectRef, Error> {
    let cap_tag = StructTag::from_str(&format!("{}::asset::{cap_type}", client.package_id()))
      .map_err(|e| Error::ParsingFailed(e.to_string()))?;
    client
      .find_owned_ref(cap_tag, |obj_data| {
        cap_type == "SenderCap" && self.sender_cap_id == obj_data.object_id
          || cap_type == "RecipientCap" && self.recipient_cap_id == obj_data.object_id
      })
      .await?
      .ok_or_else(|| {
        Error::MissingPermission(format!(
          "no owned `{cap_type}` for transfer proposal {}",
          self.id.object_id(),
        ))
      })
  }

  async fn asset_metadata(&self, client: &IotaClient) -> anyhow::Result<(ObjectRef, TypeTag)> {
    let res = client
      .read_api()
      .get_object_with_options(self.asset_id, IotaObjectDataOptions::default().with_type())
      .await?;
    let asset_ref = res
      .object_ref_if_exists()
      .context("missing object reference in response")?;
    let param_type = res
      .data
      .context("missing data")
      .and_then(|data| data.type_.context("missing type"))
      .and_then(StructTag::try_from)
      .and_then(|mut tag| {
        if tag.type_params.is_empty() {
          anyhow::bail!("no type parameter")
        } else {
          Ok(tag.type_params.remove(0))
        }
      })?;

    Ok((asset_ref, param_type))
  }

  async fn initial_shared_version(&self, client: &IotaClient) -> anyhow::Result<SequenceNumber> {
    let owner = client
      .read_api()
      .get_object_with_options(*self.id.object_id(), IotaObjectDataOptions::default().with_owner())
      .await?
      .owner()
      .context("missing owner information")?;
    match owner {
      Owner::Shared { initial_shared_version } => Ok(initial_shared_version),
      _ => anyhow::bail!("`TransferProposal` is not a shared object"),
    }
  }

  pub async fn accept<S>(&mut self, gas_budget: u64, client: &IdentityClient<S>) -> Result<(), Error>
  where
    S: Signer<IotaKeySignature>,
  {
    if self.done {
      return Err(Error::TransactionBuildingFailed(
        "the transfer has already been concluded".to_owned(),
      ));
    }

    let cap = self.get_cap("RecipientCap", client).await?;
    let (asset_ref, param_type) = self
      .asset_metadata(client)
      .await
      .map_err(|e| Error::ObjectLookup(e.to_string()))?;
    let initial_shared_version = self
      .initial_shared_version(client)
      .await
      .map_err(|e| Error::ObjectLookup(e.to_string()))?;
    let tx = move_calls::asset::accept_proposal(
      (self.id(), initial_shared_version),
      cap,
      asset_ref,
      param_type,
      client.package_id(),
    )?;
    let response = client.execute_transaction(tx, gas_budget).await?;

    if response.errors.is_empty() {
      self.done = true;
      Ok(())
    } else {
      let err_str = response.errors.join("; ");
      Err(Error::TransactionUnexpectedResponse(err_str))
    }
  }

  pub async fn conclude_or_cancel<S>(self, gas_budget: u64, client: &IdentityClient<S>) -> Result<(), Error>
  where
    S: Signer<IotaKeySignature>,
  {
    let cap = self.get_cap("SenderCap", client).await?;
    let (asset_ref, param_type) = self
      .asset_metadata(client)
      .await
      .map_err(|e| Error::ObjectLookup(e.to_string()))?;
    let initial_shared_version = self
      .initial_shared_version(client)
      .await
      .map_err(|e| Error::ObjectLookup(e.to_string()))?;

    let tx = move_calls::asset::conclude_or_cancel(
      (self.id(), initial_shared_version),
      cap,
      asset_ref,
      param_type,
      client.package_id(),
    )?;
    let response = client.execute_transaction(tx, gas_budget).await?;

    if response.errors.is_empty() {
      Ok(())
    } else {
      let err_str = response.errors.join("; ");
      Err(Error::TransactionUnexpectedResponse(err_str))
    }
  }

  pub fn id(&self) -> ObjectID {
    *self.id.object_id()
  }

  pub fn sender(&self) -> IotaAddress {
    self.sender_address
  }

  pub fn recipient(&self) -> IotaAddress {
    self.recipient_address
  }

  pub fn is_concluded(&self) -> bool {
    self.done
  }
}
