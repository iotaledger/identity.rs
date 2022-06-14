// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Tangle network types.

pub use network::Network;
pub use network::NetworkName;

mod message_id;
mod network;

pub use message_id::Message;
pub use message_id::MessageId;
pub use message_id::MessageIdExt;
