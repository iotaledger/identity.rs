use std::time::Instant;

use crate::{
    deref::{Dereference, Resource},
    resolver::{ErrorKind, Resolution},
};

#[derive(Clone, Debug)]
pub struct DereferenceContext {
    instant: Instant,
    dereference: Dereference,
}

impl DereferenceContext {
    pub fn new() -> Self {
        Self {
            instant: Instant::now(),
            dereference: Dereference::new(),
        }
    }

    pub fn set_error(&mut self, value: ErrorKind) {
        self.dereference.metadata.error = Some(value);
    }

    pub fn set_resource(&mut self, value: impl Into<Resource>) {
        self.dereference.content = Some(value.into());
    }

    pub fn set_resolution(&mut self, value: Resolution) {
        self.dereference.resolution = value;
    }

    pub fn finish(mut self) -> Dereference {
        self.dereference.metadata.duration = self.instant.elapsed();
        self.dereference
    }
}

impl Default for DereferenceContext {
    fn default() -> Self {
        Self::new()
    }
}
