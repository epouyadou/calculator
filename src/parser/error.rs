use crate::lexer::token::Token;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ParseError {
    UnexpectedToken { expected: Token, found: Token },
    MissingClosingParenthesis,
    InvalidExpression,
    InvalidToken { token: Token, char: u8 },
    Empty,
}