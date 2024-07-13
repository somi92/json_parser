use std::collections::HashMap;

use tokenizer::tokenize;

mod tokenizer;

/// Representation of possible JSON values.
pub enum Value {
    /// literal 'null' value
    Null,

    /// literal 'true' or 'false'
    Boolean(bool),

    /// value within doubel quotes "..."
    String(String),

    /// numbers stored as 64-bit floating point
    Number(f64),

    /// Zero or more JSON values
    Array(Vec<Value>),

    /// JSON value identified by a String key
    Object(HashMap<String, Value>),
}

pub fn parse(input: String) -> Result<(), ()> {
    let _ = tokenize(input).unwrap();
    Ok(())
}
