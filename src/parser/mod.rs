pub mod expr;
pub mod error;

use crate::lexer::Lexer;
use crate::lexer::token::Token;
use crate::parser::expr::Expr;
use crate::parser::error::ParseError;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Parser<'a> {
        let first_token = lexer.next_token();
        Self {
            lexer,
            current_token: first_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn consume(&mut self, expected: Token) -> Result<(), ParseError> {
        if std::mem::discriminant(&self.current_token) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken { expected: expected.clone(), found: self.current_token.clone() })
        }
    }

    fn parse_primary(&mut self) -> Result<Box<Expr>, ParseError> {
        match self.current_token {
            Token::Integer(i) => {
                self.advance();
                Ok(Box::new(Expr::Int(i)))
            },
            Token::Float(f) => {
                self.advance();
                Ok(Box::new(Expr::Float(f)))
            },
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_addition()?;
                self.consume(Token::RightParen).map_err(|_| ParseError::MissingClosingParenthesis)?;
                Ok(Box::new(Expr::Grouping(expr)))
            },
            Token::Illegal(u) => Err(ParseError::InvalidToken { token: self.current_token.clone(), char: u }),
            _ => Err(ParseError::InvalidExpression)
        }
    }

    fn parse_unary(&mut self) -> Result<Box<Expr>, ParseError> {
        if matches!(self.current_token, Token::Minus) {
            let op = self.current_token.clone();
            self.advance();
            let right = self.parse_primary()?;
            return Ok(Box::new(Expr::Unary {op, right}));
        }

        self.parse_primary()
    }

    fn parse_multiplicative(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut left = self.parse_unary()?;

        while matches!(self.current_token, Token::Star | Token::Slash | Token::LeftParen) {
            let op = match self.current_token {
                Token::LeftParen => Token::Star,
                _ => {
                    let token = self.current_token.clone();
                    self.advance();
                    token
                },
            };

            let right = self.parse_unary()?;
            left = Box::new(Expr::Binary { left, op, right });
        }

        Ok(left)
    }

    fn parse_addition(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut left = self.parse_multiplicative()?;

        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let op = self.current_token.clone();
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Box::new(Expr::Binary { left, op, right });
        }

        Ok(left)
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, ParseError> {
        if matches!(self.current_token, Token::EOF) {
            return Err(ParseError::Empty);
        }
        self.parse_addition()
    }
}