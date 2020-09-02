mod authentication;
mod context;
pub mod helpers;
mod keys;
mod service;
mod service_serialize;
mod subject;

pub use authentication::Authentication;
pub use context::Context;
pub use keys::{KeyData, PublicKey, PublicKeyTypes};
pub use service::{Service, ServiceEndpoint};
pub use subject::Subject;

pub trait Dedup<T: PartialEq + Clone> {
    fn clear_duplicates(&mut self);
}
