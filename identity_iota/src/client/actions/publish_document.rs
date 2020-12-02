use core::fmt::{Debug, Formatter, Result as FmtResult};
use identity_core::convert::ToJson as _;
use iota::{client::Transfer, transaction::bundled::BundledTransaction};

use crate::{
    client::{Client, TransactionPrinter},
    did::{IotaDID, IotaDocument},
    error::{Error, Result},
};

#[derive(Clone, PartialEq)]
#[repr(transparent)]
pub struct PublishDocumentResponse {
    pub tail: BundledTransaction,
}

impl Debug for PublishDocumentResponse {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.debug_struct("PublishDocumentResponse")
            .field("tail", &TransactionPrinter::full(&self.tail))
            .finish()
    }
}

#[derive(Debug)]
pub struct PublishDocumentRequest<'a, 'b> {
    pub(crate) client: &'a Client,
    pub(crate) document: &'b IotaDocument,
}

impl<'a, 'b> PublishDocumentRequest<'a, 'b> {
    pub const fn new(client: &'a Client, document: &'b IotaDocument) -> Self {
        Self { client, document }
    }

    pub async fn send(self) -> Result<PublishDocumentResponse> {
        let did: &IotaDID = self.document.id();

        trace!("Publish Document with DID: {}", did);

        // Ensure the correct network is selected.
        if !self.client.network.matches_did(&did) {
            return Err(Error::InvalidDIDNetwork);
        }

        trace!("Authentication: {:?}", self.document.authentication());
        trace!("Tangle Address: {}", did.address_hash());

        // Create a transfer to publish the DID document at the specified address.
        let transfer: Transfer = Transfer {
            address: did.address()?,
            value: 0,
            message: Some(self.document.to_json()?),
            tag: None,
        };

        // Submit the transfer to the tangle.
        self.client
            .send_transfer(transfer)
            .await
            .map(|tail| PublishDocumentResponse { tail })
    }
}
