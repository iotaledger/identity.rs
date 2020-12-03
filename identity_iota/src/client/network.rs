use identity_core::common::Url;
use iota::{client::builder, transaction::bundled::BundledTransaction};

use crate::{client::TransactionPrinter, did::IotaDID};

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
    pub fn from_str(string: &str) -> Self {
        match string {
            "dev" => Self::Devnet,
            "com" => Self::Comnet,
            _ => Self::Mainnet,
        }
    }

    pub fn matches_did(self, did: &IotaDID) -> bool {
        did.network() == self.as_str() || self == Self::Mainnet
    }

    pub fn transaction_url(&self, transaction: &BundledTransaction) -> Url {
        let hash: TransactionPrinter<_> = TransactionPrinter::hash(transaction);

        let mut url: Url = self.explorer_url().clone();

        url.path_segments_mut()
            .unwrap()
            .push("transaction")
            .push(&hash.to_string());

        url
    }

    pub fn explorer_url(self) -> &'static Url {
        match self {
            Self::Mainnet => &*EXPLORER_MAIN,
            Self::Devnet => &*EXPLORER_DEV,
            Self::Comnet => &*EXPLORER_COM,
        }
    }

    pub fn node_url(self) -> &'static Url {
        match self {
            Self::Mainnet => &*NODE_MAIN,
            Self::Devnet => &*NODE_DEV,
            Self::Comnet => &*NODE_COM,
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Mainnet => "main",
            Self::Devnet => "dev",
            Self::Comnet => "com",
        }
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
