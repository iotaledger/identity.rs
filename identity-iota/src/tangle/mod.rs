// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod message_ext;
mod message_index;
mod traits;

pub use self::message_ext::MessageExt;
pub use self::message_ext::MessageIdExt;
pub use self::message_index::MessageIndex;
pub use self::traits::TangleRef;

#[doc(inline)]
pub use iota_client::bee_message::MessageId;
