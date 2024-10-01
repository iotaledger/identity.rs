// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::iota_types::base_types::{IotaAddress, TransactionDigest};
use thiserror::Error;

//pub use crate::json_rpc_error::Error as JsonRpcError;

pub type IotaRpcResult<T = ()> = Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Rpc(#[from] jsonrpsee::core::ClientError),
    #[error(transparent)]
    BcsSerialization(#[from] bcs::Error),
    #[error("Subscription error: {0}")]
    Subscription(String),
    #[error("Failed to confirm tx status for {0:?} within {1} seconds.")]
    FailToConfirmTransactionStatus(TransactionDigest, u64),
    #[error("Data error: {0}")]
    Data(String),
    #[error(
        "Client/Server api version mismatch, client api version: {client_version}, server api version: {server_version}"
    )]
    ServerVersionMismatch {
        client_version: String,
        server_version: String,
    },
    #[error("Insufficient fund for address [{address}], requested amount: {amount}")]
    InsufficientFund { address: IotaAddress, amount: u128 },
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}