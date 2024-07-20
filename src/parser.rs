use std::collections::HashMap;

use crate::{tokenizer::Token, Value};

#[derive(Debug, PartialEq)]
pub enum TokenParseError {
    /// An escape sequence was started without 4 hexadecimal digits afterwards
    UnfinishedEscape,

    /// A character in an escape sequence was not valid hexadecimal
    InvalidHexValue,

    /// Invalid unicode value
    InvalidCodePointValue,

    /// Value was expected but not found
    ExpectedValue,

    /// Property name was expected but not found
    ExpectedProperty,

    /// Comma was expected but not found
    ExpectedComma,

    /// Colon was expected but not found
    ExpectedColon,

    /// Trailing comma found
    TrailingComma,
}

type ParseResult = Result<Value, TokenParseError>;

pub fn parse_tokens(tokens: &[Token], index: &mut usize) -> ParseResult {
    let token = &tokens[*index];

    if matches!(
        token,
        Token::Null | Token::False | Token::True | Token::Number(_) | Token::String(_)
    ) {
        *index += 1;
    }

    match token {
        Token::Null => Ok(Value::Null),
        Token::False => Ok(Value::Boolean(false)),
        Token::True => Ok(Value::Boolean(true)),
        Token::Number(number) => Ok(Value::Number(*number)),
        Token::String(string) => parse_string(string),
        Token::LeftBracket => parse_array(tokens, index),
        Token::LeftBrace => parse_object(tokens, index),
        _ => Err(TokenParseError::ExpectedValue),
    }
}

fn parse_string(input: &str) -> ParseResult {
    let output = unescape_string(input)?;
    Ok(Value::String(output))
}

fn unescape_string(input: &str) -> Result<String, TokenParseError> {
    let mut output = String::with_capacity(input.len());
    let mut in_escape_mode = false;
    let mut chars = input.chars();
    while let Some(next_char) = chars.next() {
        if in_escape_mode {
            match next_char {
                '\\' => output.push('\\'),
                '"' => output.push('"'),
                'n' => output.push('\n'),
                'r' => output.push('\r'),
                't' => output.push('\t'),
                'b' => output.push('\u{8}'),
                'f' => output.push('\u{12}'),
                'u' => {
                    let mut sum = 0;
                    for i in 0..4 {
                        let next_char = chars.next().ok_or(TokenParseError::UnfinishedEscape)?;
                        let digit = next_char
                            .to_digit(16)
                            .ok_or(TokenParseError::InvalidHexValue)?;
                        sum += (16u32).pow(3 - i) * digit;
                    }
                    let unescaped_char =
                        char::from_u32(sum).ok_or(TokenParseError::InvalidCodePointValue)?;
                    output.push(unescaped_char);
                }
                _ => output.push(next_char),
            }
            in_escape_mode = false;
        } else if next_char == '\\' {
            in_escape_mode = true;
        } else {
            output.push(next_char);
        }
    }
    Ok(output)
}

fn parse_array(tokens: &[Token], index: &mut usize) -> ParseResult {
    let mut output: Vec<Value> = Vec::new();

    loop {
        *index += 1;

        if tokens[*index] == Token::RightBracket {
            break;
        }

        let value = parse_tokens(tokens, index)?;
        output.push(value);

        let token = &tokens[*index];
        match token {
            Token::Comma => {}
            Token::RightBracket => break,
            _ => return Err(TokenParseError::ExpectedComma),
        }
    }

    *index += 1;

    Ok(Value::Array(output))
}

fn parse_object(tokens: &[Token], index: &mut usize) -> ParseResult {
    let mut output: HashMap<String, Value> = HashMap::new();

    loop {
        *index += 1;

        if tokens[*index] == Token::RightBrace {
            break;
        }

        if let Token::String(prop) = &tokens[*index] {
            *index += 1;

            if Token::Colon == tokens[*index] {
                *index += 1;

                let key = unescape_string(prop)?;
                let value = parse_tokens(tokens, index)?;

                output.insert(key, value);
            }

            match &tokens[*index] {
                Token::Comma => {}
                Token::RightBrace => break,
                _ => return Err(TokenParseError::ExpectedComma),
            }
        } else {
            return Err(TokenParseError::ExpectedProperty);
        }
    }

    Ok(Value::Object(output))
}

#[cfg(test)]
mod tests {
    use crate::{tokenizer::Token, Value};

    use super::{parse_tokens, TokenParseError};

    fn assert_parse_tokens(input: &[Token], expected: Value) {
        let actual = parse_tokens(input, &mut 0).unwrap();
        assert_eq!(actual, expected);
    }

    fn assert_error(input: &[Token], expected: TokenParseError) {
        let actual = parse_tokens(input, &mut 0).unwrap_err();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_null() {
        let input = [Token::Null];
        let expected = Value::Null;

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn parses_true() {
        let input = [Token::True];
        let expected = Value::Boolean(true);

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn parses_false() {
        let input = [Token::False];
        let expected = Value::Boolean(false);

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn parses_number() {
        let input = [Token::Number(23.31)];
        let expected = Value::Number(23.31);

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn parses_string_no_escapes() {
        let input = [Token::String("hello world".into())];
        let expected = Value::String("hello world".into());

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn parses_string_non_ascii() {
        let input = [Token::string("olá_こんにちは_नमस्ते_привіт")];
        let expected = Value::String(String::from("olá_こんにちは_नमस्ते_привіт"));

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn parses_string_with_emoji() {
        let input = [Token::string("hello 💩 world")];
        let expected = Value::String(String::from("hello 💩 world"));

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn parses_string_unescape_backslash() {
        let input = [Token::String(r#"hello\\world"#.into())];
        let expected = Value::String(r#"hello\world"#.into());

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn parses_string_unescape_newline() {
        let input = [Token::string(r#"hello\nworld"#)];
        let expected = Value::String(String::from("hello\nworld"));

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn all_the_simple_escapes() {
        let input = [Token::string(r#"\"\/\\\b\f\n\r\t"#)];
        let expected = Value::String(String::from("\"/\\\u{8}\u{12}\n\r\t"));

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn parses_string_with_unescaped_emoji() {
        let input = [Token::string("hello 💩 world")];
        let expected = Value::String(String::from("hello 💩 world"));

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn parses_string_with_unnecessarily_escaped_emoji() {
        let input = [Token::string(r#"hello \💩 world"#)];
        let expected = Value::String(String::from("hello 💩 world"));

        assert_parse_tokens(&input, expected);
    }

    #[test]
    #[ignore = "decoding of UTF-16 surrogate pairs is not implemented"]
    fn parses_string_with_escaped_surrogate_pairs_for_an_emoji() {
        let input = [Token::string(r#"hello\uD83C\uDF3Cworld"#)];
        let expected = Value::String(String::from("hello🌼world"));

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn parses_empty_arrays() {
        // []
        let input = [Token::LeftBracket, Token::RightBracket];
        let expected = Value::Array(vec![]);

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn parses_array_one_element() {
        // [true]
        let input = [Token::LeftBracket, Token::True, Token::RightBracket];
        let expected = Value::Array(vec![Value::Boolean(true)]);

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn parses_array_two_elements() {
        // [null, 16]
        let input = [
            Token::LeftBracket,
            Token::Null,
            Token::Comma,
            Token::Number(16.0),
            Token::RightBracket,
        ];
        let expected = Value::Array(vec![Value::Null, Value::Number(16.0)]);

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn parses_nested_array() {
        // [null, [null]]
        let input = [
            Token::LeftBracket,
            Token::Null,
            Token::Comma,
            Token::LeftBracket,
            Token::Null,
            Token::RightBracket,
            Token::RightBracket,
        ];
        let expected = Value::Array(vec![Value::Null, Value::Array(vec![Value::Null])]);

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn fails_array_leading_comma() {
        // [,true]
        let input = [
            Token::LeftBracket,
            Token::Comma,
            Token::True,
            Token::RightBracket,
        ];
        let expected = TokenParseError::ExpectedValue;

        assert_error(&input, expected);
    }

    #[test]
    #[ignore = "the current implementation allows trailing commas"]
    fn fails_array_trailing_comma() {
        // [true,]
        let input = [
            Token::LeftBracket,
            Token::True,
            Token::Comma,
            Token::RightBracket,
        ];
        let expected = TokenParseError::TrailingComma;

        assert_error(&input, expected);
    }

    #[test]
    fn parses_empty_object() {
        let input = [Token::LeftBrace, Token::RightBrace];
        let expected = Value::object([]);

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn parses_object_one_string_value() {
        let input = [
            Token::LeftBrace,
            Token::string("name"),
            Token::Colon,
            Token::string("davimiku"),
            Token::RightBrace,
        ];
        let expected = Value::object([("name", Value::string("davimiku"))]);

        assert_parse_tokens(&input, expected);
    }

    #[test]
    fn parses_object_escaped_key() {
        let input = [
            Token::LeftBrace,
            Token::string(r#"\u540D\u524D"#),
            Token::Colon,
            Token::string("davimiku"),
            Token::RightBrace,
        ];
        let expected = Value::object([("名前", Value::string("davimiku"))]);

        assert_parse_tokens(&input, expected);
    }
}
