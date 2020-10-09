use identity_core::did::DID;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Network {
    Mainnet,
    Devnet,
    Comnet,
}

impl Network {
    pub fn matches_did(self, did: &DID) -> bool {
        // TODO: Move this to a generic DID helper
        let network: Option<&str> = did.id_segments.get(0).map(|string| string.as_str());

        match (self, network) {
            (Self::Devnet, Some(network)) => network == "dev",
            (Self::Comnet, Some(network)) => network == "com",
            (Self::Devnet, _) => false,
            (Self::Comnet, _) => false,
            (Self::Mainnet, _) => true,
        }
    }
}

use iota::client::builder;

impl From<builder::Network> for Network {
    fn from(other: builder::Network) -> Network {
        match other {
            builder::Network::Mainnet => Self::Mainnet,
            builder::Network::Devnet => Self::Devnet,
            builder::Network::Comnet => Self::Comnet,
        }
    }
}

impl From<Network> for builder::Network {
    fn from(other: Network) -> builder::Network {
        match other {
            Network::Mainnet => Self::Mainnet,
            Network::Devnet => Self::Devnet,
            Network::Comnet => Self::Comnet,
        }
    }
}
