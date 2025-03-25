// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context as _;
use async_trait::async_trait;
use identity_iota_interaction::rpc_types::IotaTransactionBlockEffects;
use identity_iota_interaction::rpc_types::IotaTransactionBlockResponseOptions;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::crypto::PublicKey;
use identity_iota_interaction::types::crypto::Signature;
use identity_iota_interaction::types::crypto::SignatureScheme;
use identity_iota_interaction::types::quorum_driver_types::ExecuteTransactionRequestType;
use identity_iota_interaction::types::transaction::GasData;
use identity_iota_interaction::types::transaction::ProgrammableTransaction;
use identity_iota_interaction::types::transaction::TransactionData;
use identity_iota_interaction::types::transaction::TransactionDataAPI as _;
use identity_iota_interaction::types::transaction::TransactionKind;
use identity_iota_interaction::IotaClientTrait;
use identity_iota_interaction::IotaKeySignature;
use itertools::Itertools;
use secret_storage::Signer;

use super::client::IdentityClient;
use super::client::IdentityClientReadOnly;
use super::transaction::TransactionOutput;
use super::Error;

/// Effects of a transaction to local context.
#[async_trait]
pub trait TxEffect {
  /// Output type for the effect.
  type Output;
  /// Parses a transaction result in order to compute this effect.
  async fn apply(
    self,
    tx_results: &IotaTransactionBlockEffects,
    client: &IdentityClientReadOnly,
  ) -> Result<Self::Output, Error>;
}

/// Builds an executable transaction on a step by step manner.
#[derive(Debug)]
pub struct TxBuilder<Effect> {
  programmable_tx: ProgrammableTransaction,
  sender: Option<IotaAddress>,
  gas: Option<GasData>,
  signatures: Vec<Signature>,
  effect: Effect,
}

impl<Effect> TxBuilder<Effect> {
  /// Starts the creation of a transaction by supplying the transaction's data
  /// together with its local effect.
  pub fn new(pt: ProgrammableTransaction, effect: Effect) -> Self {
    Self {
      programmable_tx: pt,
      effect,
      gas: None,
      signatures: vec![],
      sender: None,
    }
  }

  /// Attempts to construct a [TxBuilder] from a whole transaction.
  pub fn try_from_signed_transaction(
    tx_data: TransactionData,
    signatures: Vec<Signature>,
    effect: Effect,
  ) -> Result<Self, Error> {
    let TransactionKind::ProgrammableTransaction(pt) = tx_data.kind().clone() else {
      return Err(Error::TransactionBuildingFailed(
        "only programmable transactions are supported".to_string(),
      ));
    };
    let sender = tx_data.sender();
    let gas = tx_data.gas_data().clone();

    Ok(Self {
      programmable_tx: pt,
      sender: Some(sender),
      gas: Some(gas),
      signatures,
      effect,
    })
  }

  /// Sets the address that will execute the transaction.
  pub fn with_sender(mut self, sender: IotaAddress) -> Self {
    self.sender = Some(sender);
    self
  }

  /// Sets the gas information that must be used to execute this transaction.
  pub fn with_gas_data(mut self, gas_data: GasData) -> Self {
    self.gas = Some(gas_data);
    self
  }

  fn transaction_data(&self) -> anyhow::Result<TransactionData> {
    let sender = self.sender.context("missing sender")?;
    let gas_data = self.gas.clone().context("missing gas data")?;

    Ok(TransactionData::new_with_gas_data(
      TransactionKind::ProgrammableTransaction(self.programmable_tx.clone()),
      sender,
      gas_data,
    ))
  }

  /// Adds `signer`'s signature to this this transaction's signatures list.
  /// # Notes
  /// This methods asserts that `signer`'s address matches the address of
  /// either this transaction's sender or the gas owner - failing otherwise.
  pub async fn with_signature<S>(mut self, signer: &S) -> Result<Self, Error>
  where
    S: Signer<IotaKeySignature>,
  {
    let pk = signer
      .public_key()
      .await
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
    let signer_address = IotaAddress::from(&pk);
    let matches_sender = self.sender.is_none_or(|sender| sender == signer_address);
    let matches_gas_owner = self.gas.as_ref().is_none_or(|gas| gas.owner == signer_address);
    if !(matches_sender || matches_gas_owner) {
      return Err(Error::TransactionBuildingFailed(format!(
        "signer's address {signer_address} doesn't match the address of either the transaction sender or the gas owner"
      )));
    }

    let tx_data = self
      .transaction_data()
      .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
    let tx_data_bcs = bcs::to_bytes(&tx_data)?;

    let sig = signer
      .sign(&tx_data_bcs)
      .await
      .map_err(|e| Error::TransactionSigningFailed(e.to_string()))?;
    self.signatures.push(sig);

    Ok(self)
  }

  /// Returns this transaction's [ProgrammableTransaction] together with its effect,
  /// consuming all other information.
  pub fn into_programmable_transaction_and_effect(self) -> (ProgrammableTransaction, Effect) {
    (self.programmable_tx, self.effect)
  }

  /// Returns this transaction's [ProgrammableTransaction], consuming all other information.
  pub fn into_programmable_transaction(self) -> ProgrammableTransaction {
    self.programmable_tx
  }
}

impl<Effect> TxBuilder<Effect>
where
  Effect: TxEffect,
{
  /// Attempts to execute this transaction using `client` in a best effort manner:
  /// - when no sender had been supplied, client's address is used;
  /// - when no gas data had been supplied, the client will provide it, making use of whatever funds its address has;
  /// - when signatures are missing, the client will provide its own if possible;
  ///
  /// After the transaction has been successfully executed, the transaction's effect will be computed.
  /// ## Notes
  /// This method *DO NOT* removes nor checks for invalid signatures.
  /// Transaction with invalid signatures will fail after attempting to execute them.
  pub async fn execute<S>(self, client: &IdentityClient<S>) -> Result<TransactionOutput<Effect::Output>, Error>
  where
    S: Signer<IotaKeySignature>,
  {
    let client_address = client.sender_address();
    let sender = self.sender.unwrap_or(client_address);
    let gas_data = if let Some(gas_data) = self.gas {
      gas_data
    } else {
      default_gas_data_for_tx(&self.programmable_tx, client)
        .await
        .map_err(|e| Error::GasIssue(e.to_string()))?
    };

    let tx_data = TransactionData::new_with_gas_data(
      TransactionKind::ProgrammableTransaction(self.programmable_tx),
      sender,
      gas_data,
    );
    let tx_data_bcs = bcs::to_bytes(&tx_data)?;

    let mut signatures = self.signatures;
    let needs_client_signature = client_address == sender
      || client_address == tx_data.gas_data().owner
        && !signatures.iter().map(address_from_signature).contains(&client_address);
    if needs_client_signature {
      let signature = client
        .signer()
        .sign(&tx_data_bcs)
        .await
        .map_err(|e| Error::TransactionSigningFailed(e.to_string()))?;
      signatures.push(signature);
    }

    let signatures_bcs = signatures
      .into_iter()
      .map(|sig| bcs::to_bytes(&sig))
      .collect::<Result<Vec<_>, _>>()?;

    let response = client
      .quorum_driver_api()
      .execute_transaction_block(
        &tx_data_bcs,
        &signatures_bcs,
        Some(IotaTransactionBlockResponseOptions::full_content()),
        Some(ExecuteTransactionRequestType::WaitForLocalExecution),
      )
      .await?
      .clone_native_response();

    let tx_effects = response
      .effects
      .as_ref()
      .ok_or_else(|| Error::TransactionUnexpectedResponse("missing effects in response".to_owned()))?;
    let output = self.effect.apply(tx_effects, client).await?;

    Ok(TransactionOutput { output, response })
  }
}

/// Returns a best effort [GasData] for the given transaction and client.
/// ## Notes
/// - client's address is set as the gas owner;
/// - current gas price is fetched from a node;
/// - budget is calculated by dry running the transaction;
/// - payment is set to whatever IOTA coins client's address owns that satisfy the tx's budget;
async fn default_gas_data_for_tx<S>(pt: &ProgrammableTransaction, client: &IdentityClient<S>) -> anyhow::Result<GasData>
where
  S: Signer<IotaKeySignature>,
{
  let owner = client.sender_address();
  let price = client.read_api().get_reference_gas_price().await?;
  let pt_bcs = bcs::to_bytes(pt)?;
  let budget = client.default_gas_budget(owner, &pt_bcs).await?;
  let payment = client.get_iota_coins_with_at_least_balance(owner, budget).await?;

  Ok(GasData {
    owner,
    payment,
    price,
    budget,
  })
}

/// Extract the signer's address from an IOTA [Signature].
fn address_from_signature(signature: &Signature) -> IotaAddress {
  let scheme_bytes_flag = signature.as_ref()[0];
  let scheme = SignatureScheme::from_flag_byte(&scheme_bytes_flag).expect("valid signature");
  let pk_bytes = &signature.as_ref()[65..]; // flag || sig || pk -> flag is 1 bytes, sig is 64 bytes.
  let pk = PublicKey::try_from_bytes(scheme, pk_bytes).expect("valid signature hence valid key");

  IotaAddress::from(&pk)
}

mod tests {
  use crate::iota_interaction_rust::IdentityMoveCallsAdapter;
  use crate::IotaDocument;
  use crate::StateMetadataDocument;
  use crate::StateMetadataEncoding;
  use identity_iota_interaction::rpc_types::IotaTransactionBlockEffectsAPI as _;
  use identity_iota_interaction::IdentityMoveCalls as _;

  use super::*;

  #[derive(Debug)]
  pub struct PublishDidDocument;

  #[async_trait]
  impl TxEffect for PublishDidDocument {
    type Output = IotaDocument;
    async fn apply(
      self,
      effects: &IotaTransactionBlockEffects,
      client: &IdentityClientReadOnly,
    ) -> Result<Self::Output, Error> {
      if effects.status().is_err() {
        return Err(Error::TransactionUnexpectedResponse(
          "unsuccessfull transaction".to_owned(),
        ));
      }

      let identity_id = effects.created()[0].object_id();
      let identity = client.get_identity(identity_id).await?;

      Ok(identity.did_document(client.network())?)
    }
  }

  impl<S> IdentityClient<S>
  where
    S: Signer<IotaKeySignature>,
  {
    pub fn publish_did_document_builder_api(
      &self,
      did_doc: IotaDocument,
    ) -> anyhow::Result<TxBuilder<PublishDidDocument>> {
      let did_doc = StateMetadataDocument::from(did_doc).pack(StateMetadataEncoding::Json)?;
      let controllers = [(self.sender_address(), 1)];
      let programmable_tx_bcs =
        IdentityMoveCallsAdapter::new_with_controllers(Some(&did_doc), controllers, 1, self.package_id())?;
      let programmable_tx = bcs::from_bytes(&programmable_tx_bcs)?;

      Ok(TxBuilder::new(programmable_tx, PublishDidDocument))
    }
  }
}
