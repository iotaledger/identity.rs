use async_trait::async_trait;

use crate::{
    common::Object,
    did::{DIDDocument, DID},
    error::Result,
    resolver::{ErrorKind, Resolution, ResolutionContext, ResolutionInput},
};

pub type Document = (DIDDocument, Object);

#[async_trait]
pub trait IdentityResolver {
    fn is_supported(&self, did: &DID) -> bool;

    async fn document(&self, did: &DID, input: &ResolutionInput) -> Result<Option<Document>>;

    async fn resolve_str(&self, did: &str, input: ResolutionInput) -> Result<Resolution> {
        let mut context: ResolutionContext = ResolutionContext::new();

        self.resolve_str_(did, input, &mut context).await?;

        Ok(context.finish())
    }

    async fn resolve_did(&self, did: &DID, input: ResolutionInput) -> Result<Resolution> {
        let mut context: ResolutionContext = ResolutionContext::new();

        self.resolve_did_(did, input, &mut context).await?;

        Ok(context.finish())
    }

    async fn resolve_str_stream(&self, did: &str, input: ResolutionInput, _buffer: &mut [u8]) -> Result<()> {
        let resolution: Resolution = self.resolve_str(did, input).await?;

        if let Some(_document) = resolution.did_document {
            todo!("Write document to buffer")
        }

        Ok(())
    }

    async fn resolve_did_stream(&self, did: &DID, input: ResolutionInput, _buffer: &mut [u8]) -> Result<()> {
        let resolution: Resolution = self.resolve_did(did, input).await?;

        if let Some(_document) = resolution.did_document {
            todo!("Write document to buffer")
        }

        Ok(())
    }

    async fn resolve_str_(&self, did: &str, input: ResolutionInput, context: &mut ResolutionContext) -> Result<()> {
        // ==
        // 1. Validate that the input DID conforms to the did rule of the DID Syntax
        // ==

        match DID::from(did) {
            Ok(ref did) => {
                self.resolve_did_(did, input, context).await?;
            }
            Err(_) => {
                context.set_error(ErrorKind::InvalidDID);
            }
        }

        Ok(())
    }

    async fn resolve_did_(&self, did: &DID, input: ResolutionInput, context: &mut ResolutionContext) -> Result<()> {
        // ==
        // 2. Determine if the input DID method is supported by the DID resolver
        //    that implements this algorithm.
        // ==

        if self.is_supported(did) {
            // ==
            // 3. Obtain the DID document for the input DID by executing the Read
            //    operation against the input DID's verifiable data registry.
            // ==

            match self.document(did, &input).await? {
                Some((document, metadata)) => {
                    // TODO: Ensure document DID and user-provided DID are consistent

                    // ==
                    // 4-7. Skip - this is all handled by the caller
                    // ==

                    context.set_document(document);
                    context.set_metadata(metadata);
                }
                None => {
                    context.set_error(ErrorKind::NotFound);
                }
            }
        } else {
            context.set_error(ErrorKind::NotSupported);
        }

        Ok(())
    }
}
