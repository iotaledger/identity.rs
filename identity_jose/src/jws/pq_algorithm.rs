//TODO: JWS PQ algorithms

use std::str::FromStr;
use crate::error::Error;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

/// Supported post-quantum algorithms for the JSON Web Signatures `alg` claim.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
#[allow(non_camel_case_types)]
pub enum JwsAlgorithmPQ {
    /// JSON Web Signature Algorithm for ML-DSA-44
    /// [More Info](https://datatracker.ietf.org/doc/html/draft-ietf-cose-dilithium#name-the-ml-dsa-algorithm-family)
    #[serde(rename = "ML-DSA-44")]
    ML_DSA_44,
    /// JSON Web Signature Algorithm for ML-DSA-44
    /// [More Info](https://datatracker.ietf.org/doc/html/draft-ietf-cose-dilithium#name-the-ml-dsa-algorithm-family)
    #[serde(rename = "ML-DSA-65")]
    ML_DSA_65,
    /// JSON Web Signature Algorithm for ML-DSA-44
    /// [More Info](https://datatracker.ietf.org/doc/html/draft-ietf-cose-dilithium#name-the-ml-dsa-algorithm-family)
    #[serde(rename = "ML-DSA-87")]
    ML_DSA_87
}




impl JwsAlgorithmPQ {
    /// A slice of all supported [`JwsAlgorithmPQ`]s.
    pub const ALL: &'static [Self] = &[
      Self::ML_DSA_44,
      Self::ML_DSA_65,
      Self::ML_DSA_87,
    ];
  
    /// Returns the JWS algorithm as a `str` slice.
    pub const fn name(self) -> &'static str {
      match self {
        Self::ML_DSA_44 => "ML-DSA-44",
        Self::ML_DSA_65 => "ML-DSA-65",
        Self::ML_DSA_87 => "ML-DSA-87",
      }
    }
  }
  
  impl FromStr for JwsAlgorithmPQ {
    type Err = crate::error::Error;
  
    fn from_str(string: &str) -> std::result::Result<Self, Self::Err> {
      match string {
        "ML-DSA-44" => Ok(Self::ML_DSA_44),
        "ML-DSA-65" => Ok(Self::ML_DSA_65),
        "ML-DSA-87" => Ok(Self::ML_DSA_87),
        _ => Err(Error::JwsAlgorithmParsingError),
      }
    }
  }
  
  impl Display for JwsAlgorithmPQ {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      f.write_str(self.name())
    }
  }