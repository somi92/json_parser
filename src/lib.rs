use std::collections::HashMap;

use parser::{parse_tokens, TokenParseError};
use tokenizer::{tokenize, TokenizeError};

mod parser;
mod tokenizer;

pub fn parse(input: String) -> Result<Value, ParseError> {
    let tokens = tokenize(input)?;
    let value = parse_tokens(&tokens, &mut 0)?;
    Ok(value)
}

/// Representation of possible JSON values.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Literal 'null' value
    Null,

    /// Literal 'true' or 'false'
    Boolean(bool),

    /// Value within doubel quotes "..."
    String(String),

    /// Numbers stored as 64-bit floating point
    Number(f64),

    /// Zero or more JSON values
    Array(Vec<Value>),

    /// JSON value identified by a String key
    Object(HashMap<String, Value>),
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    TokenizeError(TokenizeError),
    ParseError(TokenParseError),
}

#[cfg(test)]
impl Value {
    pub(crate) fn object<const N: usize>(pairs: [(&'static str, Self); N]) -> Self {
        let owned_pairs = pairs.map(|(key, value)| (String::from(key), value));
        let map = HashMap::from(owned_pairs);
        Self::Object(map)
    }

    pub(crate) fn string(s: &str) -> Self {
        Self::String(String::from(s))
    }
}

impl From<TokenParseError> for ParseError {
    fn from(err: TokenParseError) -> Self {
        Self::ParseError(err)
    }
}

impl From<TokenizeError> for ParseError {
    fn from(err: TokenizeError) -> Self {
        Self::TokenizeError(err)
    }
}
