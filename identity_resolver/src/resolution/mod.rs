// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
mod commands;
mod resolver;
use identity_credential::validator::ValidatorDocument;
pub use resolver::Resolver;
use self::commands::SingleThreadedCommand;
pub type SingleThreadedResolver<DOC=Box<dyn ValidatorDocument>> = Resolver<DOC,SingleThreadedCommand<DOC>>; 
