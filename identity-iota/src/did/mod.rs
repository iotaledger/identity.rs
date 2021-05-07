// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
mod macros;

mod doc;
mod url;

pub use self::doc::DocumentDiff;
pub use self::doc::IotaDocument;
pub use self::doc::IotaVerificationMethod;
pub use self::doc::Properties;
pub use self::doc::Signer;
pub use self::doc::Verifier;
pub use self::url::IotaDID;
pub use self::url::Segments;
