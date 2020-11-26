use core::fmt::{Debug, Formatter, Result as FmtResult};
use iota::{client::Transfer, transaction::bundled::BundledTransaction};

use crate::{
    client::{Client, TransactionPrinter},
    error::{Error, Result},
};

#[derive(Clone, PartialEq)]
pub struct SendTransferResponse {
    pub tail: BundledTransaction,
}

impl Debug for SendTransferResponse {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.debug_struct("SendTransferResponse")
            .field("tail", &TransactionPrinter::full(&self.tail))
            .finish()
    }
}

#[derive(Debug)]
pub struct SendTransferRequest<'a> {
    pub(crate) client: &'a Client,
    pub(crate) trace: bool,
}

impl<'a> SendTransferRequest<'a> {
    pub const fn new(client: &'a Client) -> Self {
        Self { client, trace: false }
    }

    pub fn trace(mut self, value: bool) -> Self {
        self.trace = value;
        self
    }

    pub async fn send(self, transfer: Transfer) -> Result<SendTransferResponse> {
        if self.trace {
            println!("[+] trace: Sending Transfer: {:?}", transfer.message);
        }

        self.client
            .client
            .send(None)
            .transfers(vec![transfer])
            .send()
            .await?
            .into_iter()
            .find(|transaction| transaction.is_tail())
            .ok_or(Error::InvalidTransferTail)
            .map(|tail| SendTransferResponse { tail })
    }
}
