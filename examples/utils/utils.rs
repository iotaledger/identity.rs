// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use anyhow::Context;

use identity_iota::crypto::KeyPair;
use identity_iota::crypto::KeyType;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::IotaVerificationMethod;
use identity_iota::iota::NetworkName;
use identity_iota::verification::MethodScope;

use iota_client::block::address::Address;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::Output;
use iota_client::crypto::keys::bip39;
use iota_client::node_api::indexer::query_parameters::QueryParameter;
use iota_client::secret::SecretManager;
use iota_client::Client;
use rand::distributions::DistString;

pub static API_ENDPOINT: &str = "https://api.testnet.shimmer.network/";
pub static FAUCET_ENDPOINT: &str = "http://localhost:8091/api/enqueue";

/// Creates a DID Document and publishes it in a new Alias Output.
///
/// Its functionality is equivalent to the "create DID" example
/// and exists for convenient calling from the other examples.
pub async fn create_did(
  client: &Client,
  secret_manager: &mut SecretManager,
) -> anyhow::Result<(Address, IotaDocument, KeyPair)> {
  let address: Address = get_address_with_funds(client, secret_manager, FAUCET_ENDPOINT)
    .await
    .context("failed to get address with funds")?;

  let network_name: NetworkName = client.network_name().await?;

  let (document, key_pair): (IotaDocument, KeyPair) = create_did_document(&network_name)?;

  let alias_output: AliasOutput = client.new_did_output(address, document, None).await?;

  let document: IotaDocument = client.publish_did_output(secret_manager, alias_output).await?;

  Ok((address, document, key_pair))
}

/// Creates an example DID document with the given `network_name`.
///
/// Its functionality is equivalent to the "create DID" example
/// and exists for convenient calling from the other examples.
pub fn create_did_document(network_name: &NetworkName) -> anyhow::Result<(IotaDocument, KeyPair)> {
  let mut document: IotaDocument = IotaDocument::new(network_name);

  let key_pair: KeyPair = KeyPair::new(KeyType::Ed25519)?;

  let method: IotaVerificationMethod =
    IotaVerificationMethod::new(document.id().clone(), key_pair.type_(), key_pair.public(), "#key-1")?;

  document.insert_method(method, MethodScope::VerificationMethod)?;

  Ok((document, key_pair))
}

/// Generates an address from the given [`SecretManager`] and adds funds from the faucet.
pub async fn get_address_with_funds(
  client: &Client,
  stronghold: &mut SecretManager,
  faucet_endpoint: &str,
) -> anyhow::Result<Address> {
  let address: Address = get_address(client, stronghold).await?;

  request_faucet_funds(
    client,
    address,
    client.get_bech32_hrp().await?.as_str(),
    faucet_endpoint,
  )
  .await
  .context("failed to request faucet funds")?;

  Ok(address)
}

/// Initializes the [`SecretManager`] with a new mnemonic, if necessary,
/// and generates an address from the given [`SecretManager`].
pub async fn get_address(client: &Client, secret_manager: &mut SecretManager) -> anyhow::Result<Address> {
  let keypair = KeyPair::new(KeyType::Ed25519)?;
  let mnemonic =
    iota_client::crypto::keys::bip39::wordlist::encode(keypair.private().as_ref(), &bip39::wordlist::ENGLISH)
      .map_err(|err| anyhow::anyhow!(format!("{err:?}")))?;

  if let SecretManager::Stronghold(ref mut stronghold) = secret_manager {
    match stronghold.store_mnemonic(mnemonic).await {
      Ok(()) => (),
      Err(iota_client::Error::StrongholdMnemonicAlreadyStored) => (),
      Err(err) => anyhow::bail!(err),
    }
  } else {
    anyhow::bail!("expected a `StrongholdSecretManager`");
  }

  let address = client.get_addresses(secret_manager).with_range(0..1).get_raw().await?[0];

  Ok(address)
}

/// Requests funds from the faucet for the given `address`.
async fn request_faucet_funds(
  client: &Client,
  address: Address,
  network_hrp: &str,
  faucet_endpoint: &str,
) -> anyhow::Result<()> {
  let address_bech32 = address.to_bech32(network_hrp);

  iota_client::request_funds_from_faucet(faucet_endpoint, &address_bech32).await?;

  tokio::time::timeout(std::time::Duration::from_secs(45), async {
    loop {
      tokio::time::sleep(std::time::Duration::from_secs(5)).await;

      let balance = get_address_balance(client, &address_bech32)
        .await
        .context("failed to get address balance")?;
      if balance > 0 {
        break;
      }
    }
    Ok::<(), anyhow::Error>(())
  })
  .await
  .context("maximum timeout exceeded")??;

  Ok(())
}

/// Returns the balance of the given Bech32-encoded `address`.
async fn get_address_balance(client: &Client, address: &str) -> anyhow::Result<u64> {
  let output_ids = client
    .basic_output_ids(vec![
      QueryParameter::Address(address.to_owned()),
      QueryParameter::HasExpiration(false),
      QueryParameter::HasTimelock(false),
      QueryParameter::HasStorageDepositReturn(false),
    ])
    .await?;

  let outputs_responses = client.get_outputs(output_ids).await?;

  let mut total_amount = 0;
  for output_response in outputs_responses {
    let output = Output::try_from_dto(&output_response.output, client.get_token_supply().await?)?;
    total_amount += output.amount();
  }

  Ok(total_amount)
}

/// Creates a random stronghold path in the temporary directory, whose exact location is OS-dependent.
pub fn random_stronghold_path() -> PathBuf {
  let mut file = std::env::temp_dir();
  file.push("test_strongholds");
  file.push(rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 32));
  file.set_extension("stronghold");
  file.to_owned()
}
