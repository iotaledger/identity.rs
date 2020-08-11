use pest::{iterators::Pairs, Parser};
use pest_derive::*;

use crate::did::DID;

#[derive(Parser)]
#[grammar = "did.pest"]
pub struct DIDParser;
