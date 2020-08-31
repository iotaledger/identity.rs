use crate::signature::SignatureSuite;

pub struct Proof(Box<dyn SignatureSuite>);
