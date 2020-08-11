use pest::{iterators::Pairs, Parser};
use pest_derive::*;

use crate::did::DID;

#[derive(Parser)]
#[grammar = "did.pest"]
pub struct DIDParser;

pub fn parse<T>(input: T) -> crate::Result<DID>
where
    T: ToString,
{
    let input_str = input.to_string();
    let pairs = DIDParser::parse(Rule::did, &*input_str);

    match pairs {
        Ok(p) => return Ok(parse_pairs(p)?),
        Err(e) => return Err(crate::Error::ParseError(e)),
    }
}

fn parse_pairs(pairs: Pairs<Rule>) -> crate::Result<DID> {
    let mut name: String = String::new();
    let mut id_segs: Vec<String> = Vec::new();
    let mut params: Option<Vec<(String, Option<String>)>> = None;
    let mut prms: Vec<(String, Option<String>)> = Vec::new();
    let mut path_segments: Option<Vec<String>> = None;
    let mut path_segs: Vec<String> = Vec::new();
    let mut query: Option<String> = None;
    let mut frag: Option<String> = None;

    for pair in pairs {
        match pair.as_rule() {
            Rule::method_name => {
                name = pair.as_str().to_string();
            }
            Rule::id_segment => {
                id_segs.push(pair.as_str().to_string());
            }
            Rule::param => {
                let mut inner = pair.into_inner();
                let name = inner.next().expect("No name for this value");

                match inner.next() {
                    Some(val) => {
                        prms.push((name.as_str().to_string(), Some(val.as_str().to_string())));
                    }
                    None => {
                        prms.push((name.as_str().to_string(), None));
                    }
                }
            }
            Rule::path_segment => {
                path_segs.push(pair.as_str().to_string());
            }
            Rule::query => query = Some(pair.as_str().to_string()),
            Rule::fragment => frag = Some(pair.as_str().to_string()),
            _ => {}
        }
    }

    if !prms.is_empty() {
        params = Some(prms);
    }

    if !path_segs.is_empty() {
        path_segments = Some(path_segs);
    }

    Ok(DID::new(name, id_segs, params, path_segments, query, frag))
}
