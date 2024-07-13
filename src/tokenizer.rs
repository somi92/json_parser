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
    let ch = chars[*index];
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
        c if c.is_ascii_digit() => tokenize_float(chars, index)?,

        _ => todo!("implement other tokens"),
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
            c if c.is_ascii_digit() => unparsed_num.push(c),
            c if c == '.' && !is_decimal => {
                unparsed_num.push('.');
                is_decimal = true;
            }
            _ => break,
        }
        *index += 1;
    }

    match unparsed_num.parse() {
        Ok(f) => Ok(Token::Number(f)),
        Err(err) => Err(TokenizeError::ParseNumberError(err)),
    }
}

#[cfg(test)]
mod tests {
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
}
