use core::iter::FromIterator as _;

use crate::network::Network;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeList {
    network: Network,
    nodes: Vec<String>,
}

impl NodeList {
    pub const fn new() -> Self {
        Self::with_network(Network::Devnet)
    }

    pub const fn with_network(network: Network) -> Self {
        Self {
            network,
            nodes: Vec::new(),
        }
    }

    pub fn with_nodes(nodes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            network: Network::Devnet,
            nodes: Vec::from_iter(nodes.into_iter().map(Into::into)),
        }
    }

    pub fn with_network_and_nodes(network: Network, nodes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            network,
            nodes: Vec::from_iter(nodes.into_iter().map(Into::into)),
        }
    }

    pub fn set_network(&mut self, network: Network) {
        self.network = network;
    }

    pub fn set_node(&mut self, node: impl Into<String>) {
        self.nodes.push(node.into());
    }

    pub fn nodes(&self) -> &[String] {
        self.nodes.as_slice()
    }

    pub fn network(&self) -> Network {
        self.network
    }
}

impl<T, U> From<T> for NodeList
where
    T: IntoIterator<Item = U>,
    U: Into<String>,
{
    fn from(other: T) -> Self {
        Self::with_nodes(other)
    }
}
