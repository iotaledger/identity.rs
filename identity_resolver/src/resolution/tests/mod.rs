// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::resolver::*;
mod resolution_errors;
mod send_sync;

#[cfg(feature = "iota")]
mod successful_presentation_validation;
#[cfg(feature = "iota")]
mod valid_presentation_data;

#[cfg(feature = "iota")]
mod presentation_validation_errors;
