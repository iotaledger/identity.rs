use identity_core::{
    convert::{SerdeInto as _, ToJson as _},
    did_doc::{Document, SetSignature, Signature, TrySignature, TrySignatureMut},
    identity_diff::Diff,
};

use crate::{
    client::{Client, ClientBuilder, Network},
    did::{IotaDID, IotaDocument},
    error::Result,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DIDDiff {
    pub did: IotaDID,
    pub prev_msg: String,
    pub diff: String,
    pub proof: Option<Signature>,
    #[serde(skip)]
    pub message_id: Option<String>,
}

impl DIDDiff {
    pub fn new(document: &IotaDocument, other: &IotaDocument, prev_msg: String) -> Result<Self> {
        let a: Document = document.serde_into()?;
        let b: Document = other.serde_into()?;
        let diff: String = Diff::diff(&a, &b)?.to_json()?;

        Ok(Self {
            did: document.id().clone(),
            prev_msg,
            diff,
            proof: None,
            message_id: None,
        })
    }

    /// Returns the Tangle message id of the published DID diff, if any.
    pub fn message_id(&self) -> Option<&str> {
        self.message_id.as_deref()
    }

    // Sets the Tangle message id the published DID diff.
    pub fn set_message_id<T>(&mut self, value: T)
    where
        T: Into<String>,
    {
        self.message_id = Some(value.into());
    }

    pub async fn publish(&mut self, index: usize) -> Result<()> {
        let network: Network = Network::from_str(self.did.network());

        let client: Client = ClientBuilder::new()
            .node(network.node_url().as_str())
            .network(network)
            .build()?;

        self.publish_with_client(index, &client).await
    }

    pub async fn publish_with_client(&mut self, index: usize, client: &Client) -> Result<()> {
        let transaction: _ = client.publish_diff(&*self, index).await?;
        let message_id: String = client.transaction_hash(&transaction);

        self.set_message_id(message_id);

        Ok(())
    }
}

impl TrySignature for DIDDiff {
    fn try_signature(&self) -> Option<&Signature> {
        self.proof.as_ref()
    }
}

impl TrySignatureMut for DIDDiff {
    fn try_signature_mut(&mut self) -> Option<&mut Signature> {
        self.proof.as_mut()
    }
}

impl SetSignature for DIDDiff {
    fn set_signature(&mut self, value: Signature) {
        self.proof = Some(value);
    }
}
