// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
mod commands;
mod resolver;
#[cfg(test)]
mod tests;

use self::commands::SingleThreadedCommand;
use identity_credential::validator::AbstractValidatorDocument;
pub use resolver::Resolver;
/// Alias for a [`Resolver`] that is not [`Send`] + [`Sync`].
pub type SingleThreadedResolver<DOC = AbstractValidatorDocument> = Resolver<DOC, SingleThreadedCommand<DOC>>;
