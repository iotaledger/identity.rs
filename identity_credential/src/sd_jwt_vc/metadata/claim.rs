use std::ops::Deref;

use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use serde_json::Value;

/// Information about a particular claim for displaying and validation purposes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimMetadata {
  /// [`ClaimPath`] of the claim or claims that are being addressed.
  pub path: ClaimPath,
  /// Object containing display information for the claim.
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub display: Vec<ClaimDisplay>,
  /// A string indicating whether the claim is selectively disclosable.
  pub sd: Option<ClaimDisclosability>,
  /// A string defining the ID of the claim for reference in the SVG template.
  pub svg_id: Option<String>,
}

/// A non-empty list of string, `null` values, or non-negative integers.
/// It is used to selected a particular claim in the credential or a
/// set of claims. See [Claim Path](https://www.ietf.org/archive/id/draft-ietf-oauth-sd-jwt-vc-05.html#name-claim-path) for more information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Vec<ClaimPathSegment>")]
pub struct ClaimPath(Vec<ClaimPathSegment>);

impl TryFrom<Vec<ClaimPathSegment>> for ClaimPath {
  type Error = anyhow::Error;
  fn try_from(value: Vec<ClaimPathSegment>) -> Result<Self, Self::Error> {
    if value.is_empty() {
      Err(anyhow::anyhow!("`ClaimPath` cannot be empty"))
    } else {
      Ok(Self(value))
    }
  }
}

impl Deref for ClaimPath {
  type Target = [ClaimPathSegment];
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

/// A single segment of a [`ClaimPath`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged, try_from = "Value")]
pub enum ClaimPathSegment {
  /// JSON object property.
  Name(String),
  /// JSON array entry.
  Position(usize),
  /// All properties or entries.
  #[serde(serialize_with = "serialize_all_variant")]
  All,
}

impl TryFrom<Value> for ClaimPathSegment {
  type Error = anyhow::Error;
  fn try_from(value: Value) -> Result<Self, Self::Error> {
    match value {
      Value::Null => Ok(ClaimPathSegment::All),
      Value::String(s) => Ok(ClaimPathSegment::Name(s)),
      Value::Number(n) => n
        .as_u64()
        .ok_or_else(|| anyhow::anyhow!("expected number greater or equal to 0"))
        .map(|n| ClaimPathSegment::Position(n as usize)),
      _ => Err(anyhow::anyhow!("expected either a string, number, or null")),
    }
  }
}

fn serialize_all_variant<S>(serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  serializer.serialize_none()
}

/// Information about whether a given claim is selectively disclosable.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClaimDisclosability {
  /// The issuer **must** make the claim selectively disclosable.
  Always,
  /// The issuer **may** make the claim selectively disclosable.
  #[default]
  Allowed,
  /// The issuer **must not** make the claim selectively disclosable.
  Never,
}

/// Display information for a given claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimDisplay {
  /// A language tag as defined in [RFC5646](https://www.rfc-editor.org/rfc/rfc5646.txt).
  pub lang: String,
  /// A human-readable label for the claim.
  pub label: String,
  /// A human-readable description for the claim.
  pub description: Option<String>,
}
