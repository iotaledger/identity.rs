// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Creates a new IOTA DID from a `public` key and optional `network`/`shard`.
///
/// # Panics
///
/// Panics if the DID format is not valid.
///
/// # Example
///
/// ```
/// # use identity_iota::did;
/// #
/// let did = did!(b"public-key");
/// assert_eq!(did.as_str(), "did:iota:2xQiiGHDq5gCi1H7utY1ni7cf65fTay3G11S4xKp1vkS");
///
/// let did = did!(b"public-key", "com");
/// assert_eq!(
///   did.as_str(),
///   "did:iota:com:2xQiiGHDq5gCi1H7utY1ni7cf65fTay3G11S4xKp1vkS"
/// );
///
/// let did = did!(b"public-key", "com", "xyz");
/// assert_eq!(
///   did.as_str(),
///   "did:iota:com:xyz:2xQiiGHDq5gCi1H7utY1ni7cf65fTay3G11S4xKp1vkS"
/// );
/// ```
#[macro_export]
macro_rules! did {
  // Defining explicit branches rather than `$($tt:tt)+` gives much better docs.
  ($public:expr, $network:expr, $shard:expr) => {
    $crate::try_did!($public, $network, $shard).unwrap()
  };
  ($public:expr, $network:expr) => {
    $crate::try_did!($public, $network).unwrap()
  };
  ($public:expr) => {
    $crate::try_did!($public).unwrap()
  };
}

/// A fallible version of the [did] macro.
#[macro_export]
macro_rules! try_did {
  ($public:expr, $network:expr, $shard:expr) => {
    $crate::did::IotaDID::parse(format!(
      "{}:{}:{}:{}:{}",
      $crate::did::IotaDID::SCHEME,
      $crate::did::IotaDID::METHOD,
      $network,
      $shard,
      $crate::did::IotaDID::encode_key($public),
    ))
  };
  ($public:expr, $network:expr) => {
    $crate::did::IotaDID::parse(format!(
      "{}:{}:{}:{}",
      $crate::did::IotaDID::SCHEME,
      $crate::did::IotaDID::METHOD,
      $network,
      $crate::did::IotaDID::encode_key($public),
    ))
  };
  ($public:expr) => {
    $crate::did::IotaDID::parse(format!(
      "{}:{}:{}",
      $crate::did::IotaDID::SCHEME,
      $crate::did::IotaDID::METHOD,
      $crate::did::IotaDID::encode_key($public),
    ))
  };
}
