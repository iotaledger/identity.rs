// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
mod commands;
mod resolver;
mod resolver_builder;
use self::commands::SingleThreadedCommand;
use identity_credential::validator::ValidatorDocument;
pub use resolver::Resolver;
pub type SingleThreadedResolver<DOC = Box<dyn ValidatorDocument>> = Resolver<DOC, SingleThreadedCommand<DOC>>;
