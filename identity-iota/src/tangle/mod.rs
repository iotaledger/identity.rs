// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod client;
mod client_builder;
mod client_map;
mod message_ext;
mod message_index;
mod network;
mod receipt;
mod traits;

pub use self::client::Client;
pub use self::client_builder::ClientBuilder;
pub use self::client_map::ClientMap;
pub use self::message_ext::MessageExt;
pub use self::message_ext::MessageIdExt;
pub use self::message_index::MessageIndex;
pub use self::network::Network;
pub use self::receipt::Receipt;
pub use self::traits::TangleRef;
pub use self::traits::TangleResolve;

#[doc(inline)]
pub use iota_client::bee_message::Message;

#[doc(inline)]
pub use iota_client::bee_message::MessageId;

// Re-export bee_message::Error to use it directly in bindings
#[doc(inline)]
pub use iota_client::bee_message::Error as BeeMessageError;
