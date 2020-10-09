use async_trait::async_trait;
use identity_core::{
    deref::IdentityDeref,
    did::DID,
    error::Result,
    resolver::{IdentityResolver as _, Resolution, ResolutionInput},
};

use crate::resolver::TangleResolver;

#[derive(Debug)]
pub struct TangleDeref {
    resolver: TangleResolver,
}

impl TangleDeref {
    pub const fn new() -> Self {
        Self {
            resolver: TangleResolver::new(),
        }
    }

    pub fn with_resolver(resolver: TangleResolver) -> Self {
        Self { resolver }
    }

    pub fn resolve(&self) -> &TangleResolver {
        &self.resolver
    }

    pub fn resolve_mut(&mut self) -> &mut TangleResolver {
        &mut self.resolver
    }
}

#[async_trait]
impl IdentityDeref for TangleDeref {
    async fn resolve(&self, did: &DID, input: ResolutionInput) -> Result<Resolution> {
        self.resolver.resolve_did(did, input).await.map_err(Into::into)
    }
}
