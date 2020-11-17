use crate::{
    client::{Client, Network},
    error::Result,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ClientBuilder {
    pub(crate) network: Network,
    pub(crate) nodes: Vec<String>,
}

impl ClientBuilder {
    pub const fn new() -> Self {
        Self {
            network: Network::Mainnet,
            nodes: Vec::new(),
        }
    }

    pub fn network(mut self, network: Network) -> Self {
        self.network = network;
        self
    }

    pub fn node(mut self, node: impl Into<String>) -> Self {
        self.nodes.push(node.into());
        self
    }

    pub fn nodes(mut self, nodes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.nodes.extend(nodes.into_iter().map(Into::into));
        self
    }

    pub fn build(self) -> Result<Client> {
        Client::from_builder(self)
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
