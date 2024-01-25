use std::str::FromStr;

use identity_core::common::{Object, Url, Timestamp};
use identity_core::convert::{ToJson, FromJson};
use identity_did::DIDUrl;
use identity_core::common::Value;
use serde::Deserialize;
use serde::Serialize;
use crate::error::Result;
use crate::error::Error;
use super::Status;


//TODO: ZKP - RevocationTimeframeStatus and ValidityTimeframeEpoch


/// Validity Timeframe granularity
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidityTimeframeGranularity {
    /// Seconds
    SECOND,
    /// Minutes
    MINUTE,
    /// Hours
    HOUR
}

impl ToString for ValidityTimeframeGranularity {
  fn to_string(&self) -> String {
      match self {
          ValidityTimeframeGranularity::SECOND => String::from("SECOND"),
          ValidityTimeframeGranularity::MINUTE => String::from("MINUTE"),
          ValidityTimeframeGranularity::HOUR => String::from("HOUR"),
      }
  }
}

impl FromStr for ValidityTimeframeGranularity {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
          "SECOND" => Ok(ValidityTimeframeGranularity::SECOND),
          "MINUTE" => Ok(ValidityTimeframeGranularity::MINUTE),
          "HOUR" => Ok(ValidityTimeframeGranularity::HOUR),
          _ => Err("Invalid string representation for ValidityTimeframeEpoch"),
      }
  }
}

/// Information used to determine the current status of a [`Credential`][crate::credential::Credential]
/// using the `RevocationBitmap2022` specification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RevocationTimeframeStatus(Status);

impl RevocationTimeframeStatus {
  const TIMEFRAME_PROPERTY: &'static str = "validityTimeFrame";
  const GRANULARITY: &'static str = "granularity"; 
  /// Type name of the revocation bitmap.
  pub const TYPE: &'static str = "RevocationTimeframe2024";

  /// Creates a new `RevocationTimeframeStatus`.
  pub fn new(epoch: ValidityTimeframeGranularity) -> Self {
    let did_url: DIDUrl = DIDUrl::parse("did:method:0xffff#revocation-1").unwrap();
    let mut object = Object::new();

    

    let validity_timeframe = match epoch {
        ValidityTimeframeGranularity::SECOND => Timestamp::now_utc(),
        ValidityTimeframeGranularity::MINUTE => Timestamp::now_utc().to_minute(),
        ValidityTimeframeGranularity::HOUR => Timestamp::now_utc().to_hour(),
    };
    
    object.insert(Self::TIMEFRAME_PROPERTY.to_owned(), Value::String(validity_timeframe.to_rfc3339()));
    object.insert(Self::GRANULARITY.to_owned(), Value::String(epoch.to_string()));
    Self(Status::new_with_properties(
      Url::from(did_url), // Here maybe i could put the id of the service of the issuer document containing the revocationbitmap if we choose to use it and add also an index here
      // if we use a database though this field is useless
      Self::TYPE.to_owned(),
      object,
    ))

  }


  /// Get validityTimeframe value
  pub fn validity_timeframe(&self) -> Result<Timestamp> {
    if let Some(Value::String(timeframe)) = self.0.properties.get(Self::TIMEFRAME_PROPERTY) {

      Timestamp::from_str(&timeframe).map_err(|_| Error::InvalidStatus(format!(
        "property '{}' is not a string",
        Self::TIMEFRAME_PROPERTY
      )))
    
    } else {
      return Err(Error::InvalidStatus(format!(
        "property '{}' is not a string",
        Self::TIMEFRAME_PROPERTY
      )));
    }
  }

  /// Get granularity value
  pub fn granularity(&self) -> Result<ValidityTimeframeGranularity> {
    if let Some(Value::String(epoch)) = self.0.properties.get(Self::GRANULARITY) {

      ValidityTimeframeGranularity::from_str(&epoch).map_err(|_| Error::InvalidStatus(format!(
        "property '{}' is not a string",
        Self::GRANULARITY
      )))
    
    } else {
      return Err(Error::InvalidStatus(format!(
        "property '{}' is not a string",
        Self::GRANULARITY
      )));
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

    let revocation_timeframe: &Value =
      if let Some(revocation_timeframe) = status.properties.get(Self::TIMEFRAME_PROPERTY) {
        revocation_timeframe
      } else {
        return Err(Self::Error::InvalidStatus(format!(
          "missing required property '{}'",
          Self::TIMEFRAME_PROPERTY
        )));
      };

    let revocation_timeframe: Timestamp = if let Value::String(timeframe) = revocation_timeframe {

      Timestamp::from_str(&timeframe).map_err(|_| Self::Error::InvalidStatus(format!(
        "property '{}' is not a string",
        Self::TIMEFRAME_PROPERTY
      )))?
    
    } else {
      return Err(Self::Error::InvalidStatus(format!(
        "property '{}' is not a string",
        Self::TIMEFRAME_PROPERTY
      )));
    };


    let revocation_timeframe_epoch: &Value =
      if let Some(revocation_timeframe_epoch) = status.properties.get(Self::GRANULARITY) {
        revocation_timeframe_epoch
      } else {
        return Err(Self::Error::InvalidStatus(format!(
          "missing required property '{}'",
          Self::GRANULARITY
        )));
      };

    let revocation_timeframe_epoch: ValidityTimeframeGranularity = if let Value::String(epoch) = revocation_timeframe_epoch {

      ValidityTimeframeGranularity::from_str(&epoch).map_err(|_| Self::Error::InvalidStatus(format!(
        "property '{}' is not a string",
        Self::GRANULARITY
      )))?
    
    } else {
      return Err(Self::Error::InvalidStatus(format!(
        "property '{}' is not a string",
        Self::GRANULARITY
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