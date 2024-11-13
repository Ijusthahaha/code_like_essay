use nom::character::complete::space0;
use crate::parser::parse_token;
use crate::token::Token;

pub(crate) fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let input_to_lowercase = input.to_lowercase();
    let mut remaining = String::as_str(&input_to_lowercase);

    while !remaining.is_empty() {
        let (new_remaining, _) = space0::<_, nom::error::Error<&str>>(remaining).unwrap();
        remaining = new_remaining;
        if let Ok((new_remaining, token)) = parse_token(remaining) {
            if let Token::Skip = token {
                remaining = new_remaining;
                continue;
            }
            tokens.push(token);
            remaining = new_remaining;
        } else {
            panic!("Unexpected token in input: {}", remaining);
        }
    }
    tokens
}
