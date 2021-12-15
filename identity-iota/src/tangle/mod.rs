// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Re-export bee_message::Error to use it directly in bindings
#[doc(inline)]
pub use iota_client::bee_message::Error as BeeMessageError;
#[doc(inline)]
pub use iota_client::bee_message::Message;
#[doc(inline)]
pub use iota_client::bee_message::MessageId;

pub use self::client::Client;
pub use self::client_builder::ClientBuilder;
pub use self::client_map::ClientMap;
pub use self::explorer::ExplorerUrl;
pub(crate) use self::message::pack_did_message;
pub use self::message::DIDMessageEncoding;
pub use self::message::DIDMessageVersion;
pub use self::message::MessageExt;
pub use self::message::MessageIdExt;
pub use self::message::MessageIndex;
pub use self::message::TryFromMessage;
pub use self::network::Network;
pub use self::network::NetworkName;
pub use self::publish::PublishType;
pub use self::receipt::Receipt;
pub use self::traits::TangleRef;
pub use self::traits::TangleResolve;

mod client;
mod client_builder;
mod client_map;
mod explorer;
mod message;
mod network;
mod publish;
mod receipt;
mod traits;
