use core::str;

use crate::error::Error;
use crate::error::Result;

pub fn parse_utf8(slice: &(impl AsRef<[u8]> + ?Sized)) -> Result<&str> {
  str::from_utf8(slice.as_ref()).map_err(Error::InvalidUtf8)
}
