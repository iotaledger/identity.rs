use crate::{
    chain::{AuthChain, DiffChain},
    did::IotaDocument,
    error::Result,
};

#[derive(Debug)]
pub struct DocumentChain {
    auth_chain: AuthChain,
    diff_chain: DiffChain,
}

impl DocumentChain {
    pub fn new(auth_chain: AuthChain, diff_chain: DiffChain) -> Self {
        Self { auth_chain, diff_chain }
    }

    pub fn auth(&self) -> &AuthChain {
        &self.auth_chain
    }

    pub fn auth_mut(&mut self) -> &mut AuthChain {
        &mut self.auth_chain
    }

    pub fn diff(&self) -> &DiffChain {
        &self.diff_chain
    }

    pub fn diff_mut(&mut self) -> &mut DiffChain {
        &mut self.diff_chain
    }

    pub fn fold(mut self) -> Result<IotaDocument> {
        for diff in self.diff_chain.iter() {
            self.auth_chain.current.merge(diff)?;
        }

        Ok(self.auth_chain.current)
    }
}
