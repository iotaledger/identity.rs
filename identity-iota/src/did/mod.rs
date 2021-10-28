// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
mod macros;

#[allow(clippy::module_inception)]
mod did;
mod doc;

pub use self::did::IotaDID;
pub use self::did::IotaDIDUrl;
pub use self::did::Segments;
pub use self::doc::DocumentDiff;
pub use self::doc::IotaDocument;
pub use self::doc::IotaVerificationMethod;
pub use self::doc::Properties;
pub use self::doc::Signer;
pub use self::doc::Verifier;
