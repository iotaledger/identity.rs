use identity_core::common::Url;
use iota::client::builder;

use crate::did::IotaDID;

lazy_static! {
    static ref EXPLORER_MAIN: Url = Url::parse("https://explorer.iota.org/mainnet").unwrap();
    static ref EXPLORER_DEV: Url = Url::parse("https://explorer.iota.org/devnet").unwrap();
    static ref EXPLORER_COM: Url = Url::parse("https://comnet.thetangle.org").unwrap();
    static ref NODE_MAIN: Url = Url::parse("https://nodes.iota.org:443").unwrap();
    static ref NODE_DEV: Url = Url::parse("https://nodes.devnet.iota.org:443").unwrap();
    static ref NODE_COM: Url = Url::parse("https://nodes.comnet.thetangle.org:443").unwrap();
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Network {
    Mainnet,
    Devnet,
    Comnet,
}

impl Network {
    pub fn from_name(string: &str) -> Self {
        match string {
            "dev" => Self::Devnet,
            "com" => Self::Comnet,
            _ => Self::Mainnet,
        }
    }

    pub fn matches_did(self, did: &IotaDID) -> bool {
        did.network() == self.as_str()
    }

    /// Returns the default node URL of the Tangle network.
    pub fn node_url(self) -> &'static Url {
        match self {
            Self::Mainnet => &*NODE_MAIN,
            Self::Devnet => &*NODE_DEV,
            Self::Comnet => &*NODE_COM,
        }
    }

    /// Returns the web explorer URL of the Tangle network.
    pub fn explorer_url(self) -> &'static Url {
        match self {
            Self::Mainnet => &*EXPLORER_MAIN,
            Self::Devnet => &*EXPLORER_DEV,
            Self::Comnet => &*EXPLORER_COM,
        }
    }

    /// Returns the name of the network as a static `str`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mainnet => "main",
            Self::Devnet => "dev",
            Self::Comnet => "com",
        }
    }
}

impl Default for Network {
    fn default() -> Self {
        Network::Mainnet
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_name() {
        assert_eq!(Network::from_name("com"), Network::Comnet);
        assert_eq!(Network::from_name("dev"), Network::Devnet);
        assert_eq!(Network::from_name("main"), Network::Mainnet);
        assert_eq!(Network::from_name("anything"), Network::Mainnet);
    }

    #[test]
    fn test_matches_did() {
        let did: IotaDID = IotaDID::new(b"").unwrap();
        assert!(Network::matches_did(Network::Mainnet, &did));
        assert!(!Network::matches_did(Network::Comnet, &did));
        assert!(!Network::matches_did(Network::Devnet, &did));

        let did: IotaDID = IotaDID::with_network(b"", "com").unwrap();
        assert!(Network::matches_did(Network::Comnet, &did));
        assert!(!Network::matches_did(Network::Mainnet, &did));
        assert!(!Network::matches_did(Network::Devnet, &did));

        let did: IotaDID = IotaDID::with_network(b"", "dev").unwrap();
        assert!(Network::matches_did(Network::Devnet, &did));
        assert!(!Network::matches_did(Network::Mainnet, &did));
        assert!(!Network::matches_did(Network::Comnet, &did));
    }
}
