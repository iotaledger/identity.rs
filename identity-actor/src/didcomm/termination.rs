// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// A type that can be returned from a hook to indicate that the protocol should terminate immediately.
/// This doesn't include any way to set a cause for the termination, as it is expected that
/// a hook sends a problem report to the peer before returning this type.
pub struct DidCommTermination;
