use std::num::ParseFloatError;

/// Represents possible lexical tokens.
#[derive(Debug, PartialEq)]
pub enum Token {
    /// '{'
    LeftBrace,

    /// '}'
    RightBrace,

    /// '['
    LeftBracket,

    /// ']'
    RightBracket,

    /// ','
    Comma,

    /// ':'
    Colon,

    /// 'null'
    Null,

    /// 'false'
    False,

    /// 'true'
    True,

    /// Any number literal
    Number(f64),

    /// Key of the value or string value
    String(String),
}

/// Possible errors that can occur when tokenizing the input
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenizeError {
    /// The input started with as a literal value but did not match it
    UnfinishedLiteralValue,

    /// Unable to parse the float number
    ParseNumberError(ParseFloatError),

    /// Matching closing quotes are not found
    UnclosedQuotes,

    /// Character is not recognized as a part of a valid JSON token
    CharNotRecognized(char),

    /// Input ended prematurely
    UnexpectedEof,
}

/// Creates a vector of tokens from a given String input.
pub fn tokenize(input: String) -> Result<Vec<Token>, TokenizeError> {
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;

    let mut tokens = Vec::new();
    while index < chars.len() {
        let token = create_token(&chars, &mut index)?;
        tokens.push(token);
        index += 1;
    }

    Ok(tokens)
}

fn create_token(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    let mut ch = chars[*index];

    while ch.is_ascii_whitespace() {
        *index += 1;

        if *index >= chars.len() {
            return Err(TokenizeError::UnexpectedEof);
        }

        ch = chars[*index];
    }

    let token = match ch {
        '{' => Token::LeftBrace,
        '}' => Token::RightBrace,
        '[' => Token::LeftBracket,
        ']' => Token::RightBracket,
        ',' => Token::Comma,
        ':' => Token::Colon,
        'n' => tokenize_literal(chars, index, "null", Token::Null)?,
        't' => tokenize_literal(chars, index, "true", Token::True)?,
        'f' => tokenize_literal(chars, index, "false", Token::False)?,
        '"' => tokenize_string(chars, index)?,
        c if c.is_ascii_digit() || c == '-' => tokenize_float(chars, index)?,

        ch => return Err(TokenizeError::CharNotRecognized(ch)),
    };

    Ok(token)
}

fn tokenize_literal(
    chars: &[char],
    index: &mut usize,
    literal_value: &str,
    token_value: Token,
) -> Result<Token, TokenizeError> {
    for expected_char in literal_value.chars() {
        if expected_char != chars[*index] {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1;
    }

    *index -= 1;
    Ok(token_value)
}

fn tokenize_float(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    let mut unparsed_num = String::new();
    let mut is_decimal = false;

    while *index < chars.len() {
        let ch = chars[*index];
        match ch {
            c if c.is_ascii_digit() || c == '-' => unparsed_num.push(c),
            c if c == '.' && !is_decimal => {
                unparsed_num.push('.');
                is_decimal = true;
            }
            _ => break,
        }
        *index += 1;
    }

    *index -= 1;

    match unparsed_num.parse() {
        Ok(f) => Ok(Token::Number(f)),
        Err(err) => Err(TokenizeError::ParseNumberError(err)),
    }
}

fn tokenize_string(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    let mut string = String::new();
    let mut in_escape_mode = false;

    loop {
        *index += 1;
        if *index >= chars.len() {
            return Err(TokenizeError::UnclosedQuotes);
        }

        let ch = chars[*index];
        match ch {
            '"' if !in_escape_mode => break,
            '\\' => in_escape_mode = !in_escape_mode,
            _ => in_escape_mode = false,
        }

        string.push(ch);
    }

    Ok(Token::String(string))
}

#[cfg(test)]
impl Token {
    pub(crate) fn string(input: &str) -> Self {
        Self::String(String::from(input))
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::TokenizeError;

    use super::{tokenize, Token};

    #[test]
    fn just_comma() {
        let input = String::from(",");
        let expected = [Token::Comma];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_null() {
        let input = String::from("null");
        let expected = [Token::Null];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_true() {
        let input = String::from("true");
        let expected = [Token::True];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_false() {
        let input = String::from("false");
        let expected = [Token::False];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn true_comma() {
        let input = String::from("true,");
        let expected = [Token::True, Token::Comma];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn integer() {
        let input = String::from("123");
        let expected = [Token::Number(123.0)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn floating_point() {
        let input = String::from("1.23");
        let expected = [Token::Number(1.23)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn negative_integer() {
        let input = String::from("-123.5");
        let expected = [Token::Number(-123.5)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn number_comma() {
        let input = String::from("123,");
        let expected = [Token::Number(123.0), Token::Comma];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn a_string() {
        let input = String::from("\"rust\"");
        let expected = [Token::string("rust")];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn empty_string() {
        let input = String::from("[\"\"]");
        let expected = [Token::LeftBracket, Token::string(""), Token::RightBracket];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn unclosed_string() {
        let input = String::from("\"unclosed");
        let expected = Err(TokenizeError::UnclosedQuotes);

        let actual = tokenize(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn escaped_quote() {
        let input = String::from(r#""the \" is OK""#);
        let expected = [Token::string(r#"the \" is OK"#)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn all_punctuation() {
        let input = String::from("[{]},:");
        let expected = [
            Token::LeftBracket,
            Token::LeftBrace,
            Token::RightBracket,
            Token::RightBrace,
            Token::Comma,
            Token::Colon,
        ];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn whitespaces() {
        let input = String::from(r#" "value1": 100,     "value2": 200"#);
        let expected = [
            Token::string("value1"),
            Token::Colon,
            Token::Number(100.0),
            Token::Comma,
            Token::string("value2"),
            Token::Colon,
            Token::Number(200.0),
        ];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn array_with_null() {
        let input = String::from("[null]");
        let expected = [Token::LeftBracket, Token::Null, Token::RightBracket];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn simple_object() {
        let input = String::from("{\"key\":\"value\"}");
        let expected = [
            Token::LeftBrace,
            Token::string("key"),
            Token::Colon,
            Token::string("value"),
            Token::RightBrace,
        ];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn array_with_numbers() {
        let input = String::from("[123.4, 567.8]");
        let expected = [
            Token::LeftBracket,
            Token::Number(123.4),
            Token::Comma,
            Token::Number(567.8),
            Token::RightBracket,
        ];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn array_with_strings() {
        let input = String::from("[\"A\", \"B\"]");
        let expected = [
            Token::LeftBracket,
            Token::string("A"),
            Token::Comma,
            Token::string("B"),
            Token::RightBracket,
        ];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn array_with_true_false() {
        let input = String::from("[true, false]");
        let expected = [
            Token::LeftBracket,
            Token::True,
            Token::Comma,
            Token::False,
            Token::RightBracket,
        ];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }
}
