// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use identity_client::IotaIdentityClient;
pub use identity_client::IotaIdentityClientExt;

#[cfg(feature = "iota-client")]
pub use self::iota_client::IotaClientExt;

mod identity_client;
#[cfg(feature = "iota-client")]
mod iota_client;
