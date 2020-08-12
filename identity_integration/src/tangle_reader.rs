use anyhow::Result;
use iota::{
  ternary::TryteBuf,
  transaction::bundled::{Address, BundledTransactionField},
};
use iota_conversion::{trytes_converter, Trinary};

#[derive(Clone)]
pub struct TangleReader {
  pub node: String,
}

impl TangleReader {
  pub async fn fetch(&self, address: &str) -> Result<String, Box<dyn std::error::Error>> {
    let iota = iota::ClientBuilder::new().node(&self.node)?.build()?;

    let address = Address::from_inner_unchecked(TryteBuf::try_from_str(address).unwrap().as_trits().encode());

    let response = iota.find_transactions().addresses(&[address]).send().await?;

    // Todo order local transactions from iota.find_transactions().addresses
    // let mut tail_txs = response
    //   .trytes
    //   .iter()
    //   .filter(|tx| tx.index() == &Index::from_inner_unchecked(0))
    //   .collect::<Vec<&BundledTransaction>>();

    let bundle = iota.get_bundle(&response.hashes[0]).await?;
    let trytes_coll: Vec<String> = bundle
      .iter()
      .map(|t| {
        t.payload()
          .to_inner()
          .as_i8_slice()
          .trytes()
          .unwrap()
          .trim_end_matches('9')
          .to_string()
      })
      .collect();
    let message = match trytes_converter::to_string(&trytes_coll.concat()) {
      Ok(m) => m,
      Err(e) => {
        println!("Error: trytes_converter.to_string()\n\t{}", e);
        std::process::exit(1);
      }
    };
    Ok(message)
  }
}
