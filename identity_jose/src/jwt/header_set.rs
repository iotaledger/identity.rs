// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::ops::Deref;
use url::Url;

use crate::error::Error;
use crate::error::Result;
use crate::jwk::Jwk;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsHeader;
use crate::jwt::JwtHeader;

macro_rules! accessor {
  ($claim:ident, $this:expr) => {{
    accessor!(@impl, and_then, $claim, $this.header, $this.unprotected, $this.protected)
  }};
  (@unwrapped, $claim:ident, $this:expr) => {{
    accessor!(@impl, map, $claim, $this.header, $this.unprotected, $this.protected)
  }};
  (@protected, $claim:ident, $this:expr) => {{
    accessor!(@impl, and_then, $claim, $this.protected, $this.header, $this.unprotected)
  }};
  (@impl, $next:ident, $claim:ident, $a:expr, $b:expr, $c:expr) => {{
    $a
      .$next(|header| header.$claim())
      .or_else(|| $b.$next(|header| header.$claim()))
      .or_else(|| $c.$next(|header| header.$claim()))
  }};
}

macro_rules! impl_accessors {
  ($fn:ident, $try_fn:ident, $ty:ty) => {
    pub fn $fn(&self) -> Option<$ty> {
      accessor!($fn, self)
    }

    pub fn $try_fn(&self) -> Result<$ty> {
      self.$fn().ok_or(Error::MissingParam(stringify!($fn)))
    }
  };
  (@unwrapped, $fn:ident, $try_fn:ident, $ty:ty) => {
    pub fn $fn(&self) -> Option<$ty> {
      accessor!(@unwrapped, $fn, self)
    }

    pub fn $try_fn(&self) -> Result<$ty> {
      self.$fn().ok_or(Error::MissingParam(stringify!($fn)))
    }
  };
  (@protected, $fn:ident, $try_fn:ident, $ty:ty) => {
    pub fn $fn(&self) -> Option<$ty> {
      accessor!(@protected, $fn, self)
    }

    pub fn $try_fn(&self) -> Result<$ty> {
      self.$fn().ok_or(Error::MissingParam(stringify!($fn)))
    }
  };
}

#[derive(Debug)]
pub struct JwtHeaderSet<'a, T> {
  protected: Option<&'a T>,
  unprotected: Option<&'a T>,
  header: Option<&'a T>,
}

impl<'a, T> JwtHeaderSet<'a, T> {
  pub const fn new() -> Self {
    Self {
      protected: None,
      unprotected: None,
      header: None,
    }
  }

  /// Set the protected header.
  pub fn with_protected(mut self, value: impl Into<Option<&'a T>>) -> Self {
    self.protected = value.into();
    self
  }

  /// Get the protected header if it is set.
  pub fn protected(&self) -> Option<&'a T> {
    self.protected
  }

  /// Set the unprotected header.
  pub fn with_unprotected(mut self, value: impl Into<Option<&'a T>>) -> Self {
    self.unprotected = value.into();
    self
  }

  /// Get the unprotected header if it is set.
  pub fn unprotected(&self) -> Option<&'a T> {
    self.unprotected
  }

  // TODO: When is this used?
  pub fn header(mut self, value: impl Into<Option<&'a T>>) -> Self {
    self.header = value.into();
    self
  }
}

#[rustfmt::skip]
impl<'a, T: 'a> JwtHeaderSet<'a, T>
where
  T: Deref<Target = JwtHeader>,
{
  impl_accessors!(jku, try_jku, &Url);
  impl_accessors!(jwk, try_jwk, &Jwk);
  impl_accessors!(kid, try_kid, &str);
  impl_accessors!(x5u, try_x5u, &Url);
  impl_accessors!(x5c, try_x5c, &[String]);
  impl_accessors!(x5t, try_x5t, &str);
  impl_accessors!(x5t_s256, try_x5t_s256, &str);
  impl_accessors!(typ, try_typ, &str);
  impl_accessors!(cty, try_cty, &str);
  impl_accessors!(crit, try_crit, &[String]);
  impl_accessors!(url, try_url, &Url);
  impl_accessors!(nonce, try_nonce, &str);
}

#[rustfmt::skip]
impl<'a> JwtHeaderSet<'a, JwsHeader> {
  impl_accessors!(@protected, alg, try_alg, JwsAlgorithm);
  impl_accessors!(@protected, b64, try_b64, bool);
  impl_accessors!(@protected, ppt, try_ppt, &str);
}

// #[rustfmt::skip]
// impl<'a> JwtHeaderSet<'a, JweHeader> {
//   impl_accessors!(@unwrapped, alg, try_alg, JweAlgorithm);
//   impl_accessors!(@unwrapped, enc, try_enc, JweEncryption);
//   impl_accessors!(zip, try_zip, JweCompression);
//   impl_accessors!(epk, try_epk, &Jwk);
//   impl_accessors!(@protected, apu, try_apu, &str);
//   impl_accessors!(@protected, apv, try_apv, &str);
//   impl_accessors!(@protected, iv, try_iv, &str);
//   impl_accessors!(@protected, tag, try_tag, &str);
//   impl_accessors!(@protected, p2s, try_p2s, &str);
//   impl_accessors!(@protected, p2c, try_p2c, u64);
// }

impl<'a, T: 'a> Default for JwtHeaderSet<'a, T> {
  fn default() -> Self {
    Self::new()
  }
}
