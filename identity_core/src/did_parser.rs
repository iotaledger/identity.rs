use pest::{iterators::Pairs, Parser};
use pest_derive::*;

use crate::did::{Param, DID};

/// a derived parser for the `DID` struct.  Rules are derived from the `did.pest` file using the `pest` crate.
#[derive(Parser)]
#[grammar = "did.pest"]
struct DIDParser;

/// parses a `ToString` type into a `DID` if it follows the proper format.  
pub fn parse<T>(input: T) -> crate::Result<DID>
where
    T: ToString,
{
    let input_str = input.to_string();
    let pairs = DIDParser::parse(Rule::did, &*input_str);

    match pairs {
        Ok(p) => Ok(parse_pairs(p)?),
        Err(e) => Err(crate::Error::ParseError(e)),
    }
}

/// The inner parsing method for the `DIDParser`.
fn parse_pairs(pairs: Pairs<Rule>) -> crate::Result<DID> {
    let mut prms: Vec<Param> = Vec::new();
    let mut path_segs: Vec<String> = Vec::new();

    let mut did = DID::default();

    for pair in pairs {
        // match the rules from the did.pest file.
        match pair.as_rule() {
            Rule::method_name => {
                did.method_name = pair.as_str().to_string();
            }
            Rule::id_segment => {
                if pair.as_str() == String::new() {
                    return Err(crate::Error::FormatError(
                        "Format of id_segment is wrong or empty".into(),
                    ));
                }
                did.id_segments.push(pair.as_str().to_string());
            }
            Rule::param => {
                let mut inner = pair.into_inner();
                let name = inner.next().expect("No name for this value");

                match inner.next() {
                    Some(val) => {
                        prms.push(Param::from((name.as_str().to_string(), Some(val.as_str().to_string()))));
                    }
                    None => {
                        prms.push(Param::from((name.as_str().to_string(), None)));
                    }
                }
            }
            Rule::path_segment => {
                path_segs.push(pair.as_str().to_string());
            }
            Rule::query => did.add_query(pair.as_str().to_string()),
            Rule::fragment => did.add_fragment(pair.as_str().to_string()),
            _ => return Err(crate::Error::FormatError("Token in DID has an incorrect format".into())),
        }
    }

    if !prms.is_empty() {
        did.add_params(prms);
    }

    if !path_segs.is_empty() {
        did.add_path_segments(path_segs);
    }

    Ok(did)
}
