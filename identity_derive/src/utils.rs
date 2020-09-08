use proc_macro2::{Delimiter, TokenTree};
use syn::*;

const PARENS: Delimiter = Delimiter::Parenthesis;

pub fn should_ignore(field: &Field) -> bool {
    let mut ignore = false;

    field.attrs.iter().for_each(|field| {
        let attr_seg: Vec<String> = field.path.segments.iter().map(|seg| format!("{}", seg.ident)).collect();

        let diff_attr = attr_seg == ["diff"];
        let arg_iter = field.tokens.clone().into_iter().next();

        let should_ignore = match arg_iter {
            Some(TokenTree::Group(gr)) if gr.delimiter() == PARENS => {
                let tokens: Vec<String> = gr.stream().into_iter().map(|tt| format!("{}", tt)).collect();

                tokens == ["should_ignore"]
            }
            _ => false,
        };
        ignore = ignore || diff_attr && should_ignore
    });

    ignore
}
