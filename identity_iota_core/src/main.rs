use std::error::Error;

use iota_client::block::protocol::dto::ProtocolParametersDto;
use iota_client::block::protocol::ProtocolParameters;

fn main() -> Result<(), Box<dyn Error>> {
  let json: &str = r#"{
        "version":2,
        "networkName":"testnet",
        "bech32Hrp":"rms",
        "minPowScore":1500,
        "belowMaxDepth":15,
        "rentStructure":{"vByteCost":100,"vByteFactorKey":10,"vByteFactorData":1},
        "tokenSupply":"1450896407249092"
        }"#;
  let protocol_parameters_dto: ProtocolParametersDto = serde_json::from_str(json)?;
  let protocol_parameters: ProtocolParameters = ProtocolParameters::try_from(protocol_parameters_dto)?;
  println!("protocol parameters:  {:#?}", protocol_parameters);
  Ok(())
}
