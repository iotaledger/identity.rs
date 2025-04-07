// Copyright 2020-2025 IOTA Stiftung, Fondazione Links
// SPDX-License-Identifier: Apache-2.0

use examples::create_did_document;
use examples::get_funded_client;

use anyhow::Context as _;
use identity_iota_core::rebased::client::IdentityClient;
use identity_iota_core::rebased::transaction_builder::MutGasDataRef;
use identity_iota_core::rebased::transaction_builder::Transaction as _;
use identity_iota_core::IotaDocument;
use identity_iota_interaction::IotaClientTrait as _;
use identity_iota_interaction::IotaKeySignature;
use iota_sdk::rpc_types::IotaTransactionBlockResponseOptions;
use iota_sdk::types::crypto::Signature;
use iota_sdk::types::quorum_driver_types::ExecuteTransactionRequestType;
use iota_sdk::types::transaction::GasData;
use iota_sdk::types::transaction::Transaction;
use iota_sdk::IotaClientBuilder;
use iota_sdk::IOTA_COIN_TYPE;
use secret_storage::Signer;

/// This example demonstrates:
/// 1. A user - Alice - can build a transaction that is sponsored by another user - Bob;
/// 2. Deconstruct the transaction into its parts, to execute it manually through the SDK's IotaClient;
/// 3. Apply the transaction's off-chain effects, from its on-chain ones.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let alice_client = get_funded_client().await?;
  let bob_client = get_funded_client().await?;

  // Alice wants to create a new Identity with only her as its controller.
  let (tx_data, sigs, tx) = alice_client
    .create_identity(IotaDocument::new(test_client.network()))
    .finish()
    // Alice is the sender of this transaction
    .with_sender(alice_client.sender_address())
    // but Bob will provide Gas for it - i.e. he'll sponsor it.
    .with_sponsor(&alice_client, async |tx_data| {
      bob_sponsor_fn(tx_data, &bob_client).await
    })
    .await?
    .build(&alice_client)
    .await?;

  // A new IotaClient is created to execute the transaction from its parts.
  let iota_client = IotaClientBuilder::default().build_localnet().await?;
  let tx_response = iota_client
    .quorum_driver_api()
    .execute_transaction_block(
      Transaction::from_data(tx_data, sigs),
      IotaTransactionBlockResponseOptions::full_content(),
      ExecuteTransactionRequestType::WaitForLocalExecution,
    )
    .await?;
  let tx_effects = tx_response.effects.as_ref().expect("transaction had effects");
  // Alice's Identity is parsed out of the transaction's effects!
  let _identity = tx.apply(tx_effects, &alice_client).await?;

  Ok(())
}

async fn bob_sponsor_fn<S>(mut tx_data: MutGasDataRef<'_>, client: &IdentityClient<S>) -> anyhow::Result<Signature>
where
  S: Signer<IotaKeySignature> + Sync,
{
  let coin_ref = client
    .coin_read_api()
    .get_coins(client.sender_address(), Some(IOTA_COIN_TYPE.to_owned()), None, None)
    .await?
    .data
    .first()
    .expect("should have at least 1 coin")
    .object_ref();
  let gas_data = GasData {
    price: 1000,
    payment: vec![coin_ref],
    owner: client.sender_address(),
    budget: TEST_GAS_BUDGET,
  };

  *tx_data.gas_data_mut() = gas_data;

  client
    .signer()
    .sign(&tx_data)
    .await
    .context("failed to sign transaction tx data")
}