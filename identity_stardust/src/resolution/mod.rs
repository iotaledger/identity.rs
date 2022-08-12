// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
mod resolution_handler;
mod resolver;
mod resolver_delegate;
#[cfg(test)]
mod tests;
pub use resolution_handler::ResolutionHandler;
pub use resolver::Resolver;
