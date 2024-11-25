// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use iota_sdk::rpc_types::IotaExecutionStatus;
use iota_sdk::rpc_types::IotaObjectDataOptions;
use iota_sdk::rpc_types::IotaTransactionBlockEffectsAPI;
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::id::UID;
use iota_sdk::types::transaction::ProgrammableTransaction;
use iota_sdk::types::TypeTag;
use iota_sdk::types::STARDUST_PACKAGE_ID;
use secret_storage::Signer;
use serde;
use serde::Deserialize;
use serde::Serialize;

use crate::rebased::client::IdentityClient;
use crate::rebased::client::IdentityClientReadOnly;
use crate::rebased::client::IotaKeySignature;
use crate::rebased::sui::move_calls;
use crate::rebased::transaction::Transaction;
use crate::rebased::transaction::TransactionOutput;
use crate::rebased::utils::MoveType;
use crate::rebased::Error;

use super::get_identity;
use super::Identity;
use super::OnChainIdentity;

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

impl UnmigratedAlias {
  /// Returns a transaction that when executed migrates a legacy `Alias`
  /// containing a DID Document to a new [`OnChainIdentity`].
  pub async fn migrate(
    self,
    client: &IdentityClientReadOnly,
  ) -> Result<impl Transaction<Output = OnChainIdentity>, Error> {
    // Try to parse a StateMetadataDocument out of this alias.
    let identity = Identity::Legacy(self);
    let did_doc = identity.did_document(client)?;
    let Identity::Legacy(alias) = identity else {
      unreachable!("alias was wrapped by us")
    };
    // Get the ID of the `AliasOutput` that owns this `Alias`.
    let dynamic_field_wrapper = client
      .read_api()
      .get_object_with_options(*alias.id.object_id(), IotaObjectDataOptions::new().with_owner())
      .await
      .map_err(|e| Error::RpcError(e.to_string()))?
      .owner()
      .expect("owner was requested")
      .get_owner_address()
      .expect("alias is a dynamic field")
      .into();
    let alias_output_id = client
      .read_api()
      .get_object_with_options(dynamic_field_wrapper, IotaObjectDataOptions::new().with_owner())
      .await
      .map_err(|e| Error::RpcError(e.to_string()))?
      .owner()
      .expect("owner was requested")
      .get_owner_address()
      .expect("alias is owned by an alias_output")
      .into();
    // Get alias_output's ref.
    let alias_output_ref = client
      .read_api()
      .get_object_with_options(alias_output_id, IotaObjectDataOptions::default())
      .await
      .map_err(|e| Error::RpcError(e.to_string()))?
      .object_ref_if_exists()
      .expect("alias_output exists");
    // Get migration registry ref.
    let migration_registry_ref = client
      .get_object_ref_by_id(client.migration_registry_id())
      .await?
      .expect("migration registry exists");

    // Extract creation metadata
    let created = did_doc
      .metadata
      .created
      // `to_unix` returns the seconds since EPOCH; we need milliseconds.
      .map(|timestamp| timestamp.to_unix() as u64 * 1000);

    // Build migration tx.
    let tx =
      move_calls::migration::migrate_did_output(alias_output_ref, created, migration_registry_ref, client.package_id())
        .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    Ok(MigrateLegacyAliasTx(tx))
  }
}

#[derive(Debug)]
struct MigrateLegacyAliasTx(ProgrammableTransaction);

#[async_trait]
impl Transaction for MigrateLegacyAliasTx {
  type Output = OnChainIdentity;
  async fn execute_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutput<Self::Output>, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let response = self.0.execute_with_opt_gas(gas_budget, client).await?.response;
    // Make sure the tx was successful.
    let effects = response
      .effects
      .as_ref()
      .ok_or_else(|| Error::TransactionUnexpectedResponse("transaction had no effects".to_string()))?;
    if let IotaExecutionStatus::Failure { error } = effects.status() {
      Err(Error::TransactionUnexpectedResponse(error.to_string()))
    } else {
      let identity_ref = effects
        .created()
        .iter()
        .find(|obj_ref| obj_ref.owner.is_shared())
        .ok_or_else(|| {
          Error::TransactionUnexpectedResponse("Identity not found in transaction's results".to_string())
        })?;

      get_identity(client, identity_ref.object_id())
        .await
        .map(move |identity| TransactionOutput {
          output: identity.expect("identity exists on-chain"),
          response,
        })
    }
  }
}
