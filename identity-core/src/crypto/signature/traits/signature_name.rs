// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// A trait for signature suites identified by a particular name.
pub trait SignatureName {
  /// Returns a unique identifier for the signatures created by this suite.
  const NAME: &'static str;
}
