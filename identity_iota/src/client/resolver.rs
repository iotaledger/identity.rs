use async_trait::async_trait;
use identity_core::{
    convert::SerdeInto as _,
    did_url::DID,
    error::{Error, Result},
    resolver::{DocumentMetadata, InputMetadata, MetaDocument, ResolverMethod},
};

use crate::{client::Client, did::IotaDID};

#[async_trait(?Send)]
impl ResolverMethod for Client {
    fn is_supported(&self, did: &DID) -> bool {
        match IotaDID::try_from_borrowed(did) {
            Ok(did) => self.check_network(&did).is_ok(),
            Err(_) => false,
        }
    }

    async fn read(&self, did: &DID, _input: InputMetadata) -> Result<Option<MetaDocument>> {
        let did: &IotaDID = IotaDID::try_from_borrowed(did).map_err(err)?;
        let (document, metadata): _ = self.read_document(&did).await.map_err(err)?;

        let mut meta: DocumentMetadata = DocumentMetadata::new();
        meta.created = Some(document.created());
        meta.updated = Some(document.updated());
        meta.properties = metadata;

        Ok(Some(MetaDocument {
            data: document.serde_into()?,
            meta,
        }))
    }
}

fn err(error: crate::error::Error) -> Error {
    Error::ResolutionError(error.into())
}
