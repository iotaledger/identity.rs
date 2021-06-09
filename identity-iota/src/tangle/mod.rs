// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod client;
mod client_builder;
mod message_ext;
mod message_index;
mod network;
mod traits;

pub use self::client::Client;
pub use self::client_builder::ClientBuilder;
pub use self::message_ext::MessageExt;
pub use self::message_ext::MessageIdExt;
pub use self::message_index::MessageIndex;
pub use self::network::Network;
pub use self::traits::TangleRef;

#[doc(inline)]
pub use iota_client::bee_message::Message;

#[doc(inline)]
pub use iota_client::bee_message::MessageId;
