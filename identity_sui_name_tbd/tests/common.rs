use std::io::Write;
use std::ops::Deref;

use anyhow::anyhow;
use anyhow::Context;
use identity_sui_name_tbd::migration;
use jsonpath_rust::JsonPathQuery;
use serde_json::Value;
use sui_sdk::rpc_types::SuiObjectDataOptions;
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::SuiClient;
use sui_sdk::SuiClientBuilder;
use tokio::process::Command;
use tokio::sync::OnceCell;

static CLIENT: OnceCell<TestClient> = OnceCell::const_new();
const SCRIPT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/scripts/");
const CHACHED_PKG_ID: &str = "/tmp/identity_iota_pkg_id.txt";

pub async fn get_client() -> anyhow::Result<TestClient> {
  CLIENT.get_or_try_init(init).await.cloned()
}

async fn init() -> anyhow::Result<TestClient> {
  let client = SuiClientBuilder::default().build_localnet().await?;
  let address = get_active_address().await?;

  let package_id = if let Some(id) = std::env::var("IDENTITY_IOTA_PKG_ID")
    .or(std::fs::read_to_string(CHACHED_PKG_ID))
    .ok()
  {
    std::env::set_var("IDENTITY_IOTA_PKG_ID", id.clone());
    id.parse()?
  } else {
    publish_package().await?
  };

  Ok(TestClient {
    client,
    package_id,
    address,
  })
}

async fn get_active_address() -> anyhow::Result<SuiAddress> {
  Command::new("sui")
    .arg("client")
    .arg("active-address")
    .arg("--json")
    .output()
    .await
    .context("Failed to execute command")
    .and_then(|output| Ok(serde_json::from_slice::<SuiAddress>(&output.stdout)?))
}

async fn publish_package() -> anyhow::Result<ObjectID> {
  let output = Command::new("sh")
    .current_dir(SCRIPT_DIR)
    .arg("publish_identity_package.sh")
    .output()
    .await?;

  if !output.status.success() {
    anyhow::bail!(
      "Failed to publish move package: {}",
      std::str::from_utf8(&output.stderr).unwrap()
    );
  }

  let publish_result = {
    let output_str = std::str::from_utf8(&output.stdout).unwrap();
    let start_of_json = output_str.find('{').ok_or(anyhow!("No json in output"))?;
    serde_json::from_str::<Value>(output_str[start_of_json..].trim())?
  };

  let package_id = publish_result
    .path("$.objectChanges[?(@.type == 'published')].packageId")
    .map_err(|e| anyhow!("Failed to parse JSONPath: {e}"))
    .and_then(|value| Ok(serde_json::from_value::<Vec<ObjectID>>(value)?))?
    .first()
    .copied()
    .ok_or_else(|| anyhow!("Failed to parse package ID after publishing"))?;

  // Persist package ID in order to avoid publishing the package for every test.
  std::env::set_var("IDENTITY_IOTA_PKG_ID", package_id.to_string());
  let mut file = std::fs::File::create(CHACHED_PKG_ID)?;
  file.write_all(package_id.to_string().as_bytes())?;

  Ok(package_id)
}

#[derive(Clone)]
pub struct TestClient {
  client: SuiClient,
  package_id: ObjectID,
  #[allow(dead_code)]
  address: SuiAddress,
}

impl Deref for TestClient {
  type Target = SuiClient;
  fn deref(&self) -> &Self::Target {
    &self.client
  }
}

impl TestClient {
  /// Creates a new DID document inside a stardust alias output.
  /// Returns alias's ID.
  pub async fn create_legacy_did(&self) -> anyhow::Result<ObjectID> {
    let output = Command::new("sh")
      .current_dir(SCRIPT_DIR)
      .arg("create_test_alias_output.sh")
      .arg(self.package_id.to_string())
      .output()
      .await?;

    if !output.status.success() {
      anyhow::bail!(
        "Failed to create alias output: {}",
        std::str::from_utf8(&output.stderr).unwrap()
      );
    }

    let result = {
      let output_str = std::str::from_utf8(&output.stdout).unwrap();
      let start_of_json = output_str.find('{').ok_or(anyhow!("No json in output"))?;
      serde_json::from_str::<Value>(output_str[start_of_json..].trim())?
    };

    result
      .path("$.objectChanges[?(@.type == 'created' && @.objectType ~= '.*Alias$')].objectId")
      .map_err(|e| anyhow!("Failed to parse JSONPath: {e}"))
      .and_then(|value| Ok(serde_json::from_value::<Vec<ObjectID>>(value)?))?
      .first()
      .copied()
      .ok_or_else(|| anyhow!("No `AliasOutput` object was created"))
  }

  /// Migrates a legacy DID document into the new Iota 3.0 DID document format.
  /// `alias_id` is the ID of the Alias move object - i.e. the same as stardust's AliasID.
  /// Returns the tuple `(document's ID, ControllerCapability's ID)`.
  pub async fn migrate_legacy_did(&self, alias_id: ObjectID) -> anyhow::Result<(ObjectID, ObjectID)> {
    // Find the ID of the AliasOutput that owns `alias_id`.
    let alias_output_id = {
      let dynamic_field_id = self
        .read_api()
        .get_object_with_options(alias_id, SuiObjectDataOptions::default().with_owner())
        .await?
        .data
        .context("Failed to read object with ID {alias_id}")?
        .owner
        .context("No owner informat provided for object {alias_id}")?
        .get_owner_address()
        .map(ObjectID::from)?;

      self
        .read_api()
        .get_object_with_options(dynamic_field_id, SuiObjectDataOptions::default().with_owner())
        .await?
        .data
        .context("Failed to read object with ID {alias_id}")?
        .owner
        .context("No owner informat provided for object {alias_id}")?
        .get_owner_address()
        .map(ObjectID::from)?
    };

    // Call migration script.
    let output = Command::new("sh")
      .current_dir(SCRIPT_DIR)
      .arg("migrate_alias_output.sh")
      .arg(self.package_id.to_string())
      .arg(alias_output_id.to_string())
      .arg(migration::migration_registry_id(&self).await?.to_string())
      .output()
      .await?;

    let result = {
      let output_str = std::str::from_utf8(&output.stdout).unwrap();
      let start_of_json = output_str.find('{').ok_or(anyhow!("No json in output"))?;
      serde_json::from_str::<Value>(output_str[start_of_json..].trim())?
    };

    let document_id = result
      .clone()
      .path("$.objectChanges[?(@.type == 'created' && @.objectType ~= '.*Identity$')].objectId")
      .map_err(|e| anyhow!("Failed to parse JSONPath: {e}"))
      .and_then(|value| Ok(serde_json::from_value::<Vec<ObjectID>>(value)?))?
      .first()
      .copied()
      .ok_or_else(|| anyhow!("no Document in transaction's result"))?;
    let capability_id = result
      .path("$.objectChanges[?(@.type == 'created' && @.objectType ~= '.*ControllerCap$')].objectId")
      .map_err(|e| anyhow!("Failed to parse JSONPath: {e}"))
      .and_then(|value| Ok(serde_json::from_value::<Vec<ObjectID>>(value)?))?
      .first()
      .copied()
      .ok_or_else(|| anyhow!("no ControllerCapability in transaction's result"))?;

    Ok((document_id, capability_id))
  }
}
