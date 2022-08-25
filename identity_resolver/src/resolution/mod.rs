use identity_credential::validator::ValidatorDocument;

// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
mod resolver;
//pub use resolver::Resolver;
pub type Resolver<DOC = Box<dyn ValidatorDocument + Send + Sync + 'static>> = resolver::Resolver<resolver::SendSyncCommand<DOC>,DOC>;
