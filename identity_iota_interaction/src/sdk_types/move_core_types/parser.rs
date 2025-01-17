// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::iter::Peekable;

use anyhow::{bail, format_err, Result};

use super::super::move_types::identifier;
use super::super::move_types::account_address::AccountAddress;
use super::super::move_types::identifier::Identifier;

use super::language_storage::{TypeTag, StructTag};

#[derive(Eq, PartialEq, Debug)]
enum Token {
    U8Type,
    U16Type,
    U32Type,
    U64Type,
    U128Type,
    U256Type,
    BoolType,
    AddressType,
    VectorType,
    SignerType,
    Whitespace(String),
    Name(String),
    Address(String),
    U8(String),
    U16(String),
    U32(String),
    U64(String),
    U128(String),
    U256(String),

    Bytes(String),
    True,
    False,
    ColonColon,
    Lt,
    Gt,
    Comma,
    EOF,
}

impl Token {
    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace(_))
    }
}

fn token_as_name(tok: Token) -> Result<String> {
    use Token::*;
    Ok(match tok {
        U8Type => "u8".to_string(),
        U16Type => "u16".to_string(),
        U32Type => "u32".to_string(),
        U64Type => "u64".to_string(),
        U128Type => "u128".to_string(),
        U256Type => "u256".to_string(),
        BoolType => "bool".to_string(),
        AddressType => "address".to_string(),
        VectorType => "vector".to_string(),
        True => "true".to_string(),
        False => "false".to_string(),
        SignerType => "signer".to_string(),
        Name(s) => s,
        Whitespace(_) | Address(_) | U8(_) | U16(_) | U32(_) | U64(_) | U128(_) | U256(_)
        | Bytes(_) | ColonColon | Lt | Gt | Comma | EOF => {
            bail!("Invalid token. Expected a name but got {:?}", tok)
        }
    })
}

fn name_token(s: String) -> Token {
    match s.as_str() {
        "u8" => Token::U8Type,
        "u16" => Token::U16Type,
        "u32" => Token::U32Type,
        "u64" => Token::U64Type,
        "u128" => Token::U128Type,
        "u256" => Token::U256Type,
        "bool" => Token::BoolType,
        "address" => Token::AddressType,
        "vector" => Token::VectorType,
        "true" => Token::True,
        "false" => Token::False,
        "signer" => Token::SignerType,
        _ => Token::Name(s),
    }
}

fn next_number(initial: char, mut it: impl Iterator<Item = char>) -> Result<(Token, usize)> {
    let mut num = String::new();
    num.push(initial);
    loop {
        match it.next() {
            Some(c) if c.is_ascii_digit() || c == '_' => num.push(c),
            Some(c) if c.is_alphanumeric() => {
                let mut suffix = String::new();
                suffix.push(c);
                loop {
                    match it.next() {
                        Some(c) if c.is_ascii_alphanumeric() => suffix.push(c),
                        _ => {
                            let len = num.len() + suffix.len();
                            let tok = match suffix.as_str() {
                                "u8" => Token::U8(num),
                                "u16" => Token::U16(num),
                                "u32" => Token::U32(num),
                                "u64" => Token::U64(num),
                                "u128" => Token::U128(num),
                                "u256" => Token::U256(num),
                                _ => bail!("invalid suffix"),
                            };
                            return Ok((tok, len));
                        }
                    }
                }
            }
            _ => {
                let len = num.len();
                return Ok((Token::U64(num), len));
            }
        }
    }
}

#[allow(clippy::many_single_char_names)]
fn next_token(s: &str) -> Result<Option<(Token, usize)>> {
    let mut it = s.chars().peekable();
    match it.next() {
        None => Ok(None),
        Some(c) => Ok(Some(match c {
            '<' => (Token::Lt, 1),
            '>' => (Token::Gt, 1),
            ',' => (Token::Comma, 1),
            ':' => match it.next() {
                Some(':') => (Token::ColonColon, 2),
                _ => bail!("unrecognized token"),
            },
            '0' if it.peek() == Some(&'x') || it.peek() == Some(&'X') => {
                it.next().unwrap();
                match it.next() {
                    Some(c) if c.is_ascii_hexdigit() => {
                        let mut r = String::new();
                        r.push('0');
                        r.push('x');
                        r.push(c);
                        for c in it {
                            if c.is_ascii_hexdigit() {
                                r.push(c);
                            } else {
                                break;
                            }
                        }
                        let len = r.len();
                        (Token::Address(r), len)
                    }
                    _ => bail!("unrecognized token"),
                }
            }
            c if c.is_ascii_digit() => next_number(c, it)?,
            'b' if it.peek() == Some(&'"') => {
                it.next().unwrap();
                let mut r = String::new();
                loop {
                    match it.next() {
                        Some('"') => break,
                        Some(c) if c.is_ascii() => r.push(c),
                        _ => bail!("unrecognized token"),
                    }
                }
                let len = r.len() + 3;
                (Token::Bytes(hex::encode(r)), len)
            }
            'x' if it.peek() == Some(&'"') => {
                it.next().unwrap();
                let mut r = String::new();
                loop {
                    match it.next() {
                        Some('"') => break,
                        Some(c) if c.is_ascii_hexdigit() => r.push(c),
                        _ => bail!("unrecognized token"),
                    }
                }
                let len = r.len() + 3;
                (Token::Bytes(r), len)
            }
            c if c.is_ascii_whitespace() => {
                let mut r = String::new();
                r.push(c);
                for c in it {
                    if c.is_ascii_whitespace() {
                        r.push(c);
                    } else {
                        break;
                    }
                }
                let len = r.len();
                (Token::Whitespace(r), len)
            }
            c if c.is_ascii_alphabetic() => {
                let mut r = String::new();
                r.push(c);
                for c in it {
                    if identifier::is_valid_identifier_char(c) {
                        r.push(c);
                    } else {
                        break;
                    }
                }
                let len = r.len();
                (name_token(r), len)
            }
            _ => bail!("unrecognized token"),
        })),
    }
}

fn tokenize(mut s: &str) -> Result<Vec<Token>> {
    let mut v = vec![];
    while let Some((tok, n)) = next_token(s)? {
        v.push(tok);
        s = &s[n..];
    }
    Ok(v)
}

struct Parser<I: Iterator<Item = Token>> {
    it: Peekable<I>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    fn new<T: IntoIterator<Item = Token, IntoIter = I>>(v: T) -> Self {
        Self {
            it: v.into_iter().peekable(),
        }
    }

    fn next(&mut self) -> Result<Token> {
        match self.it.next() {
            Some(tok) => Ok(tok),
            None => bail!("out of tokens, this should not happen"),
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.it.peek()
    }

    fn consume(&mut self, tok: Token) -> Result<()> {
        let t = self.next()?;
        if t != tok {
            bail!("expected token {:?}, got {:?}", tok, t)
        }
        Ok(())
    }

    fn parse_comma_list<F, R>(
        &mut self,
        parse_list_item: F,
        end_token: Token,
        allow_trailing_comma: bool,
    ) -> Result<Vec<R>>
    where
        F: Fn(&mut Self) -> Result<R>,
        R: std::fmt::Debug,
    {
        let mut v = vec![];
        if !(self.peek() == Some(&end_token)) {
            loop {
                v.push(parse_list_item(self)?);
                if self.peek() == Some(&end_token) {
                    break;
                }
                self.consume(Token::Comma)?;
                if self.peek() == Some(&end_token) && allow_trailing_comma {
                    break;
                }
            }
        }
        Ok(v)
    }

    fn parse_type_tag(&mut self) -> Result<TypeTag> {
        Ok(match self.next()? {
            Token::U8Type => TypeTag::U8,
            Token::U16Type => TypeTag::U16,
            Token::U32Type => TypeTag::U32,
            Token::U64Type => TypeTag::U64,
            Token::U128Type => TypeTag::U128,
            Token::U256Type => TypeTag::U256,
            Token::BoolType => TypeTag::Bool,
            Token::AddressType => TypeTag::Address,
            Token::SignerType => TypeTag::Signer,
            Token::VectorType => {
                self.consume(Token::Lt)?;
                let ty = self.parse_type_tag()?;
                self.consume(Token::Gt)?;
                TypeTag::Vector(Box::new(ty))
            }
            Token::Address(addr) => {
                self.consume(Token::ColonColon)?;
                let module = self.next().and_then(token_as_name)?;
                self.consume(Token::ColonColon)?;
                let name = self.next().and_then(token_as_name)?;
                let ty_args = if self.peek() == Some(&Token::Lt) {
                    self.next()?;
                    let ty_args =
                        self.parse_comma_list(|parser| parser.parse_type_tag(), Token::Gt, true)?;
                    self.consume(Token::Gt)?;
                    ty_args
                } else {
                    vec![]
                };
                TypeTag::Struct(Box::new(StructTag {
                    address: AccountAddress::from_hex_literal(&addr)?,
                    module: Identifier::new(module)?,
                    name: Identifier::new(name)?,
                    type_params: ty_args,
                }))
            }
            tok => bail!("unexpected token {:?}, expected type tag", tok),
        })
    }
}

fn parse<F, T>(s: &str, f: F) -> Result<T>
    where
        F: Fn(&mut Parser<std::vec::IntoIter<Token>>) -> Result<T>,
{
    let mut tokens: Vec<_> = tokenize(s)?
        .into_iter()
        .filter(|tok| !tok.is_whitespace())
        .collect();
    tokens.push(Token::EOF);
    let mut parser = Parser::new(tokens);
    let res = f(&mut parser)?;
    parser.consume(Token::EOF)?;
    Ok(res)
}

pub fn parse_type_tag(s: &str) -> Result<TypeTag> {
    parse(s, |parser| parser.parse_type_tag())
}

pub fn parse_struct_tag(s: &str) -> Result<StructTag> {
    let type_tag = parse(s, |parser| parser.parse_type_tag())
        .map_err(|e| format_err!("invalid struct tag: {}, {}", s, e))?;
    if let TypeTag::Struct(struct_tag) = type_tag {
        Ok(*struct_tag)
    } else {
        bail!("invalid struct tag: {}", s)
    }
}