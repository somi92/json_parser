use crate::{tokenizer::Token, Value};

#[derive(Debug, PartialEq)]
enum TokenParseError {}

type ParseResult = Result<Value, TokenParseError>;

fn parse_tokens(tokens: &[Token], index: &mut usize) -> ParseResult {
    let token = &tokens[*index];

    match token {
        Token::Null => Ok(Value::Null),
        Token::False => Ok(Value::Boolean(false)),
        Token::True => Ok(Value::Boolean(true)),
        Token::Number(number) => Ok(Value::Number(*number)),
        Token::String(string) => parse_string(string),
        Token::LeftBracket => todo!(),
        Token::LeftBrace => todo!(),
        _ => todo!(),
    }
}

fn parse_string(input: &str) -> ParseResult {
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
                'u' => todo!("implement hex code escapes"),
                _ => output.push(next_char),
            }
            in_escape_mode = false;
        } else if next_char == '\\' {
            in_escape_mode = true;
        } else {
            output.push(next_char);
        }
    }

    Ok(Value::String(output))
}

#[cfg(test)]
mod tests {
    use crate::{tokenizer::Token, Value};

    use super::parse_tokens;

    fn assert_parse_tokens(input: &[Token], expected: Value) {
        let actual = parse_tokens(input, &mut 0).unwrap();
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
}
