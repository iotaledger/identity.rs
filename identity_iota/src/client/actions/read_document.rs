use identity_core::common::{FromJson as _, Object};
use iota::transaction::bundled::{Address, BundledTransactionField as _};

use crate::{
    client::{Client, ReadTransactionsRequest, ReadTransactionsResponse, TangleMessage},
    did::{IotaDID, IotaDocument},
    error::{Error, Result},
    utils::encode_trits,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ReadDocumentResponse {
    pub document: IotaDocument,
    pub metadata: Object,
}

#[derive(Debug)]
pub struct ReadDocumentRequest<'a, 'b> {
    pub(crate) client: &'a Client,
    pub(crate) did: &'b IotaDID,
    pub(crate) trace: bool,
}

impl<'a, 'b> ReadDocumentRequest<'a, 'b> {
    pub const fn new(client: &'a Client, did: &'b IotaDID) -> Self {
        Self {
            client,
            did,
            trace: false,
        }
    }

    pub fn trace(mut self, value: bool) -> Self {
        self.trace = value;
        self
    }

    pub async fn send(self) -> Result<ReadDocumentResponse> {
        let target_id: &str = self.did.method_id();

        if self.trace {
            println!("[+] trace(1): Target Id: {:?}", target_id);
        }

        // Create a document tangle address from the DID.
        let address: Address = self.did.create_address()?;

        if self.trace {
            println!(
                "[+] trace(2): AuthChain Address: {:?}",
                encode_trits(address.to_inner())
            );
        }

        // Fetch all messages for the auth chain.
        let request: ReadTransactionsRequest = ReadTransactionsRequest::new(self.client, address);
        let response: ReadTransactionsResponse = request.trace(self.trace).send().await?;

        if self.trace {
            println!("[+] trace(3): Tangle Documents: {:?}", response);
        }

        let document: Option<IotaDocument> = self.extract_auth_document(response.messages);
        let document: IotaDocument = document.ok_or(Error::InvalidTransactionBundle)?;

        if self.trace {
            println!("[+] trace(4): Auth Document: {:?}", document);
        }

        if document.has_diff_chain() {
            todo!("Handle Document Diff Chain")
        }

        Ok(ReadDocumentResponse {
            document,
            metadata: Object::new(),
        })
    }

    fn extract_auth_document(&self, messages: Vec<TangleMessage>) -> Option<IotaDocument> {
        let documents: Vec<IotaDocument> = self.extract_auth_chain(messages);

        let (mut initials, mut additionals): (Vec<IotaDocument>, Vec<IotaDocument>) =
            documents.into_iter().partition(|item| item.supersedes().is_none());

        // Sort documents in ASCENDING order
        initials.sort_by(|a, b| a.created.cmp(&b.created));
        additionals.sort_by(|a, b| a.created.cmp(&b.created));

        // Find the first initial document with a valid signature
        let initial: IotaDocument = initials.into_iter().find(|item| item.verify().is_ok())?;

        if !additionals.is_empty() {
            todo!("Handle Document Succession")
        }

        Some(initial)
    }

    fn extract_auth_chain(&self, messages: Vec<TangleMessage>) -> Vec<IotaDocument> {
        let mut documents: Vec<IotaDocument> = Vec::with_capacity(messages.len());

        for message in messages {
            let document: Option<IotaDocument> = message
                .message_utf8()
                .ok()
                .and_then(|json| IotaDocument::from_json(&json).ok());

            if let Some(document) = document {
                // Only include documents matching the target DID
                if self.did != document.did() {
                    continue;
                }

                // Only include documents with valid timestamps
                if document.created.is_none() || document.updated.is_none() {
                    continue;
                }

                documents.push(document);
            }
        }

        documents
    }
}
