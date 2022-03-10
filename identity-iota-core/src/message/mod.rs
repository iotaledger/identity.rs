// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[doc(inline)]
pub use iota_client::bee_message::Error as BeeMessageError;
#[doc(inline)]
pub use iota_client::bee_message::Message;
#[doc(inline)]
pub use iota_client::bee_message::MessageId;

pub use traits::MessageIdExt;

mod traits;
