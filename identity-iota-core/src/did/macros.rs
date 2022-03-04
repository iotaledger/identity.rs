// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Creates a new IOTA DID from a `public` key and optional `network`.
///
/// # Errors
///
/// Errors if the [`IotaDID`][crate::did::IotaDID] is invalid.
///
/// # Example
///
/// ```
/// # use identity_did::did::DID;
/// # use identity_iota_core::try_construct_did;
/// #
/// let did = try_construct_did!(b"public-key")?;
/// assert_eq!(did.as_str(), "did:iota:2xQiiGHDq5gCi1H7utY1ni7cf65fTay3G11S4xKp1vkS");
///
/// let did = try_construct_did!(b"public-key", "com")?;
/// assert_eq!(
///   did.as_str(),
///   "did:iota:com:2xQiiGHDq5gCi1H7utY1ni7cf65fTay3G11S4xKp1vkS"
/// );
/// # Ok::<(), identity_iota_core::Error>(())
/// ```
#[macro_export]
macro_rules! try_construct_did {
  // Defining explicit branches rather than `$($tt:tt)+` gives much better docs.
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
