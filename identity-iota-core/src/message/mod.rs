// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[doc(inline)]
pub use iota_client::bee_message::MessageId;

pub use self::traits::MessageIdExt;
pub use self::traits::TangleRef;

mod traits;
