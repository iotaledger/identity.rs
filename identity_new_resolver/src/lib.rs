mod resolver;
mod error;

pub use error::{Result, Error};
pub use resolver::Resolver;

pub use compound_resolver::CompoundResolver;