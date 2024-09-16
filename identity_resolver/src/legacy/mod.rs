// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod commands;
mod resolver;
mod error;

use self::commands::SingleThreadedCommand;
use identity_document::document::CoreDocument;

pub use resolver::Resolver;
/// Alias for a [`Resolver`] that is not [`Send`] + [`Sync`].
pub type SingleThreadedResolver<DOC = CoreDocument> = Resolver<DOC, SingleThreadedCommand<DOC>>;
pub use error::Error;
pub use error::Result;
