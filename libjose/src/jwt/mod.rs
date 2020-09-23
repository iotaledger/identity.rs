//! JSON Web Tokens ([JWT](https://tools.ietf.org/html/rfc7519))

mod claims;

pub use self::claims::*;

cfg_if::cfg_if! {
  if #[cfg(feature = "std")] {
    mod profile;

    pub use self::profile::*;
  }
}
