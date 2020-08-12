use anyhow::Result;
use iota::{
  client::Transfer,
  crypto::ternary::sponge::{CurlP81, Sponge},
  crypto::ternary::Hash,
  ternary::{T1B1Buf, TritBuf, TryteBuf},
  transaction::bundled::{Address, BundledTransaction, BundledTransactionField, Index},
};
use iota_conversion::Trinary;

#[derive(Clone)]
pub struct TangleWriter {
  pub node: String,
  pub network: iota::client::builder::Network,
  // Add the sent trytes here?
}

impl TangleWriter {
  pub async fn publish_document(&self, address: &str, did_document: Option<String>) -> Result<Vec<BundledTransaction>> {
    // Get address from did_document or external diff chain?
    // Is it possible to get the address from the did_document after an auth change?
    let mut transfers = Vec::new();
    transfers.push(Transfer {
      address: Address::from_inner_unchecked(TryteBuf::try_from_str(address).unwrap().as_trits().encode()),
      value: 0,
      message: did_document,
      tag: None,
    });

    // Create a client instance
    let iota = iota::ClientBuilder::new()
      .node(&self.node)?
      // Investigate why this doesn't work, would be better than setting mwm
      // .network(iota::client::builder::Network::Comnet)
      .build()?;

    // Send the actual transaction
    let res = iota
      .send(None)
      .transfers(transfers)
      .min_weight_magnitude(10)
      .send()
      .await?;

    Ok(res)
  }

  pub async fn promote(&self, tx: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut transfers = Vec::new();
    transfers.push(Transfer {
      address: Address::from_inner_unchecked(
        TryteBuf::try_from_str(&String::from(
          "PROMOTEADDRESSPROMOTEADDRESSPROMOTEADDRESSPROMOTEADDRESSPROMOTEADDRESSPROMOTEADDR",
        ))
        .unwrap()
        .as_trits()
        .encode(),
      ),
      value: 0,
      message: None,
      tag: None,
    });

    let iota = iota::ClientBuilder::new()
      .node(&self.node)?
      .network(self.network.clone())
      .build()?;

    let prepared_transfers = iota.prepare_transfers(None).transfers(transfers).build().await?;
    let tips = iota.get_transactions_to_approve().depth(2).send().await?;
    let attached_trytes = iota
      .attach_to_tangle()
      .trunk_transaction(&Hash::from_inner_unchecked(
        TryteBuf::try_from_str(tx).unwrap().as_trits().encode(),
      ))
      .branch_transaction(&tips.branch_transaction)
      .trytes(&[prepared_transfers[0].clone()])
      .send()
      .await?;

    iota.broadcast_transactions(&attached_trytes.trytes).await.unwrap();
    Ok(
      prepared_transfers[0]
        .bundle()
        .to_inner()
        .as_i8_slice()
        .trytes()
        .unwrap(),
    )
  }

  pub async fn reattach(&self, trytes: Vec<BundledTransaction>) -> Result<String, Box<dyn std::error::Error>> {
    let iota = iota::ClientBuilder::new()
      .node(&self.node)?
      .network(self.network.clone())
      .build()?;
    let reattached = iota.send_trytes().trytes(trytes).depth(2).send().await?;
    Ok(reattached[0].bundle().to_inner().as_i8_slice().trytes().unwrap())
  }

  pub async fn is_confirmed(&self, bundle: Hash) -> Result<(Hash, bool), Box<dyn std::error::Error>> {
    let iota = iota::ClientBuilder::new()
      .node(&self.node)?
      .network(self.network.clone())
      .build()?;

    let response = iota.find_transactions().bundles(&[bundle]).send().await?;

    let trytes = iota.get_trytes(&response.hashes).await?;

    let mut tail_txs = trytes
      .trytes
      .iter()
      .filter(|tx| tx.index() == &Index::from_inner_unchecked(0))
      .collect::<Vec<&BundledTransaction>>();

    //sort txs based on attachment timestamp
    tail_txs.sort_by(|a, b| b.get_timestamp().cmp(&a.get_timestamp()));

    let tail_txs_hashes: Vec<Hash> = tail_txs
      .iter()
      .map(|tx| {
        let mut curl = CurlP81::new();
        let mut trits = TritBuf::<T1B1Buf>::zeros(BundledTransaction::trit_len());
        tx.into_trits_allocated(&mut trits);
        return Hash::from_inner_unchecked(curl.digest(&trits).unwrap());
      })
      .collect();

    let inclusion_states = iota
      .get_inclusion_states()
      .transactions(&tail_txs_hashes)
      .send()
      .await?;
    println!("Inclusion states: {:?}", inclusion_states.states);

    Ok((tail_txs_hashes[0], inclusion_states.states.contains(&true)))
  }
}
