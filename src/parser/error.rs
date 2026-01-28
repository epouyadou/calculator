use crate::lexer::token::Token;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken { expected: Token, found: Token },
    MissingClosingParenthesis,
    InvalidExpression,
    InvalidToken { token: Token, char: u8 },
    Empty,
}