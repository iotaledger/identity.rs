// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod borrow_asset;
mod config;
mod controller_execution;
mod create;
mod deactivate;
pub(crate) mod proposal;
mod send_asset;
mod update;
mod upgrade;

pub(crate) use borrow_asset::*;
pub(crate) use config::*;
pub(crate) use controller_execution::*;
pub(crate) use create::*;
pub(crate) use deactivate::*;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder as Ptb;
use iota_sdk::types::transaction::Argument;
pub(crate) use send_asset::*;
pub(crate) use update::*;
pub(crate) use upgrade::*;

struct ProposalContext {
  ptb: Ptb,
  controller_cap: Argument,
  delegation_token: Argument,
  borrow: Argument,
  identity: Argument,
  proposal_id: Argument,
}
