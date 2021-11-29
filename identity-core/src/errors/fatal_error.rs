// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0 
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
/// Error indicating that a fundamental assumption or invariant has been broken.  
pub struct FatalError {
  source: Option<Box<dyn std::error::Error + 'static + Send + Sync>>,
  description: String,
}

impl FatalError {
  /// Consumes the error returning its inner error (if any).
  pub fn into_source(self) -> Option<Box<dyn std::error::Error + Send + Sync + 'static>> {
    self.source
  }

  /// Constructs a new FatalError from any boxed `Error` trait object and description `String`
  pub fn new(error: Box<dyn std::error::Error + Send + Sync>, description: String) -> Self {
    Self {
      source: Some(error),
      description,
    }
  }
}

impl Display for FatalError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.description)
  }
}

impl Error for FatalError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    self.source.as_ref().map(|src| src.as_ref() as &(dyn Error + 'static))
  }
}

impl From<String> for FatalError {
  fn from(description: String) -> Self {
    Self {
      source: None,
      description,
    }
  }
}
