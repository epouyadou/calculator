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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::expr::Expr;
    use crate::lexer::token::Token;

    // Helper to keep tests clean
    fn parse_to_ast(input: &str) -> Box<Expr> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse().expect("Failed to parse expression")
    }

    #[test]
    fn test_precedence_multiplication() {
        // 1 + 2 * 3 should be 1 + (2 * 3)
        let ast = parse_to_ast("1 + 2 * 3");

        if let Expr::Binary { left, op, right } = *ast {
            assert_eq!(op, Token::Plus);
            assert!(matches!(*left, Expr::Int(1)));

            if let Expr::Binary { op: op_inner, .. } = *right {
                assert_eq!(op_inner, Token::Star);
            } else {
                panic!("Right side should be a multiplication binary expression");
            }
        } else {
            panic!("Root should be an addition binary expression");
        }
    }

    #[test]
    fn test_implicit_multiplication() {
        // 2(3 + 4) should parse as 2 * (3 + 4)
        let ast = parse_to_ast("2(3 + 4)");

        match *ast {
            Expr::Binary { op, .. } => assert_eq!(op, Token::Star),
            _ => panic!("Expected implicit multiplication to be a Star binary expression"),
        }
    }

    #[test]
    fn test_unary_negative() {
        let ast = parse_to_ast("-5");
        match *ast {
            Expr::Unary { op, .. } => assert_eq!(op, Token::Minus),
            _ => panic!("Expected unary minus expression"),
        }
    }

    #[test]
    fn test_grouping() {
        let ast = parse_to_ast("(1 + 2)");
        assert!(matches!(*ast, Expr::Grouping(_)));
    }

    #[test]
    fn test_error_missing_paren() {
        let lexer = Lexer::new("(1 + 2");
        let mut parser = Parser::new(lexer);
        let result = parser.parse();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParseError::MissingClosingParenthesis);
    }

    #[test]
    fn test_empty_input() {
        let lexer = Lexer::new("");
        let mut parser = Parser::new(lexer);
        let result = parser.parse();

        assert!(matches!(result, Err(ParseError::Empty)));
    }
}