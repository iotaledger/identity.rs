// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use communication_refactored::firewall::{
  PermissionValue, RequestPermissions, ToPermissionVariants, VariantPermission,
};

pub mod communicator;
pub mod errors;
pub mod handler;
pub mod types;

pub use communicator::{DefaultIdentityHandler, IdentityCommunicator};
pub use errors::{Error, Result};
pub use handler::IdentityStorageHandler;
pub use types::IdentityRequestHandler;
