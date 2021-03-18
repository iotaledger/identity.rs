// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::slice::from_ref;
use identity_core::common::Url;
use identity_core::convert::ToJson;
use iota::client::FindTransactionsResponse;
use iota::client::GetTrytesResponse;
use iota::client::Transfer;
use iota::transaction::bundled::Address;
use iota::transaction::bundled::BundledTransaction;
use iota::transaction::bundled::BundledTransactionField;

use crate::chain::AuthChain;
use crate::chain::DiffChain;
use crate::chain::DocumentChain;
use crate::client::ClientBuilder;
use crate::client::Network;
use crate::client::TxnPrinter;
use crate::did::Document;
use crate::did::DocumentDiff;
use crate::did::DID;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::Message;
use crate::tangle::MessageId;
use crate::utils::bundles_from_trytes;
use crate::utils::create_address_from_trits;
use crate::utils::encode_trits;
use crate::utils::txn_hash_trytes;

#[derive(Clone, Debug)]
pub struct Client {
  pub(crate) client: iota::Client,
  pub(crate) network: Network,
}

impl Client {
  /// Creates a new `Client`  with default settings.
  pub fn new() -> Result<Self> {
    Self::from_builder(Self::builder())
  }

  /// Creates a `ClientBuilder` to configure a new `Client`.
  ///
  /// This is the same as [ClientBuilder::new]`.
  pub fn builder() -> ClientBuilder {
    ClientBuilder::new()
  }

  /// Creates a new `Client` with default settings for the given `Network`.
  pub fn from_network(network: Network) -> Result<Self> {
    Self::builder()
      .node(network.node_url().as_str())
      .network(network)
      .build()
  }

  /// Creates a new `Client` based on the `ClientBuilder` configuration.
  pub fn from_builder(builder: ClientBuilder) -> Result<Self> {
    let mut client: iota::ClientBuilder = iota::ClientBuilder::new();

    if builder.nodes.is_empty() {
      client = client.node(builder.network.node_url().as_str())?;
    } else {
      for node in builder.nodes {
        client = client.node(&node)?;
      }
    }

    client = client.network(builder.network.into());

    Ok(Self {
      client: client.build()?,
      network: builder.network,
    })
  }

  /// Returns the `Client` Tangle network.
  pub fn network(&self) -> Network {
    self.network
  }

  /// Returns the default node URL of the `Client` network.
  pub fn default_node_url(&self) -> &'static Url {
    self.network.node_url()
  }

  /// Returns the web explorer URL of the `Client` network.
  pub fn explorer_url(&self) -> &'static Url {
    self.network.explorer_url()
  }

  /// Returns the web explorer URL of the given `transaction`.
  pub fn transaction_url(&self, transaction: &BundledTransaction) -> Url {
    let hash: TxnPrinter<'_, _> = TxnPrinter::hash(transaction);
    let mut url: Url = self.network.explorer_url().clone();

    url
      .path_segments_mut()
      .unwrap()
      .push("transaction")
      .push(&hash.to_string());

    url
  }

  /// Returns the hash of the Tangle transaction as a tryte-encoded `String`.
  pub fn transaction_hash(&self, transaction: &BundledTransaction) -> String {
    txn_hash_trytes(transaction)
  }

  /// Publishes an DID Document to the Tangle; returns the `MessageId` of
  /// the bundled transaction.
  ///
  /// Note: The only validation performed is to ensure the correct Tangle
  /// network is selected.
  pub async fn publish_document(&self, document: &Document) -> Result<MessageId> {
    trace!("Publish Document: {}", document.id());
    trace!("Tangle Address: {}", document.id().address());

    self.check_network(document.id())?;

    let address: String = document.id().address();
    let transfer: Transfer = create_transfer(&address, document)?;
    let bundled: BundledTransaction = self.send_transfer(transfer).await?;

    Ok(txn_hash_trytes(&bundled).into())
  }

  /// Publishes a `DocumentDiff` to the Tangle; returns the `MessageId` of
  /// the bundled transaction.
  ///
  /// Note: The only validation performed is to ensure the correct Tangle
  /// network is selected.
  pub async fn publish_diff(&self, message_id: &MessageId, diff: &DocumentDiff) -> Result<MessageId> {
    trace!("Publish Diff: {}", diff.id());
    trace!("Tangle Address: {}", Document::diff_address(message_id)?);

    self.check_network(diff.id())?;

    let address: String = Document::diff_address(message_id)?;
    let transfer: Transfer = create_transfer(&address, diff)?;
    let bundled: BundledTransaction = self.send_transfer(transfer).await?;

    Ok(txn_hash_trytes(&bundled).into())
  }

  pub async fn read_document(&self, did: &DID) -> Result<Document> {
    self.read_document_chain(did).await.and_then(DocumentChain::fold)
  }

  pub async fn read_document_chain(&self, did: &DID) -> Result<DocumentChain> {
    trace!("Read Document Chain: {}", did);
    trace!("Auth Chain Address: {}", did.address());

    // Fetch all messages for the auth chain.
    let address: String = did.address();
    let messages: Vec<Message> = self.read_messages(&address).await?;

    let auth: AuthChain = AuthChain::try_from_messages(did, &messages)?;

    let diff: DiffChain = if auth.current().immutable() {
      DiffChain::new()
    } else {
      // Fetch all messages for the diff chain.
      let address: String = Document::diff_address(auth.current_message_id())?;
      let messages: Vec<Message> = self.read_messages(&address).await?;

      trace!("Tangle Messages: {:?}", messages);

      DiffChain::try_from_messages(&auth, &messages)?
    };

    DocumentChain::with_diff_chain(auth, diff)
  }

  #[doc(hidden)]
  pub async fn read_messages(&self, address: &str) -> Result<Vec<Message>> {
    let address: Address = create_address_from_trits(address)?;

    trace!("Read Transactions: {}", encode_trits(address.to_inner()));

    // Fetch all transaction hashes containing the tangle address.
    let response: FindTransactionsResponse = self
      .client
      .find_transactions()
      .addresses(from_ref(&address))
      .send()
      .await?;

    trace!("Transactions Found: {:?}", __dbg_transactions(&response));

    if response.hashes.is_empty() {
      return Ok(Vec::new());
    }

    // Fetch the content of all transactions.
    let content: GetTrytesResponse = self.client.get_trytes(&response.hashes).await?;

    trace!("Transaction Trytes: {:?}", __dbg_trytes(&content));

    if content.trytes.is_empty() {
      return Err(Error::InvalidTransactionTrytes);
    }

    // Re-build the fragmented messages stored in the bundle.
    bundles_from_trytes(content.trytes)
      .into_iter()
      .map(Message::try_from_bundle)
      .collect()
  }

  pub(crate) async fn send_transfer(&self, transfer: Transfer) -> Result<BundledTransaction> {
    trace!("Sending Transfer: {:?}", transfer.message);

    self
      .client
      .send(None)
      .transfers(vec![transfer])
      .send()
      .await?
      .into_iter()
      .find(BundledTransaction::is_tail)
      .ok_or(Error::InvalidBundleTail)
  }

  pub(crate) fn check_network(&self, did: &DID) -> Result<()> {
    if !self.network.matches_did(did) {
      return Err(Error::InvalidDIDNetwork);
    }

    Ok(())
  }
}

fn create_transfer<T>(address: &str, data: &T) -> Result<Transfer>
where
  T: ToJson,
{
  Ok(Transfer {
    address: create_address_from_trits(address)?,
    value: 0,
    message: Some(data.to_json()?),
    tag: None,
  })
}

fn __dbg_transactions(response: &FindTransactionsResponse) -> Vec<String> {
  response.hashes.iter().map(|hash| encode_trits(hash)).collect()
}

fn __dbg_trytes(response: &GetTrytesResponse) -> Vec<TxnPrinter<'_>> {
  response.trytes.iter().map(TxnPrinter::full).collect()
}
