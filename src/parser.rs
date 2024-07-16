use crate::{tokenizer::Token, Value};

#[derive(Debug, PartialEq)]
enum TokenParseError {}

fn parse_tokens(tokens: &[Token], index: &mut usize) -> Result<Value, TokenParseError> {
    let token = &tokens[*index];

    match token {
        Token::Null => Ok(Value::Null),
        Token::False => Ok(Value::Boolean(false)),
        Token::True => Ok(Value::Boolean(true)),
        Token::Number(number) => Ok(Value::Number(*number)),
        Token::String(string) => todo!(),
        Token::LeftBracket => todo!(),
        Token::LeftBrace => todo!(),
        _ => todo!(),
    }
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
}
