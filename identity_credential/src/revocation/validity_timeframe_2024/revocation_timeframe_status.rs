use crate::credential::try_index_to_u32;
use crate::credential::Status;
use crate::error::Error;
use crate::error::Result;
use identity_core::common::Duration;
use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::common::Value;
use identity_did::DIDUrl;
use std::str::FromStr;

/// Information used to determine the current status of a [`Credential`][crate::credential::Credential]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RevocationTimeframeStatus(Status);

impl RevocationTimeframeStatus {
  /// startValidityTimeframe property name.
  pub const START_TIMEFRAME_PROPERTY: &'static str = "startValidityTimeframe";
  /// endValidityTimeframe property name.
  pub const END_TIMEFRAME_PROPERTY: &'static str = "endValidityTimeframe";
  const INDEX_PROPERTY: &'static str = "revocationBitmapIndex";

  /// Type name of the revocation mechanism.
  pub const TYPE: &'static str = "RevocationTimeframe2024";

  /// Creates a new `RevocationTimeframeStatus`.
  pub fn new(start_validity: Option<Timestamp>, duration: Duration, id: DIDUrl, index: u32) -> Result<Self> {
    let mut object = Object::new();

    let start_validity_timeframe = start_validity.unwrap_or(Timestamp::now_utc());

    let end_validity_timeframe = start_validity_timeframe
      .checked_add(duration)
      .ok_or(Error::InvalidStatus(
        "With that granularity, endValidityTimeFrame will turn out not to be in the valid range for RFC 3339"
          .to_owned(),
      ))?;

    // id.set_query(Some(&format!("index={index}")))
    // .expect("the string should be non-empty and a valid URL query");

    object.insert(
      Self::START_TIMEFRAME_PROPERTY.to_owned(),
      Value::String(start_validity_timeframe.to_rfc3339()),
    );
    object.insert(
      Self::END_TIMEFRAME_PROPERTY.to_owned(),
      Value::String(end_validity_timeframe.to_rfc3339()),
    );
    object.insert(Self::INDEX_PROPERTY.to_owned(), Value::String(index.to_string()));

    Ok(Self(Status::new_with_properties(
      Url::from(id),
      Self::TYPE.to_owned(),
      object,
    )))
  }

  /// Get startValidityTimeframe value
  pub fn start_validity_timeframe(&self) -> Result<Timestamp> {
    if let Some(Value::String(timeframe)) = self.0.properties.get(Self::START_TIMEFRAME_PROPERTY) {
      Timestamp::from_str(timeframe)
        .map_err(|_| Error::InvalidStatus(format!("property '{}' is not a string", Self::START_TIMEFRAME_PROPERTY)))
    } else {
      Err(Error::InvalidStatus(format!(
        "property '{}' is not a string",
        Self::START_TIMEFRAME_PROPERTY
      )))
    }
  }

  /// Get endValidityTimeframe value
  pub fn end_validity_timeframe(&self) -> Result<Timestamp> {
    if let Some(Value::String(timeframe)) = self.0.properties.get(Self::END_TIMEFRAME_PROPERTY) {
      Timestamp::from_str(timeframe)
        .map_err(|_| Error::InvalidStatus(format!("property '{}' is not a string", Self::END_TIMEFRAME_PROPERTY)))
    } else {
      Err(Error::InvalidStatus(format!(
        "property '{}' is not a string",
        Self::END_TIMEFRAME_PROPERTY
      )))
    }
  }

  /// Returns the [`DIDUrl`] of the `RevocationBitmapStatus`, which should resolve
  /// to a `RevocationBitmap2022` service in a DID Document.
  pub fn id(&self) -> Result<DIDUrl> {
    DIDUrl::parse(self.0.id.as_str())
      .map_err(|err| Error::InvalidStatus(format!("invalid DID Url '{}': {:?}", self.0.id, err)))
  }

  /// Returns the index of the credential in the issuer's revocation bitmap if it can be decoded.
  pub fn index(&self) -> Result<u32> {
    if let Some(Value::String(index)) = self.0.properties.get(Self::INDEX_PROPERTY) {
      try_index_to_u32(index, Self::INDEX_PROPERTY)
    } else {
      Err(Error::InvalidStatus(format!(
        "expected {} to be an unsigned 32-bit integer expressed as a string",
        Self::INDEX_PROPERTY
      )))
    }
  }
}

impl TryFrom<Status> for RevocationTimeframeStatus {
  type Error = Error;

  fn try_from(status: Status) -> Result<Self> {
    if status.type_ != Self::TYPE {
      return Err(Self::Error::InvalidStatus(format!(
        "expected type '{}', got '{}'",
        Self::TYPE,
        status.type_
      )));
    }

    let start_revocation_timeframe: &Value =
      if let Some(start_revocation_timeframe) = status.properties.get(Self::START_TIMEFRAME_PROPERTY) {
        start_revocation_timeframe
      } else {
        return Err(Self::Error::InvalidStatus(format!(
          "missing required property '{}'",
          Self::START_TIMEFRAME_PROPERTY
        )));
      };

    if let Value::String(timeframe) = start_revocation_timeframe {
      Timestamp::from_str(timeframe).map_err(|_| {
        Self::Error::InvalidStatus(format!(
          "property '{}' is not a valid Timestamp",
          Self::START_TIMEFRAME_PROPERTY
        ))
      })?
    } else {
      return Err(Self::Error::InvalidStatus(format!(
        "property '{}' is not a string",
        Self::START_TIMEFRAME_PROPERTY
      )));
    };

    let end_revocation_timeframe: &Value =
      if let Some(end_revocation_timeframe) = status.properties.get(Self::END_TIMEFRAME_PROPERTY) {
        end_revocation_timeframe
      } else {
        return Err(Self::Error::InvalidStatus(format!(
          "missing required property '{}'",
          Self::END_TIMEFRAME_PROPERTY
        )));
      };

    if let Value::String(timeframe) = end_revocation_timeframe {
      Timestamp::from_str(timeframe).map_err(|_| {
        Self::Error::InvalidStatus(format!(
          "property '{}' is not a valid Timestamp",
          Self::END_TIMEFRAME_PROPERTY
        ))
      })?
    } else {
      return Err(Self::Error::InvalidStatus(format!(
        "property '{}' is not a string",
        Self::END_TIMEFRAME_PROPERTY
      )));
    };

    let revocation_bitmap_index: &Value =
      if let Some(revocation_bitmap_index) = status.properties.get(Self::INDEX_PROPERTY) {
        revocation_bitmap_index
      } else {
        return Err(Error::InvalidStatus(format!(
          "missing required property '{}'",
          Self::INDEX_PROPERTY
        )));
      };

    if let Value::String(index) = revocation_bitmap_index {
      try_index_to_u32(index, Self::INDEX_PROPERTY)?
    } else {
      return Err(Error::InvalidStatus(format!(
        "property '{}' is not a string",
        Self::INDEX_PROPERTY
      )));
    };

    Ok(Self(status))
  }
}

impl From<RevocationTimeframeStatus> for Status {
  fn from(status: RevocationTimeframeStatus) -> Self {
    status.0
  }
}

/// Verifier
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifierRevocationTimeframeStatus(pub(crate) RevocationTimeframeStatus);

impl TryFrom<Status> for VerifierRevocationTimeframeStatus {
  type Error = Error;

  fn try_from(status: Status) -> Result<Self> {
    if status.type_ != RevocationTimeframeStatus::TYPE {
      return Err(Self::Error::InvalidStatus(format!(
        "expected type '{}', got '{}'",
        RevocationTimeframeStatus::TYPE,
        status.type_
      )));
    }

    let start_revocation_timeframe: &Value = if let Some(start_revocation_timeframe) = status
      .properties
      .get(RevocationTimeframeStatus::START_TIMEFRAME_PROPERTY)
    {
      start_revocation_timeframe
    } else {
      return Err(Self::Error::InvalidStatus(format!(
        "missing required property '{}'",
        RevocationTimeframeStatus::START_TIMEFRAME_PROPERTY
      )));
    };

    if let Value::String(timeframe) = start_revocation_timeframe {
      Timestamp::from_str(timeframe).map_err(|_| {
        Self::Error::InvalidStatus(format!(
          "property '{}' is not a valid Timestamp",
          RevocationTimeframeStatus::START_TIMEFRAME_PROPERTY
        ))
      })?
    } else {
      return Err(Self::Error::InvalidStatus(format!(
        "property '{}' is not a string",
        RevocationTimeframeStatus::START_TIMEFRAME_PROPERTY
      )));
    };

    let end_revocation_timeframe: &Value = if let Some(end_revocation_timeframe) =
      status.properties.get(RevocationTimeframeStatus::END_TIMEFRAME_PROPERTY)
    {
      end_revocation_timeframe
    } else {
      return Err(Self::Error::InvalidStatus(format!(
        "missing required property '{}'",
        RevocationTimeframeStatus::END_TIMEFRAME_PROPERTY
      )));
    };

    if let Value::String(timeframe) = end_revocation_timeframe {
      Timestamp::from_str(timeframe).map_err(|_| {
        Self::Error::InvalidStatus(format!(
          "property '{}' is not a valid Timestamp",
          RevocationTimeframeStatus::END_TIMEFRAME_PROPERTY
        ))
      })?
    } else {
      return Err(Self::Error::InvalidStatus(format!(
        "property '{}' is not a string",
        RevocationTimeframeStatus::END_TIMEFRAME_PROPERTY
      )));
    };

    let revocation_bitmap_index: &Value =
      if let Some(revocation_bitmap_index) = status.properties.get(RevocationTimeframeStatus::INDEX_PROPERTY) {
        revocation_bitmap_index
      } else {
        return Err(Error::InvalidStatus(format!(
          "missing required property '{}'",
          RevocationTimeframeStatus::INDEX_PROPERTY
        )));
      };

    if &Value::Null != revocation_bitmap_index {
      return Err(Error::InvalidStatus(format!(
        "property '{}' is not a Null",
        RevocationTimeframeStatus::INDEX_PROPERTY
      )));
    };

    Ok(Self(RevocationTimeframeStatus(status)))
  }
}

impl From<VerifierRevocationTimeframeStatus> for Status {
  fn from(status: VerifierRevocationTimeframeStatus) -> Self {
    status.0 .0
  }
}
