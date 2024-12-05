// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;
use core::ops::Deref;
use core::fmt;
use std::borrow::Borrow;

use ref_cast::RefCast;

use serde::Deserialize;
use serde::Serialize;

use anyhow::{bail, Result};


/// Return true if this character can appear in a Move identifier.
///
/// Note: there are stricter restrictions on whether a character can begin a
/// Move identifier--only alphabetic characters are allowed here.
#[inline]
pub const fn is_valid_identifier_char(c: char) -> bool {
    matches!(c, '_' | 'a'..='z' | 'A'..='Z' | '0'..='9')
}

/// Returns `true` if all bytes in `b` after the offset `start_offset` are valid
/// ASCII identifier characters.
const fn all_bytes_valid(b: &[u8], start_offset: usize) -> bool {
    let mut i = start_offset;
    // TODO(philiphayes): use for loop instead of while loop when it's stable in
    // const fn's.
    while i < b.len() {
        if !is_valid_identifier_char(b[i] as char) {
            return false;
        }
        i += 1;
    }
    true
}

/// Describes what identifiers are allowed.
///
/// For now this is deliberately restrictive -- we would like to evolve this in
/// the future.
// TODO: "<SELF>" is coded as an exception. It should be removed once
// CompiledScript goes away. Note: needs to be pub as it's used in the
// `ident_str!` macro.
pub const fn is_valid(s: &str) -> bool {
    // Rust const fn's don't currently support slicing or indexing &str's, so we
    // have to operate on the underlying byte slice. This is not a problem as
    // valid identifiers are (currently) ASCII-only.
    let b = s.as_bytes();
    match b {
        b"<SELF>" => true,
        [b'a'..=b'z', ..] | [b'A'..=b'Z', ..] => all_bytes_valid(b, 1),
        [b'_', ..] if b.len() > 1 => all_bytes_valid(b, 1),
        _ => false,
    }
}

/// An owned identifier.
///
/// For more details, see the module level documentation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Identifier(Box<str>);
// An identifier cannot be mutated so use Box<str> instead of String -- it is 1
// word smaller.

impl Identifier {
    /// Creates a new `Identifier` instance.
    pub fn new(s: impl Into<Box<str>>) -> Result<Self> {
        let s = s.into();
        if Self::is_valid(&s) {
            Ok(Self(s))
        } else {
            bail!("Invalid identifier '{}'", s);
        }
    }

    /// Returns true if this string is a valid identifier.
    pub fn is_valid(s: impl AsRef<str>) -> bool {
        is_valid(s.as_ref())
    }

    /// Returns if this identifier is `<SELF>`.
    /// TODO: remove once we fully separate CompiledScript & CompiledModule.
    pub fn is_self(&self) -> bool {
        &*self.0 == "<SELF>"
    }

    /// Converts a vector of bytes to an `Identifier`.
    pub fn from_utf8(vec: Vec<u8>) -> Result<Self> {
        let s = String::from_utf8(vec)?;
        Self::new(s)
    }

    /// Creates a borrowed version of `self`.
    pub fn as_ident_str(&self) -> &IdentStr {
        self
    }

    /// Converts this `Identifier` into a `String`.
    ///
    /// This is not implemented as a `From` trait to discourage automatic
    /// conversions -- these conversions should not typically happen.
    pub fn into_string(self) -> String {
        self.0.into()
    }

    /// Converts this `Identifier` into a UTF-8-encoded byte sequence.
    pub fn into_bytes(self) -> Vec<u8> {
        self.into_string().into_bytes()
    }
}

impl FromStr for Identifier {
    type Err = anyhow::Error;

    fn from_str(data: &str) -> Result<Self> {
        Self::new(data)
    }
}

impl From<&IdentStr> for Identifier {
    fn from(ident_str: &IdentStr) -> Self {
        ident_str.to_owned()
    }
}

impl AsRef<IdentStr> for Identifier {
    fn as_ref(&self) -> &IdentStr {
        self
    }
}

impl Deref for Identifier {
    type Target = IdentStr;

    fn deref(&self) -> &IdentStr {
        // Identifier and IdentStr maintain the same invariants, so it is safe to
        // convert.
        IdentStr::ref_cast(&self.0)
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// A borrowed identifier.
///
/// For more details, see the module level documentation.
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd, RefCast)]
#[repr(transparent)]
pub struct IdentStr(str);

impl IdentStr {
    pub fn new(s: &str) -> Result<&IdentStr> {
        if Self::is_valid(s) {
            Ok(IdentStr::ref_cast(s))
        } else {
            bail!("Invalid identifier '{}'", s);
        }
    }

    /// Returns true if this string is a valid identifier.
    pub fn is_valid(s: impl AsRef<str>) -> bool {
        is_valid(s.as_ref())
    }

    /// Returns the length of `self` in bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if `self` has a length of zero bytes.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Converts `self` to a `&str`.
    ///
    /// This is not implemented as a `From` trait to discourage automatic
    /// conversions -- these conversions should not typically happen.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts `self` to a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Borrow<IdentStr> for Identifier {
    fn borrow(&self) -> &IdentStr {
        self
    }
}

impl ToOwned for IdentStr {
    type Owned = Identifier;

    fn to_owned(&self) -> Identifier {
        Identifier(self.0.into())
    }
}

impl fmt::Display for IdentStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}