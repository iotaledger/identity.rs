// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Borrow;

use identity_credential::{credential::Credential, presentation::Presentation};
use crate::Result;
use crate::Error; 


// ----------------------------------------------------------- PresentationValidator-------------------------------------------------------------------------
#[non_exhaustive]
pub struct PresentationValidator<T: Borrow<Presentation>> {
    pub presentation: T, 
}

impl <T: Borrow<Presentation>> PresentationValidator<T> {
    pub fn new(presentation: T) -> Self {
        Self {
            presentation
        }
    }
}

