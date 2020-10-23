use core::{
    fmt::{Debug, Formatter, Result as FmtResult},
    slice::from_ref,
    time::Duration,
};
use iota::{
    client::{AttachToTangleResponse, GTTAResponse, Transfer},
    crypto::ternary::Hash,
    transaction::bundled::{Bundle, BundledTransaction},
};
use std::{thread, time::Instant};

use crate::{
    client::{Client, PromoteOptions, TransactionPrinter},
    error::{Error, Result},
    utils::{create_address_from_trits, encode_trits, txn_hash, txn_hash_trytes},
};

/// Fixed-address used for faster transaction confirmation times
const PROMOTION: &str = "PROMOTEADDRESSPROMOTEADDRESSPROMOTEADDRESSPROMOTEADDRESSPROMOTEADDRESSPROMOTEADDR";

#[derive(Clone, PartialEq)]
#[repr(transparent)]
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
    pub(crate) promote: Option<PromoteOptions>,
    pub(crate) confirm_time: Option<Duration>,
}

impl<'a> SendTransferRequest<'a> {
    const DEFAULT_CONFIRM_TIME: Duration = Duration::from_secs(5);

    pub const fn new(client: &'a Client) -> Self {
        Self {
            client,
            trace: false,
            promote: Some(PromoteOptions::new()),
            confirm_time: Some(Self::DEFAULT_CONFIRM_TIME),
        }
    }

    pub fn trace(mut self, value: bool) -> Self {
        self.trace = value;
        self
    }

    pub fn promote(mut self, value: impl Into<Option<PromoteOptions>>) -> Self {
        self.promote = value.into();
        self
    }

    pub fn confirm_time(mut self, value: impl Into<Option<Duration>>) -> Self {
        self.confirm_time = value.into();
        self
    }

    pub fn promote_timeout(mut self, value: impl Into<Option<Duration>>) -> Self {
        self.promote = Some(self.promote.unwrap_or_default().timeout(value));
        self
    }

    pub fn promote_interval(mut self, value: Duration) -> Self {
        self.promote = Some(self.promote.unwrap_or_default().interval(value));
        self
    }

    pub fn promote_ts_depth(mut self, value: u8) -> Self {
        self.promote = Some(self.promote.unwrap_or_default().ts_depth(value));
        self
    }

    pub async fn send(self, transfer: Transfer) -> Result<SendTransferResponse> {
        if self.trace {
            println!("[+] trace(1): Sending Transfer: {:?}", transfer.message);
        }

        // Send the transfer to the configured node.
        let response: Vec<BundledTransaction> = self.client.client.send(None).transfers(vec![transfer]).send().await?;

        if self.trace {
            println!("[+] trace(2): Response Transactions: {}", response.len());
        }

        // Extract the tail transaction from the response.
        let tail: BundledTransaction = response
            .into_iter()
            .find(|transaction| transaction.is_tail())
            .ok_or(Error::InvalidTransferTail)?;

        let tail_hash: Hash = txn_hash(&tail);

        if self.trace {
            println!("[+] trace(3): Tail Transaction: {:?}", TransactionPrinter::full(&tail));
        }

        if let Some(delay) = self.confirm_time {
            thread::sleep(delay);
        }

        if self.client.is_transaction_confirmed(&tail_hash).await? {
            if self.trace {
                println!("[+] trace(4): Immediate Confirmation: {}", txn_hash_trytes(&tail));
            }

            Ok(SendTransferResponse { tail })
        } else if let Some(promote) = self.promote {
            self.promote_loop(&tail_hash, &promote).await?;

            if self.trace {
                println!("[+] trace(4): Promoted Confirmation: {}", txn_hash_trytes(&tail));
            }

            Ok(SendTransferResponse { tail })
        } else {
            if self.trace {
                println!("[+] trace(4): Unconfirmable: {}", txn_hash_trytes(&tail));
            }

            Err(Error::TransferUnconfirmable)
        }
    }

    async fn promote_loop(&self, hash: &Hash, options: &PromoteOptions) -> Result<()> {
        let instant: Instant = Instant::now();

        if self.trace {
            println!("[+] trace(3.1): Promoting: {}", encode_trits(hash));
            println!("[+] trace(3.1): Options: {:?}", options);
        }

        loop {
            self.promote_once(hash, options).await?;

            thread::sleep(options.interval);

            if self.client.is_transaction_confirmed(hash).await? {
                break;
            }

            if self.trace {
                if let Some(timeout) = options.timeout {
                    println!("[+] trace(3.2): Elapsed: {:?}/{:?}", instant.elapsed(), timeout);
                } else {
                    println!("[+] trace(3.2): Elapsed: {:?}/???", instant.elapsed());
                }
            }

            if matches!(options.timeout, Some(timeout) if instant.elapsed() >= timeout) {
                return Err(Error::TransferUnconfirmable);
            }
        }

        Ok(())
    }

    async fn promote_once(&self, hash: &Hash, options: &PromoteOptions) -> Result<()> {
        // TODO: Use lazy_static and just clone
        // Create a promotional transfer.
        let transfer: Transfer = Transfer {
            address: create_address_from_trits(PROMOTION)?,
            value: 0,
            message: None,
            tag: None,
        };

        let bundle: Bundle = self
            .client
            .client
            .prepare_transfers(None)
            .transfers(vec![transfer])
            .build()
            .await?;

        // Fetch branch/trunk transaction hashes.
        let tips: GTTAResponse = self
            .client
            .client
            .get_transactions_to_approve()
            .depth(options.ts_depth)
            .send()
            .await?;

        // Construct and dispatch an `attachToTangle` request.
        let trytes: AttachToTangleResponse = self
            .client
            .client
            .attach_to_tangle()
            .trunk_transaction(hash)
            .branch_transaction(&tips.branch_transaction)
            .trytes(from_ref(bundle.tail()))
            .send()
            .await?;

        // Send the trytes to the configured node.
        let _: () = self.client.client.broadcast_transactions(&trytes.trytes).await?;

        Ok(())
    }
}
