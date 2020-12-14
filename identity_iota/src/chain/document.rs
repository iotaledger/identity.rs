use crate::{
    chain::{AuthChain, DiffChain},
    did::{DocumentDiff, IotaDID, IotaDocument},
    error::Result,
    tangle::MessageId,
};

#[derive(Debug)]
pub struct DocumentChain {
    auth_chain: AuthChain,
    diff_chain: DiffChain,
    document: Option<IotaDocument>,
}

impl DocumentChain {
    pub(crate) fn __diff_message_id<'a>(auth: &'a AuthChain, diff: &'a DiffChain) -> &'a MessageId {
        diff.current_message_id().unwrap_or_else(|| auth.current_message_id())
    }

    pub(crate) fn __fold(auth_chain: &AuthChain, diff_chain: &DiffChain) -> Result<IotaDocument> {
        let mut this: IotaDocument = auth_chain.current.clone();

        for diff in diff_chain.iter() {
            this.merge(diff)?;
        }

        Ok(this)
    }

    /// Creates a new `DocumentChain` from given the `AuthChain`.
    pub fn new(auth_chain: AuthChain) -> Self {
        Self {
            auth_chain,
            diff_chain: DiffChain::new(),
            document: None,
        }
    }

    /// Creates a new `DocumentChain` from given the `AuthChain` and `DiffChain`.
    pub fn with_diff_chain(auth_chain: AuthChain, diff_chain: DiffChain) -> Result<Self> {
        let document: Option<IotaDocument> = if diff_chain.is_empty() {
            None
        } else {
            Some(Self::__fold(&auth_chain, &diff_chain)?)
        };

        Ok(Self {
            auth_chain,
            diff_chain,
            document,
        })
    }

    /// Returns a reference to the DID identifying the document chain.
    pub fn id(&self) -> &IotaDID {
        self.auth_chain.current.id()
    }

    /// Returns a reference to the `AuthChain`.
    pub fn auth(&self) -> &AuthChain {
        &self.auth_chain
    }

    /// Returns a mutable reference to the `AuthChain`.
    pub fn auth_mut(&mut self) -> &mut AuthChain {
        &mut self.auth_chain
    }

    /// Returns a reference to the `DiffChain`.
    pub fn diff(&self) -> &DiffChain {
        &self.diff_chain
    }

    /// Returns a mutable reference to the `DiffChain`.
    pub fn diff_mut(&mut self) -> &mut DiffChain {
        &mut self.diff_chain
    }

    pub fn fold(mut self) -> Result<IotaDocument> {
        for diff in self.diff_chain.iter() {
            self.auth_chain.current.merge(diff)?;
        }

        Ok(self.auth_chain.current)
    }

    /// Returns a reference to the latest document in the chain.
    pub fn current(&self) -> &IotaDocument {
        self.document.as_ref().unwrap_or_else(|| self.auth_chain.current())
    }

    /// Returns a mutable reference to the latest document in the chain.
    pub fn current_mut(&mut self) -> &mut IotaDocument {
        if let Some(document) = self.document.as_mut() {
            document
        } else {
            self.auth_chain.current_mut()
        }
    }

    /// Returns the Tangle message Id of the latest auth document.
    pub fn auth_message_id(&self) -> &MessageId {
        self.auth_chain.current_message_id()
    }

    /// Returns the Tangle message Id of the latest diff or auth document.
    pub fn diff_message_id(&self) -> &MessageId {
        Self::__diff_message_id(&self.auth_chain, &self.diff_chain)
    }

    /// Adds a new auth document to the chain.
    ///
    /// # Errors
    ///
    /// Fails if the document is not a valid auth document.
    pub fn try_push_auth(&mut self, document: IotaDocument) -> Result<()> {
        self.auth_chain.try_push(document)?;
        self.diff_chain.clear();

        self.document = None;

        Ok(())
    }

    /// Adds a new diff to the current diff chain.
    ///
    /// # Errors
    ///
    /// Fails if the document diff is invalid.
    pub fn try_push_diff(&mut self, diff: DocumentDiff) -> Result<()> {
        self.diff_chain.check_validity(&self.auth_chain, &diff)?;

        let mut document: IotaDocument = self
            .document
            .take()
            .unwrap_or_else(|| self.auth_chain.current().clone());

        document.merge(&diff)?;

        self.document = Some(document);

        // SAFETY: we performed the necessary validation in `DiffChain::check_validity`.
        unsafe {
            self.diff_chain.push_unchecked(diff);
        }

        Ok(())
    }
}
