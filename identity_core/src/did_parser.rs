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
            Rule::path_segment => {
                path_segs.push(pair.as_str().to_string());
            }
            Rule::query => {
                let pairs = pair.clone().into_inner();
                for pair in pairs {
                    let mut param = Param { ..Default::default() };
                    match pair.as_rule() {
                        Rule::param => {
                            let pair = pair.clone().into_inner();
                            for p in pair {
                                match p.as_rule() {
                                    Rule::param_name => {
                                        param.key = p.as_str().to_string();
                                    }
                                    Rule::param_value => {
                                        param.value = Some(p.as_str().to_string());
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                    prms.push(param);
                }
            }
            Rule::fragment => did.add_fragment(pair.as_str().to_string()),
            _ => return Err(crate::Error::FormatError("Token in DID has an incorrect format".into())),
        }
    }

    if !prms.is_empty() {
        did.add_query(prms);
    }

    if !path_segs.is_empty() {
        did.add_path_segments(path_segs);
    }

    Ok(did)
}
