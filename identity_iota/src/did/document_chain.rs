use std::collections::BTreeMap;

use crate::{
    did::{DocumentDiff, IotaDocument},
    error::{Error, Result},
};

macro_rules! ensure {
  ($expr:expr, $($tt:tt)*) => {
    if !($expr) {
      return Err(Error::ChainError($($tt)*));
    }
  };
}

#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum ChainError {
    #[error("auth chain missing head")]
    AuthChainMissingHead,
    #[error("auth chain missing tail")]
    AuthChainMissingTail,
    #[error("missing latest document")]
    MissingLatestDocument,
    #[error("auth chain is empty")]
    AuthChainEmpty,
    #[error("document missing Tangle message id")]
    DocumentTangleIdMissing,
    #[error("document cannot references previous Tangle message id")]
    DocumentInvalidPreviousId,
    #[error("diff missing Tangle message id")]
    DiffTangleIdMissing,
    #[error("diff has invalid Tangle message id")]
    DiffTangleIdMismatch,
    #[error("document has invalid signature")]
    DocumentSignatureInvalid,
    #[error("diff has invalid signature")]
    DiffSignatureInvalid,
    #[error("auth chain head has invalid message id")]
    AuthChainHeadBadref,
    #[error("diff chain head has invalid message id")]
    DiffChainHeadBadref,
}
use self::ChainError::*;

#[derive(Debug)]
pub struct DocumentChain {
    auth_chain: Vec<IotaDocument>,
    diff_chain: BTreeMap<usize, Vec<DocumentDiff>>,
    latest_doc: Option<IotaDocument>,
}

impl DocumentChain {
    /// Creates a new `DocumentChain`.
    pub fn new() -> Self {
        Self {
            auth_chain: Vec::new(),
            diff_chain: BTreeMap::new(),
            latest_doc: None,
        }
    }

    /// Returns a reference to the latest `IotaDocument` with all applicable
    /// changes applied.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the chain is empty.
    pub fn latest(&self) -> Result<&IotaDocument> {
        self.latest_doc.as_ref().ok_or(Error::ChainError(MissingLatestDocument))
    }

    /// Returns the latest DID document in the auth chain.
    pub fn head(&self) -> Option<&IotaDocument> {
        self.auth_chain.last()
    }

    /// Returns the latest DID document in the auth chain.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the chain is empty.
    pub fn fetch_head(&self) -> Result<&IotaDocument> {
        self.head().ok_or(Error::ChainError(AuthChainMissingHead))
    }

    /// Returns the earliest DID document in the auth chain.
    pub fn tail(&self) -> Option<&IotaDocument> {
        self.auth_chain.first()
    }

    /// Returns the earliest DID document in the auth chain.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the chain is empty.
    pub fn fetch_tail(&self) -> Result<&IotaDocument> {
        self.tail().ok_or(Error::ChainError(AuthChainMissingTail))
    }

    /// Returns the DID Document auth chain as as slice.
    pub fn auth_chain(&self) -> &[IotaDocument] {
        &*self.auth_chain
    }

    /// Returns the DID Document diff chain at the specified index.
    pub fn diff_chain(&self, index: usize) -> Option<&[DocumentDiff]> {
        self.diff_chain.get(&index).map(|chain| &**chain)
    }

    /// Compose the latest DID document from the auth chain head and diff chain
    /// patches.
    pub fn compose(&self) -> Result<IotaDocument> {
        let head: &IotaDocument = self.fetch_head()?;
        let index: usize = self.auth_chain.len();

        let mut target: IotaDocument = head.clone();

        if let Some(diff) = self.diff_chain.get(&index) {
            for diff in diff.iter() {
                target.merge(diff)?;

                if target.verify().is_err() {
                    return Err(Error::ChainError(DocumentSignatureInvalid));
                }
            }
        }

        Ok(target)
    }

    /// Adds a new `IotaDocument` to the chain.
    ///
    /// The `IotaDocument` is expected to be signed and published to the Tangle.
    pub fn push_document(&mut self, document: IotaDocument) -> Result<()> {
        if self.auth_chain.is_empty() {
            ensure!(document.previous_message_id().is_none(), DocumentInvalidPreviousId);
            ensure!(document.message_id().is_some(), DocumentTangleIdMissing);
            ensure!(!document.message_id().unwrap().is_empty(), DocumentTangleIdMissing);
            ensure!(document.verify().is_ok(), DocumentSignatureInvalid);

            self.auth_chain.push(document.clone());
            self.latest_doc = Some(document);
        } else {
            todo!("Handle Auth Push")
        }

        Ok(())
    }

    /// Adds a new `DocumentDiff` to the chain.
    ///
    /// The `DocumentDiff` is expected to be signed and published to the Tangle.
    pub fn push_diff(&mut self, diff: DocumentDiff) -> Result<()> {
        ensure!(!self.auth_chain.is_empty(), AuthChainEmpty);
        ensure!(diff.message_id().is_some(), DiffTangleIdMissing);
        ensure!(!diff.message_id().unwrap().is_empty(), DiffTangleIdMissing);

        let latest: &IotaDocument = self.latest()?;

        ensure!(latest.verify_data(&diff).is_ok(), DiffSignatureInvalid);

        let target: String = self.diff_chain_target()?;

        ensure!(diff.previous_message_id() == target, DiffTangleIdMismatch);

        let mut target: IotaDocument = latest.clone();

        target.merge(&diff)?;

        ensure!(target.verify().is_ok(), DocumentSignatureInvalid);

        self.diff_chain
            .entry(self.auth_chain.len())
            .or_insert_with(Vec::new)
            .push(diff);

        self.latest_doc = Some(target);

        Ok(())
    }

    /// Returns the current index of the document diff chain.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the chain is empty.
    pub fn diff_chain_index(&self) -> Result<usize> {
        ensure!(!self.auth_chain.is_empty(), AuthChainEmpty);

        Ok(self.auth_chain.len())
    }

    /// Returns the Tangle message id of the *current* `DocumentDiff`.
    ///
    /// This is expected to be used as `previous_message_id` when publishing the
    /// *next* `DocumentDiff`.
    pub fn diff_chain_target(&self) -> Result<String> {
        ensure!(!self.auth_chain.is_empty(), AuthChainEmpty);

        let latest: &IotaDocument = self.latest()?;

        let latest_diff: Option<&DocumentDiff> =
            self.diff_chain.get(&self.auth_chain.len()).and_then(|list| list.last());

        if let Some(diff) = latest_diff {
            diff.message_id()
                .filter(|message_id| !message_id.is_empty())
                .map(ToString::to_string)
                .ok_or(Error::ChainError(DiffChainHeadBadref))
        } else {
            latest
                .message_id()
                .filter(|message_id| !message_id.is_empty())
                .map(ToString::to_string)
                .ok_or(Error::ChainError(AuthChainHeadBadref))
        }
    }
}
