// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use self::message_encoding::DIDMessageEncoding;
pub(crate) use self::message_ext::pack_did_message;
pub use self::message_ext::MessageExt;
pub use self::message_ext::TryFromMessage;
pub use self::message_index::MessageIndex;
pub use self::message_version::DIDMessageVersion;

mod compression_brotli;
mod message_encoding;
mod message_ext;
mod message_index;
mod message_version;
