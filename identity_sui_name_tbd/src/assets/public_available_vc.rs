use std::ops::Deref;
use std::str::FromStr;

use anyhow::Context as _;
use identity_credential::credential::Credential;
use identity_credential::credential::Jwt;
use identity_credential::credential::JwtCredential;
use identity_jose::jwt::JwtHeader;
use identity_jose::jwu;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::Argument;
use iota_sdk::types::transaction::Command;
use iota_sdk::types::transaction::ProgrammableMoveCall;
use iota_sdk::types::TypeTag;
use itertools::Itertools;
use move_core_types::ident_str;
use secret_storage::Signer;
use serde::Deserialize;
use serde::Serialize;

use crate::client::IdentityClient;
use crate::client::IdentityClientReadOnly;
use crate::client::IotaKeySignature;
use crate::transaction::Transaction;
use crate::utils::MoveType;
use crate::Error;

use super::AuthenticatedAsset;
use super::AuthenticatedAssetBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IotaVerifiableCredential {
  data: Vec<u8>,
}

impl MoveType for IotaVerifiableCredential {
  fn move_type(package: ObjectID) -> TypeTag {
    TypeTag::from_str(&format!("{package}::public_vc::PublicVc")).expect("valid utf8")
  }

  fn try_to_argument(
    &self,
    ptb: &mut ProgrammableTransactionBuilder,
    package: Option<ObjectID>,
  ) -> Result<Argument, Error> {
    let values = ptb
      .pure(&self.data)
      .map_err(|e| Error::InvalidArgument(e.to_string()))?;
    Ok(ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
      package: package.ok_or_else(|| Error::InvalidArgument("missing package ID".to_string()))?,
      module: ident_str!("public_vc").into(),
      function: ident_str!("new").into(),
      type_arguments: vec![],
      arguments: vec![values],
    }))))
  }
}

#[derive(Debug, Clone)]
pub struct PublicAvailableVC {
  asset: AuthenticatedAsset<IotaVerifiableCredential>,
  credential: Credential,
}

impl Deref for PublicAvailableVC {
  type Target = Credential;
  fn deref(&self) -> &Self::Target {
    &self.credential
  }
}

impl PublicAvailableVC {
  pub fn object_id(&self) -> ObjectID {
    self.asset.id()
  }

  pub fn jwt(&self) -> Jwt {
    String::from_utf8(self.asset.content().data.clone())
      .map(Jwt::new)
      .expect("JWT is valid UTF8")
  }

  pub async fn new<S>(jwt: Jwt, gas_budget: Option<u64>, client: &IdentityClient<S>) -> Result<Self, anyhow::Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let jwt_bytes = String::from(jwt).into_bytes();
    let credential = parse_jwt_credential(&jwt_bytes)?;
    let asset = AuthenticatedAssetBuilder::new(IotaVerifiableCredential { data: jwt_bytes })
      .transferable(false)
      .mutable(true)
      .deletable(true)
      .finish()
      .execute_with_opt_gas(gas_budget, client)
      .await?;

    Ok(Self { credential, asset })
  }

  pub async fn get_by_id(id: ObjectID, client: &IdentityClientReadOnly) -> Result<Self, crate::Error> {
    let asset = client
      .get_object_by_id::<AuthenticatedAsset<IotaVerifiableCredential>>(id)
      .await?;
    Self::try_from_asset(asset).map_err(|e| {
      crate::Error::ObjectLookup(format!(
        "object at address {id} is not a valid publicly available VC: {e}"
      ))
    })
  }

  fn try_from_asset(asset: AuthenticatedAsset<IotaVerifiableCredential>) -> Result<Self, anyhow::Error> {
    let credential = parse_jwt_credential(&asset.content().data)?;
    Ok(Self { asset, credential })
  }
}

fn parse_jwt_credential(bytes: &[u8]) -> Result<Credential, anyhow::Error> {
  let [header, payload, _signature]: [Vec<u8>; 3] = bytes
    .split(|c| *c == b'.')
    .map(jwu::decode_b64)
    .try_collect::<_, Vec<_>, _>()?
    .try_into()
    .map_err(|_| anyhow::anyhow!("invalid JWT"))?;
  let _header = serde_json::from_slice::<JwtHeader>(&header)?;
  let credential_claims = serde_json::from_slice::<JwtCredential>(&payload)?;
  credential_claims.try_into().context("invalid jwt credential claims")
}
