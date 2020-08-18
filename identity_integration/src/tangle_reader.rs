use anyhow::Result;
use iota::{
  crypto::ternary::sponge::{CurlP81, Sponge},
  crypto::ternary::Hash,
  ternary::{T1B1Buf, TritBuf, TryteBuf},
  transaction::bundled::{Address, BundledTransaction, BundledTransactionField, Index},
  transaction::Vertex,
};
use iota_conversion::{trytes_converter, Trinary};
use std::collections::HashMap;

#[derive(Clone)]
pub struct TangleReader {
  pub nodes: Vec<&'static str>,
}

impl TangleReader {
  pub async fn fetch(&self, address: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let iota = iota::ClientBuilder::new().nodes(&self.nodes)?.build()?;

    let address = Address::from_inner_unchecked(TryteBuf::try_from_str(address).unwrap().as_trits().encode());

    let response = iota.find_transactions().addresses(&[address]).send().await?;
    let txs = iota.get_trytes(&response.hashes).await?;
    // Order transaction to bundles
    let mut bundles = HashMap::new();
    let mut transactions = HashMap::new();
    for tx in &txs.trytes {
      let mut curl = CurlP81::new();
      let mut trits = TritBuf::<T1B1Buf>::zeros(BundledTransaction::trit_len());
      tx.into_trits_allocated(&mut trits);
      let tx_hash = Hash::from_inner_unchecked(curl.digest(&trits).unwrap());

      if tx.index() == &Index::from_inner_unchecked(0) {
        // Distinguish between tail transactions, because message can be changed at reattachments
        bundles.insert(tx_hash, vec![tx]);
      } else {
        transactions.insert(tx_hash, tx);
      }
    }

    for bundle in bundles.values_mut() {
      for index in 0..*bundle[0].last_index().to_inner() {
        if let Some(trunk_transaction) = transactions.get(&Hash::from_inner_unchecked(
          TritBuf::from_i8s(bundle[index].trunk().to_inner().as_i8_slice()).unwrap(),
        )) {
          bundle.push(trunk_transaction);
        } else {
          println!(
            "Trunk transaction not found: https://comnet.thetangle.org/transaction/{}",
            bundle[index].trunk().to_inner().as_i8_slice().trytes().unwrap()
          );
        }
      }
      // Check if all transactions are there
      if bundle.len() != *bundle[0].last_index().to_inner() + 1 {
        println!(
          "Not all transactions for {} are known",
          bundle[0].bundle().to_inner().as_i8_slice().trytes().unwrap()
        );
      }
    }

    // Convert messages to ascii
    let mut messages = Vec::new();
    for bundle in bundles.values_mut() {
      let trytes_coll: Vec<String> = bundle
        .iter()
        .map(|t| t.payload().to_inner().as_i8_slice().trytes().unwrap())
        .collect();

      if let Ok(message) = trytes_converter::to_string(&trytes_coll.concat()) {
        messages.push(message);
      }
    }

    Ok(messages)
  }
}
