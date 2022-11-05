use std::error::Error;

use iota_client::block::protocol::ProtocolParameters;

fn main() -> Result<(),Box<dyn Error>>{
    let json: &str = r#"{
        "version":2,
        "networkName":"testnet",
        "bech32Hrp":"rms",
        "minPowScore":1500,
        "belowMaxDepth":15,
        "rentStructure":{"vByteCost":100,"vByteFactorKey":10,"vByteFactorData":1},
        "tokenSupply":"1450896407249092"
        }"#;
    let protocol_parameters: Result<ProtocolParameters,_> = serde_json::from_str(json);
    protocol_parameters.map(|parameters| println!("{:#?}", parameters)).map_err(Into::into)

}