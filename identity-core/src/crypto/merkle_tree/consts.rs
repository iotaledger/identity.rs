// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::mem;

pub const PREFIX_L: &[u8] = &[0x00];
pub const PREFIX_B: &[u8] = &[0x01];

pub const SIZE_USIZE: usize = mem::size_of::<usize>();
