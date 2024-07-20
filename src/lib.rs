use std::collections::HashMap;

use tokenizer::tokenize;

mod parser;
mod tokenizer;

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

pub fn parse(input: String) -> Result<(), ()> {
    let _ = tokenize(input).unwrap();
    Ok(())
}
