use std::time::Instant;

use crate::{
    common::Object,
    did::DIDDocument,
    resolver::{ErrorKind, Resolution},
};

#[derive(Clone, Debug)]
pub struct ResolutionContext {
    instant: Instant,
    resolution: Resolution,
}

impl ResolutionContext {
    pub fn new() -> Self {
        Self {
            instant: Instant::now(),
            resolution: Resolution::new(),
        }
    }

    pub fn set_error(&mut self, value: ErrorKind) {
        self.resolution.metadata.error = Some(value);
    }

    pub fn set_document(&mut self, value: DIDDocument) {
        self.resolution.did_document = Some(value);
    }

    pub fn set_metadata(&mut self, value: Object) {
        self.resolution.did_document_metadata = Some(value);
    }

    pub fn finish(mut self) -> Resolution {
        self.resolution.metadata.duration = self.instant.elapsed();
        self.resolution
    }
}

impl Default for ResolutionContext {
    fn default() -> Self {
        Self::new()
    }
}
