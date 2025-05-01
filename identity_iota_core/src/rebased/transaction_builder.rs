// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use anyhow::Context as _;
use async_trait::async_trait;
use cfg_if::cfg_if;
use identity_iota_interaction::rpc_types::IotaTransactionBlockEffects;
use identity_iota_interaction::rpc_types::IotaTransactionBlockEffectsAPI as _;
use identity_iota_interaction::rpc_types::IotaTransactionBlockResponseOptions;
use identity_iota_interaction::shared_crypto::intent::Intent;
use identity_iota_interaction::shared_crypto::intent::IntentMessage;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::crypto::IotaSignature as _;
use identity_iota_interaction::types::crypto::PublicKey;
use identity_iota_interaction::types::crypto::Signature;
use identity_iota_interaction::types::quorum_driver_types::ExecuteTransactionRequestType;
use identity_iota_interaction::types::transaction::GasData;
use identity_iota_interaction::types::transaction::ProgrammableTransaction;
use identity_iota_interaction::types::transaction::TransactionData;
use identity_iota_interaction::types::transaction::TransactionDataAPI as _;
use identity_iota_interaction::types::transaction::TransactionKind;
use identity_iota_interaction::IotaClientTrait;
use identity_iota_interaction::IotaKeySignature;
use identity_iota_interaction::OptionalSync;
use itertools::Itertools;
use secret_storage::Signer;

use super::client::CoreClient;
use super::client::CoreClientReadOnly;
#[cfg(not(target_arch = "wasm32"))]
use super::transaction::TransactionOutput;
#[cfg(target_arch = "wasm32")]
use super::transaction::TransactionOutputInternal as TransactionOutput;
use super::Error;

/// An operation that combines a transaction with its off-chain effects.
#[cfg_attr(feature = "send-sync", async_trait)]
#[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
pub trait Transaction {
  /// Output type for this transaction.
  type Output;

  /// Encode this operation into a [ProgrammableTransaction].
  async fn build_programmable_transaction<C>(&self, client: &C) -> Result<ProgrammableTransaction, Error>
  where
    C: CoreClientReadOnly + OptionalSync;

  /// Parses a transaction result in order to compute its effects.
  /// ## Notes
  /// [Transaction::apply] implementations should make sure to properly consume
  /// the parts of `effects` that are needed for the transaction - e.g., removing
  /// the ID of the object the transaction created from the `effects`'s list of
  /// created objects.
  /// This is particularly important to enable the batching of transactions.
  async fn apply<C>(
    self,
    effects: IotaTransactionBlockEffects,
    client: &C,
  ) -> (Result<Self::Output, Error>, IotaTransactionBlockEffects)
  where
    C: CoreClientReadOnly + OptionalSync;
}

#[derive(Debug, Default, Clone)]
struct PartialGasData {
  payment: Vec<ObjectRef>,
  owner: Option<IotaAddress>,
  price: Option<u64>,
  budget: Option<u64>,
}

impl From<GasData> for PartialGasData {
  fn from(value: GasData) -> Self {
    Self {
      payment: value.payment,
      owner: Some(value.owner),
      price: Some(value.price),
      budget: Some(value.budget),
    }
  }
}

impl PartialGasData {
  fn into_gas_data_with_defaults(self) -> GasData {
    GasData {
      payment: self.payment,
      owner: self.owner.unwrap_or_default(),
      price: self.price.unwrap_or_default(),
      budget: self.budget.unwrap_or_default(),
    }
  }
}

impl TryFrom<PartialGasData> for GasData {
  type Error = Error;
  fn try_from(value: PartialGasData) -> Result<Self, Self::Error> {
    let owner = value
      .owner
      .ok_or_else(|| Error::GasIssue("missing gas owner".to_owned()))?;
    let price = value
      .price
      .ok_or_else(|| Error::GasIssue("missing gas price".to_owned()))?;
    let budget = value
      .budget
      .ok_or_else(|| Error::GasIssue("missing gas budget".to_owned()))?;

    Ok(GasData {
      payment: value.payment,
      owner,
      price,
      budget,
    })
  }
}

/// A reference to [TransactionData] that only allows to mutate its [GasData].
#[derive(Debug)]
pub struct MutGasDataRef<'tx>(&'tx mut TransactionData);
impl Deref for MutGasDataRef<'_> {
  type Target = TransactionData;
  fn deref(&self) -> &Self::Target {
    self.0
  }
}

impl MutGasDataRef<'_> {
  /// Returns a mutable reference to [GasData].
  pub fn gas_data_mut(&mut self) -> &mut GasData {
    self.0.gas_data_mut()
  }
}

/// Builds an executable transaction on a step by step manner.
#[derive(Debug)]
pub struct TransactionBuilder<Tx> {
  programmable_tx: Option<ProgrammableTransaction>,
  sender: Option<IotaAddress>,
  gas: PartialGasData,
  signatures: Vec<Signature>,
  tx: Tx,
}

impl<Tx> AsRef<Tx> for TransactionBuilder<Tx> {
  fn as_ref(&self) -> &Tx {
    &self.tx
  }
}

impl<Tx> TransactionBuilder<Tx>
where
  Tx: Transaction,
{
  /// Starts the creation of an executable transaction by supplying
  /// a type implementing [Transaction].
  pub fn new(effect: Tx) -> Self {
    Self {
      tx: effect,
      gas: PartialGasData::default(),
      signatures: vec![],
      sender: None,
      programmable_tx: None,
    }
  }

  async fn transaction_data<C>(&mut self, client: &C) -> anyhow::Result<TransactionData>
  where
    C: CoreClientReadOnly + OptionalSync,
  {
    // Make sure the partial gas information is actually complete to create a whole GasData.
    let gas_data: GasData = std::mem::take(&mut self.gas).try_into()?;
    self.gas = gas_data.into();

    // Forward call to "with_partial_gas" knowing no defaults will be used.
    self.transaction_data_with_partial_gas(client).await
  }

  /// Same as [Self::transaction_data] but will not fail with incomplete gas information.
  /// Missing gas data is filled with default values through [PartialGasData::into_gas_data_with_defaults].
  async fn transaction_data_with_partial_gas<C>(&mut self, client: &C) -> anyhow::Result<TransactionData>
  where
    C: CoreClientReadOnly + OptionalSync,
  {
    let sender = self.sender.context("missing sender")?;
    let gas_data = self.gas.clone().into_gas_data_with_defaults();
    let pt = self.get_or_init_programmable_tx(client).await?.clone();

    Ok(TransactionData::new_with_gas_data(
      TransactionKind::ProgrammableTransaction(pt),
      sender,
      gas_data,
    ))
  }

  /// Adds `signer`'s signature to this this transaction's signatures list.
  /// # Notes
  /// This methods asserts that `signer`'s address matches the address of
  /// either this transaction's sender or the gas owner - failing otherwise.
  pub async fn with_signature<S, C>(mut self, client: &C) -> Result<Self, Error>
  where
    C: CoreClient<S> + OptionalSync,
    S: Signer<IotaKeySignature> + OptionalSync,
  {
    let pk = client
      .signer()
      .public_key()
      .await
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
    let signer_address = IotaAddress::from(&pk);
    let matches_sender = self.sender.is_none_or(|sender| sender == signer_address);
    let matches_gas_owner = self.gas.owner.is_none_or(|owner| owner == signer_address);
    if !(matches_sender || matches_gas_owner) {
      return Err(Error::TransactionBuildingFailed(format!(
        "signer's address {signer_address} doesn't match the address of either the transaction sender or the gas owner"
      )));
    }

    let tx_data = self.transaction_data(client).await?;

    let sig = client
      .signer()
      .sign(&tx_data)
      .await
      .map_err(|e| Error::TransactionSigningFailed(e.to_string()))?;
    self.signatures.push(sig);

    Ok(self)
  }

  /// Attempts to sponsor this transaction by having another party supply [GasData] and gas owner signature.
  /// ## Notes
  /// The [TransactionData] passed to `sponsor_tx` can be constructed from partial gas data; the sponsor is
  /// tasked with setting the gas information appropriately before signing.
  pub async fn with_sponsor<F, C>(mut self, client: &C, sponsor_tx: F) -> Result<Self, Error>
  where
    C: CoreClientReadOnly + OptionalSync,
    F: AsyncFnOnce(MutGasDataRef<'_>) -> anyhow::Result<Signature>,
  {
    let mut tx_data = self.transaction_data_with_partial_gas(client).await?;
    let signature = sponsor_tx(MutGasDataRef(&mut tx_data))
      .await
      .map_err(|e| Error::GasIssue(format!("failed to sponsor transaction: {e}")))?;

    let gas_owner = tx_data.gas_owner();
    let mut intent_msg = IntentMessage::new(Intent::iota_transaction(), tx_data);
    signature
      .verify_secure(&intent_msg, gas_owner, signature.scheme())
      .map_err(|e| Error::TransactionBuildingFailed(format!("invalid sponsor signature: {e}")))?;
    let gas_data = std::mem::replace(
      intent_msg.value.gas_data_mut(),
      GasData {
        payment: vec![],
        owner: IotaAddress::ZERO,
        price: 0,
        budget: 0,
      },
    );

    self.signatures.push(signature);
    self.gas = gas_data.into();

    Ok(self)
  }

  async fn get_or_init_programmable_tx<C>(&mut self, client: &C) -> Result<&ProgrammableTransaction, Error>
  where
    C: CoreClientReadOnly + OptionalSync,
  {
    if self.programmable_tx.is_none() {
      self.programmable_tx = Some(self.tx.build_programmable_transaction(client).await?);
    }

    Ok(self.programmable_tx.as_ref().unwrap())
  }

  /// Attempts to build this transaction using `client` in a best effort manner:
  /// - when no sender had been supplied, client's address is used;
  /// - when gas information is incomplete, the client will attempt to fill it, making use of whatever funds its address
  ///   has, if possible;
  /// - when signatures are missing, the client will provide its own if possible;
  ///
  /// ## Notes
  /// This method *DOES NOT* remove nor checks for invalid signatures.
  /// Transaction with invalid signatures will fail after attempting to execute them.
  pub async fn build<S>(
    mut self,
    client: &(impl CoreClient<S> + OptionalSync),
  ) -> Result<(TransactionData, Vec<Signature>, Tx), Error>
  where
    S: Signer<IotaKeySignature> + OptionalSync,
  {
    self.get_or_init_programmable_tx(client).await?;
    let programmable_tx = self.programmable_tx.expect("just computed it");
    let client_address = client.sender_address();
    let sender = self.sender.unwrap_or(client_address);
    let gas_data = complete_gas_data_for_tx(&programmable_tx, self.gas, client)
      .await
      .map_err(|e| Error::GasIssue(e.to_string()))?;

    let tx_data = TransactionData::new_with_gas_data(
      TransactionKind::ProgrammableTransaction(programmable_tx),
      sender,
      gas_data,
    );

    let mut signatures = self.signatures;
    let needs_client_signature = client_address == sender
      || client_address == tx_data.gas_data().owner
        && !signatures.iter().map(address_from_signature).contains(&client_address);
    if needs_client_signature {
      let signature = client
        .signer()
        .sign(&tx_data)
        .await
        .map_err(|e| Error::TransactionSigningFailed(e.to_string()))?;
      signatures.push(signature);
    }

    Ok((tx_data, signatures, self.tx))
  }

  /// Attempts to build and execute this transaction using `client` in a best effort manner:
  /// - when no sender had been supplied, client's address is used;
  /// - when gas information is incomplete, the client will attempt to fill it, making use of whatever funds its address
  ///   has, if possible;
  /// - when signatures are missing, the client will provide its own if possible;
  ///
  /// After the transaction has been successfully executed, the transaction's effect will be computed.
  /// ## Notes
  /// This method *DOES NOT* remove nor checks for invalid signatures.
  /// Transaction with invalid signatures will fail after attempting to execute them.
  pub async fn build_and_execute<S, C>(self, client: &C) -> Result<TransactionOutput<Tx::Output>, Error>
  where
    C: CoreClient<S> + OptionalSync,
    S: Signer<IotaKeySignature> + OptionalSync,
  {
    // Build the transaction into its parts.
    let (tx_data, signatures, tx) = self.build(client).await?;

    // Execute and wait for the transaction to be confirmed.
    let dyn_tx_block = client
      .client_adapter()
      .quorum_driver_api()
      .execute_transaction_block(
        tx_data,
        signatures,
        Some(IotaTransactionBlockResponseOptions::full_content()),
        Some(ExecuteTransactionRequestType::WaitForLocalExecution),
      )
      .await?;

    // Get the transaction's effects, making sure they are successful.
    let tx_effects = dyn_tx_block
      .effects()
      .ok_or_else(|| Error::TransactionUnexpectedResponse("missing effects in response".to_owned()))?
      .clone();
    let tx_status = tx_effects.status();
    if tx_status.is_err() {
      return Err(Error::TransactionUnexpectedResponse(format!(
        "errors in transaction's effects: {}",
        tx_status
      )));
    }

    let (application_result, _remaining_effects) = tx.apply(tx_effects, client).await;
    let response = {
      cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
          dyn_tx_block
        } else {
          dyn_tx_block.clone_native_response()
        }
      }
    };
    // Apply the off-chain logic of the transaction by parsing the transaction's effects.
    // If the application goes awry, salvage the response by returning it alongside the error.
    let output = match application_result {
      Ok(output) => output,
      Err(e) => {
        #[cfg(not(target_arch = "wasm32"))]
        let response = Box::new(response);
        #[cfg(target_arch = "wasm32")]
        // For WASM the response is passed in the error as its JSON-encoded string representation.
        let response = response.as_native_response().to_string();
        return Err(Error::TransactionOffChainApplicationFailure {
          source: Box::new(e),
          response,
        });
      }
    };

    Ok(TransactionOutput { output, response })
  }
}

impl<Tx> TransactionBuilder<Tx> {
  /// Returns the partial [Transaction] wrapped by this builder, consuming it.
  pub fn into_inner(self) -> Tx {
    self.tx
  }

  /// Sets the address that will execute the transaction.
  pub fn with_sender(mut self, sender: IotaAddress) -> Self {
    self.sender = Some(sender);
    self
  }

  /// Sets the gas budget for this transaction.
  pub fn with_gas_budget(mut self, budget: u64) -> Self {
    self.gas.budget = Some(budget);
    self
  }

  /// Sets the coins to use to cover the gas cost.
  pub fn with_gas_payment(mut self, coins: Vec<ObjectRef>) -> Self {
    self.gas.payment = coins;
    self
  }

  /// Sets the gas owner.
  pub fn with_gas_owner(mut self, address: IotaAddress) -> Self {
    self.gas.owner = Some(address);
    self
  }

  /// Sets the gas price.
  pub fn with_gas_price(mut self, price: u64) -> Self {
    self.gas.price = Some(price);
    self
  }

  /// Sets the gas information that must be used to execute this transaction.
  pub fn with_gas_data(mut self, gas_data: GasData) -> Self {
    self.gas = gas_data.into();
    self
  }

  /// Attempts to construct a [TransactionBuilder] from a whole transaction.
  pub fn try_from_signed_transaction(
    tx_data: TransactionData,
    signatures: Vec<Signature>,
    effect: Tx,
  ) -> Result<Self, Error> {
    #[allow(irrefutable_let_patterns)]
    let TransactionKind::ProgrammableTransaction(pt) = tx_data.kind().clone() else {
      return Err(Error::TransactionBuildingFailed(
        "only programmable transactions are supported".to_string(),
      ));
    };
    let sender = tx_data.sender();
    let gas = tx_data.gas_data().clone().into();

    Ok(Self {
      programmable_tx: Some(pt),
      sender: Some(sender),
      gas,
      signatures,
      tx: effect,
    })
  }
}

/// Returns a best effort [GasData] for the given transaction, partial gas information, and client.
/// ## Notes
/// If a field is missing from gas data:
/// - client's address is set as the gas owner;
/// - current gas price is fetched from a node;
/// - budget is calculated by dry running the transaction;
/// - payment is set to whatever IOTA coins the gas owner has, that satisfy the tx's budget;
async fn complete_gas_data_for_tx<S, C>(
  pt: &ProgrammableTransaction,
  partial_gas_data: PartialGasData,
  client: &C,
) -> anyhow::Result<GasData>
where
  C: CoreClient<S> + OptionalSync,
  S: Signer<IotaKeySignature> + OptionalSync,
{
  let owner = partial_gas_data.owner.unwrap_or(client.sender_address());
  let price = if let Some(price) = partial_gas_data.price {
    price
  } else {
    client.client_adapter().read_api().get_reference_gas_price().await?
  };
  let budget = if let Some(budget) = partial_gas_data.budget {
    budget
  } else {
    client.client_adapter().default_gas_budget(owner, pt).await?
  };
  let payment = if !partial_gas_data.payment.is_empty() {
    partial_gas_data.payment
  } else {
    client.get_iota_coins_with_at_least_balance(owner, budget).await?
  };

  Ok(GasData {
    owner,
    payment,
    price,
    budget,
  })
}

/// Extract the signer's address from an IOTA [Signature].
fn address_from_signature(signature: &Signature) -> IotaAddress {
  let scheme = signature.scheme();
  let pk_bytes = signature.public_key_bytes();
  let pk = PublicKey::try_from_bytes(scheme, pk_bytes).expect("valid signature hence valid key");

  IotaAddress::from(&pk)
}
