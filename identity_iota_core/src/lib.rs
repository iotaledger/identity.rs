// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]

#[cfg(feature = "iota-client")]
pub use iota_client::block;
// Re-export the `iota_types::block` module for implementer convenience.
#[cfg(all(feature = "client", not(feature = "iota-client")))]
pub use iota_types::block;

#[cfg(feature = "client")]
pub use client::*;
pub use did::IotaDID;
pub use did::IotaDIDUrl;
pub use document::*;
pub use network::NetworkName;
pub use state_metadata::*;

pub use self::error::Error;
pub use self::error::Result;

#[cfg(feature = "client")]
mod client;
mod did;
mod document;
mod error;
mod network;
mod state_metadata;

#[cfg(test)]
mod tests {
    #[cfg(feature = "iota-client")]
    #[test]
    fn test_deserialize() {
        use iota_client::block::protocol::ProtocolParameters;

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
        assert!(protocol_parameters.is_ok()); 
    }
}
