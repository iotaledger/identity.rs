use core::ops::Deref;
use identity_core::{common::Object, convert::FromJson as _};

use crate::{
    client::{Client, ReadTransactionsRequest, ReadTransactionsResponse, TangleMessage},
    did::{IotaDID, IotaDocument},
    error::{Error, Result},
    utils::{create_address_from_trits, encode_trits},
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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
        if self.trace {
            println!("[+] trace(1): Target Id: {:?}", self.did.tag());
            println!("[+] trace(2): Auth Chain: {:?}", self.did.address());
        }

        // Fetch all messages for the auth chain.
        let request: ReadTransactionsRequest =
            ReadTransactionsRequest::new(self.client, create_address_from_trits(self.did.address())?);
        let response: ReadTransactionsResponse = request.trace(self.trace).send().await?;

        if self.trace {
            println!("[+] trace(3): Tangle Documents: {:?}", response);
        }

        let document: Option<ChainDocument> = self.extract_auth_document(response.messages);
        let document: ChainDocument = document.ok_or(Error::InvalidTransactionBundle)?;

        if self.trace {
            println!("[+] trace(4): Auth Document: {:?}", document.document);
        }

        if !document.immutable() {
            todo!("Handle Document Diff Chain")
        }

        Ok(ReadDocumentResponse {
            document: document.document,
            metadata: Object::new(),
        })
    }

    fn extract_auth_document(&self, messages: Vec<TangleMessage>) -> Option<ChainDocument> {
        let (mut docs1, mut docs2): (Vec<_>, Vec<_>) = messages
            .into_iter()
            .flat_map(|message| ChainDocument::new(self.did, message))
            .partition(ChainDocument::is_initial_document);

        // Sort documents in ASCENDING order.
        docs1.sort_by_key(|document| document.created());
        docs2.sort_by_key(|document| document.created());

        // Find the first initial document with a valid signature.
        let mut target: ChainDocument = docs1.into_iter().find(|item| item.verify().is_ok())?;

        // Follow the chain of successive documents - AKA the auth chain.
        for maybe in docs2 {
            let hash: &str = maybe.document.previous_message_id()?;

            // Ignore documents that don't reference the expected Tangle message.
            if hash != target.transaction_hash() {
                continue;
            }

            // Ignore documents with invalid signatures.
            if target.verify_data(&*maybe).is_err() {
                continue;
            }

            target = maybe;
        }

        Some(target)
    }
}

struct ChainDocument {
    document: IotaDocument,
    message: TangleMessage,
}

impl ChainDocument {
    fn is_initial_document(&self) -> bool {
        self.document.previous_message_id().is_none()
    }

    fn transaction_hash(&self) -> String {
        encode_trits(&self.message.tail_hash)
    }

    fn new(did: &IotaDID, message: TangleMessage) -> Option<Self> {
        // Convert the Tangle message content to a UTF8 string.
        let json: String = message.message_utf8().ok()?;

        // Deserialize the message; ignore any documents that fail.
        let document: IotaDocument = IotaDocument::from_json(&json).ok()?;

        // Ignore any documents that don't belong to that target DID.
        if did.authority() != document.id().authority() {
            return None;
        }

        Some(Self { document, message })
    }
}

impl Deref for ChainDocument {
    type Target = IotaDocument;

    fn deref(&self) -> &Self::Target {
        &self.document
    }
}
