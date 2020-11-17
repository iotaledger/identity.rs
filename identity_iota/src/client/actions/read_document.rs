use identity_core::{common::Object, convert::FromJson as _};

use crate::{
    client::{Client, ReadTransactionsRequest, ReadTransactionsResponse, TangleMessage},
    did::{IotaDID, IotaDocument},
    error::{Error, Result},
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ReadDocumentResponse {
    pub document: IotaDocument,
    pub metadata: Object,
}

#[derive(Debug)]
pub struct ReadDocumentRequest<'a, 'b> {
    pub(crate) client: &'a Client,
    pub(crate) did: &'b IotaDID<'b>,
    pub(crate) trace: bool,
}

impl<'a, 'b> ReadDocumentRequest<'a, 'b> {
    pub const fn new(client: &'a Client, did: &'b IotaDID<'b>) -> Self {
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
        if self.trace {
            println!("[+] trace(1): Target Id: {:?}", self.did.tag());
            println!("[+] trace(2): Auth Chain: {:?}", self.did.address_hash());
        }

        // Fetch all messages for the auth chain.
        let request: ReadTransactionsRequest = ReadTransactionsRequest::new(self.client, self.did.address()?);
        let response: ReadTransactionsResponse = request.trace(self.trace).send().await?;

        if self.trace {
            println!("[+] trace(3): Tangle Documents: {:?}", response);
        }

        let document: Option<IotaDocument> = self.extract_auth_document(response.messages);
        let document: IotaDocument = document.ok_or(Error::InvalidTransactionBundle)?;

        if self.trace {
            println!("[+] trace(4): Auth Document: {:?}", document);
        }

        if let Some(_address) = document.diff_chain() {
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
            documents.into_iter().partition(|item| item.prev_msg().is_none());

        // Sort documents in ASCENDING order
        initials.sort_by_key(|document| document.created());
        additionals.sort_by_key(|document| document.created());

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
                // Only include documents that deserialize as valid IOTA documents
                .and_then(|json| IotaDocument::from_json(&json).ok())
                // Only include documents matching the target DID
                .filter(|document| self.did.authority() == document.id().authority());

            if let Some(document) = document {
                documents.push(document);
            }
        }

        documents
    }
}
