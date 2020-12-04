use identity_core::{
    convert::{AsJson as _, SerdeInto as _},
    did_doc::{Document, SetSignature, Signature, TrySignature, TrySignatureMut},
    identity_diff::{did_doc::DiffDocument, Diff},
};

use crate::{
    client::{Client, Network},
    did::{IotaDID, IotaDocument},
    error::Result,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DocumentDiff {
    pub(crate) did: IotaDID,
    pub(crate) diff: String,
    pub(crate) previous_message_id: String,
    pub(crate) proof: Option<Signature>,
    #[serde(skip)]
    pub(crate) message_id: Option<String>,
}

impl DocumentDiff {
    pub fn new(current: &IotaDocument, updated: &IotaDocument, previous_message_id: String) -> Result<Self> {
        let a: Document = current.serde_into()?;
        let b: Document = updated.serde_into()?;
        let diff: String = Diff::diff(&a, &b)?.to_json()?;

        Ok(Self {
            did: current.id().clone(),
            previous_message_id,
            diff,
            proof: None,
            message_id: None,
        })
    }

    /// Returns the DID of associated document.
    pub fn did(&self) -> &IotaDID {
        &self.did
    }

    /// Returns the raw contents of the DID document diff.
    pub fn diff(&self) -> &str {
        &*self.diff
    }

    /// Returns the Tangle message id of the previous DID document diff.
    pub fn previous_message_id(&self) -> &str {
        &*self.previous_message_id
    }

    /// Returns a reference to the `DocumentDiff` proof.
    pub fn proof(&self) -> Option<&Signature> {
        self.proof.as_ref()
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

    /// Publishes the `DocumentDiff` to the Tangle using a default `Client`.
    pub async fn publish(&mut self, index: usize) -> Result<()> {
        let network: Network = Network::from_str(self.did.network());
        let client: Client = Client::from_network(network)?;

        self.publish_with_client(&client, index).await
    }

    /// Publishes the `DocumentDiff` to the Tangle using the provided `Client`.
    pub async fn publish_with_client(&mut self, client: &Client, index: usize) -> Result<()> {
        let transaction: _ = client.publish_diff(self, index).await?;
        let message_id: String = client.transaction_hash(&transaction);

        self.set_message_id(message_id);

        Ok(())
    }

    pub(crate) fn merge(&self, document: &IotaDocument) -> Result<IotaDocument> {
        let data: DiffDocument = DiffDocument::from_json(&self.diff)?;

        document
            .serde_into()
            .and_then(|this: Document| Diff::merge(&this, data).map_err(Into::into))
            .and_then(|this: Document| this.serde_into())
            .map_err(Into::into)
    }
}

impl TrySignature for DocumentDiff {
    fn try_signature(&self) -> Option<&Signature> {
        self.proof.as_ref()
    }
}

impl TrySignatureMut for DocumentDiff {
    fn try_signature_mut(&mut self) -> Option<&mut Signature> {
        self.proof.as_mut()
    }
}

impl SetSignature for DocumentDiff {
    fn set_signature(&mut self, value: Signature) {
        self.proof = Some(value);
    }
}
