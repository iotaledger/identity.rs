// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use identity_client::StardustIdentityClient;
pub use identity_client::StardustIdentityClientExt;

#[cfg(feature = "iota-client")]
pub use self::iota_client::StardustClientExt;

mod identity_client;
#[cfg(feature = "iota-client")]
mod iota_client;
