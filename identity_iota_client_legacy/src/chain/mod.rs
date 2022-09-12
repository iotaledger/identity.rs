// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA DID Integration and Differentiation chains.

pub use self::diff_chain::DiffChain;
pub use self::document_chain::DocumentChain;
pub use self::document_history::ChainHistory;
pub use self::document_history::DocumentHistory;
pub use self::integration_chain::IntegrationChain;

mod diff_chain;
mod document_chain;
mod document_history;
mod integration_chain;
mod milestone;
