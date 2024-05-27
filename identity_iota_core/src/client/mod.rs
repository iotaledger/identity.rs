// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use identity_client::IotaIdentityClient;
pub use identity_client::IotaIdentityClientExt;

#[cfg(feature = "iota-client")]
pub use self::iota_client::IotaClientExt;
#[cfg(feature = "iota-client")]
pub use self::kinesis_identity_client::KinesisIotaIdentityClientExt;

mod identity_client;
#[cfg(feature = "iota-client")]
mod iota_client;
#[cfg(feature = "iota-client")]
mod kinesis_identity_client;
