#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;
