use core::fmt::{Debug, Formatter, Result as FmtResult};
use identity_core::common::ToJson as _;
use iota::{
    client::Transfer,
    transaction::bundled::{Address, BundledTransaction, BundledTransactionField as _},
};

use crate::{
    client::{SendTransferRequest, SendTransferResponse, TransactionPrinter},
    did::{IotaDID, IotaDocument},
    error::{Error, Result},
    utils::encode_trits,
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
    pub(crate) transfer: SendTransferRequest<'a>,
    pub(crate) document: &'b IotaDocument,
}

impl<'a, 'b> PublishDocumentRequest<'a, 'b> {
    pub const fn new(transfer: SendTransferRequest<'a>, document: &'b IotaDocument) -> Self {
        Self { transfer, document }
    }

    pub fn trace(mut self, value: bool) -> Self {
        self.transfer = self.transfer.trace(value);
        self
    }

    pub async fn send(self) -> Result<PublishDocumentResponse> {
        let did: &IotaDID = self.document.did();

        if self.transfer.trace {
            println!("[+] trace(1): Create Document with DID: {:?}", did);
        }

        // Ensure the correct network is selected.
        if !self.transfer.client.network.matches_did(&did) {
            return Err(Error::InvalidDIDNetwork);
        }

        if self.transfer.trace {
            println!(
                "[+] trace(2): Authentication Method: {:?}",
                self.document.authentication_key()
            );
        }

        // Verify the document signature with the authentication key.
        self.document.verify()?;

        // Create a tangle address from the DID.
        let address: Address = did.create_address()?;

        if self.transfer.trace {
            println!("[+] trace(3): Tangle Address: {:?}", encode_trits(address.to_inner()));
        }

        // Create a transfer to publish the DID document at the specified address.
        let transfer: Transfer = Transfer {
            address,
            value: 0,
            message: Some(self.document.to_json()?),
            tag: None,
        };

        // Submit the transfer to the tangle.
        let response: SendTransferResponse = self.transfer.send(transfer).await?;

        Ok(PublishDocumentResponse { tail: response.tail })
    }
}
