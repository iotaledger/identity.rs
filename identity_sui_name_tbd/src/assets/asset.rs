use std::str::FromStr as _;

use crate::client::IdentityClient;
use crate::client::IotaKeySignature;
use crate::sui::move_calls;
use crate::transaction::Transaction;
use crate::utils::MoveType;
use crate::Error;
use anyhow::anyhow;
use anyhow::Context;
use async_trait::async_trait;
use iota_sdk::rpc_types::IotaData as _;
use iota_sdk::rpc_types::IotaExecutionStatus;
use iota_sdk::rpc_types::IotaObjectDataOptions;
use iota_sdk::rpc_types::IotaTransactionBlockEffectsAPI as _;
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::base_types::SequenceNumber;
use iota_sdk::types::id::UID;
use iota_sdk::types::object::Owner;
use iota_sdk::types::TypeTag;
use iota_sdk::IotaClient;
use move_core_types::ident_str;
use move_core_types::language_storage::StructTag;
use secret_storage::Signer;
use serde::de::DeserializeOwned;
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

  pub fn transfer(self, recipient: IotaAddress) -> Result<TransferAssetTx<T>, Error> {
    if !self.transferable {
      return Err(Error::InvalidConfig(format!(
        "`AuthenticatedAsset` {} is not transferable",
        self.id()
      )));
    }
    Ok(TransferAssetTx { asset: self, recipient })
  }

  pub fn delete(self) -> Result<DeleteAssetTx<T>, Error> {
    if !self.deletable {
      return Err(Error::InvalidConfig(format!(
        "`AuthenticatedAsset` {} cannot be deleted",
        self.id()
      )));
    }

    Ok(DeleteAssetTx(self))
  }
  pub fn set_content(&mut self, new_content: T) -> Result<UpdateContentTx<'_, T>, Error> {
    if !self.mutable {
      return Err(Error::InvalidConfig(format!(
        "`AuthenticatedAsset` {} is immutable",
        self.id()
      )));
    }

    Ok(UpdateContentTx {
      asset: self,
      new_content,
    })
  }
}

#[derive(Debug)]
pub struct AuthenticatedAssetBuilder<T> {
  inner: T,
  mutable: bool,
  transferable: bool,
  deletable: bool,
}

impl<T: MoveType> MoveType for AuthenticatedAsset<T> {
  fn move_type(package: ObjectID) -> TypeTag {
    TypeTag::Struct(Box::new(StructTag {
      address: package.into(),
      module: ident_str!("asset").into(),
      name: ident_str!("AuthenticatedAsset").into(),
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

  pub fn finish(self) -> CreateAssetTx<T> {
    CreateAssetTx(self)
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
      module: ident_str!("asset").into(),
      name: ident_str!("TransferProposal").into(),
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

  pub fn accept(self) -> AcceptTransferTx {
    AcceptTransferTx(self)
  }

  pub fn conclude_or_cancel(self) -> ConcludeTransferTx {
    ConcludeTransferTx(self)
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

#[derive(Debug)]
pub struct UpdateContentTx<'a, T> {
  asset: &'a mut AuthenticatedAsset<T>,
  new_content: T,
}

#[async_trait]
impl<'a, T> Transaction for UpdateContentTx<'a, T>
where
  T: MoveType + Serialize + Clone + Send + Sync,
{
  type Output = ();

  async fn execute_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<Self::Output, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let tx = move_calls::asset::update(
      self.asset.object_ref(client).await?,
      self.new_content.clone(),
      client.package_id(),
    )?;
    let tx_status = client
      .execute_transaction(tx, gas_budget)
      .await?
      .effects
      .context("transaction had no effects")
      .map(|effects| effects.into_status())
      .map_err(|e| Error::TransactionUnexpectedResponse(e.to_string()))?;
    if let IotaExecutionStatus::Failure { error } = tx_status {
      return Err(Error::TransactionUnexpectedResponse(error));
    }
    self.asset.inner = self.new_content;
    Ok(())
  }
}

#[derive(Debug)]
pub struct DeleteAssetTx<T>(AuthenticatedAsset<T>);

#[async_trait]
impl<T> Transaction for DeleteAssetTx<T>
where
  T: MoveType + Send + Sync,
{
  type Output = ();

  async fn execute_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<Self::Output, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let asset_ref = self.0.object_ref(client).await?;
    let tx = move_calls::asset::delete::<T>(asset_ref, client.package_id())?;

    client.execute_transaction(tx, gas_budget).await?;
    Ok(())
  }
}
#[derive(Debug)]
pub struct CreateAssetTx<T>(AuthenticatedAssetBuilder<T>);

#[async_trait]
impl<T> Transaction for CreateAssetTx<T>
where
  T: MoveType + Serialize + DeserializeOwned + Send,
{
  type Output = AuthenticatedAsset<T>;

  async fn execute_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<Self::Output, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let AuthenticatedAssetBuilder {
      inner,
      mutable,
      transferable,
      deletable,
    } = self.0;
    let tx = move_calls::asset::new(inner, mutable, transferable, deletable, client.package_id())?;

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

#[derive(Debug)]
pub struct TransferAssetTx<T> {
  asset: AuthenticatedAsset<T>,
  recipient: IotaAddress,
}

#[async_trait]
impl<T> Transaction for TransferAssetTx<T>
where
  T: MoveType + Send + Sync,
{
  type Output = TransferProposal;

  async fn execute_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<Self::Output, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let tx = move_calls::asset::transfer::<T>(
      self.asset.object_ref(client).await?,
      self.recipient,
      client.package_id(),
    )?;
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
}

#[derive(Debug)]
pub struct AcceptTransferTx(TransferProposal);

#[async_trait]
impl Transaction for AcceptTransferTx {
  type Output = ();
  async fn execute_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<Self::Output, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    if self.0.done {
      return Err(Error::TransactionBuildingFailed(
        "the transfer has already been concluded".to_owned(),
      ));
    }

    let cap = self.0.get_cap("RecipientCap", client).await?;
    let (asset_ref, param_type) = self
      .0
      .asset_metadata(client)
      .await
      .map_err(|e| Error::ObjectLookup(e.to_string()))?;
    let initial_shared_version = self
      .0
      .initial_shared_version(client)
      .await
      .map_err(|e| Error::ObjectLookup(e.to_string()))?;
    let tx = move_calls::asset::accept_proposal(
      (self.0.id(), initial_shared_version),
      cap,
      asset_ref,
      param_type,
      client.package_id(),
    )?;

    client.execute_transaction(tx, gas_budget).await?;
    Ok(())
  }
}

#[derive(Debug)]
pub struct ConcludeTransferTx(TransferProposal);

#[async_trait]
impl Transaction for ConcludeTransferTx {
  type Output = ();
  async fn execute_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<Self::Output, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let cap = self.0.get_cap("SenderCap", client).await?;
    let (asset_ref, param_type) = self
      .0
      .asset_metadata(client)
      .await
      .map_err(|e| Error::ObjectLookup(e.to_string()))?;
    let initial_shared_version = self
      .0
      .initial_shared_version(client)
      .await
      .map_err(|e| Error::ObjectLookup(e.to_string()))?;

    let tx = move_calls::asset::conclude_or_cancel(
      (self.0.id(), initial_shared_version),
      cap,
      asset_ref,
      param_type,
      client.package_id(),
    )?;

    client.execute_transaction(tx, gas_budget).await?;
    Ok(())
  }
}
