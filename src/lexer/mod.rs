pub mod token;

use crate::lexer::token::Token;

pub struct Lexer<'a> {
    source: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            source: input.as_bytes(),
            pos: 0,
        }
    }

    fn current_byte(&self) -> Option<u8> {
        self.source.get(self.pos).cloned()
    }

    fn peek_byte(&self, distance: usize) -> Option<u8> {
        self.source.get(self.pos + distance).cloned()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn is_next_char_a_number(&self) -> bool {
        if let Some(next_byte) = self.peek_byte(1) {
            return next_byte.is_ascii_digit()
        }
        false
    }

    fn read_number(&mut self) -> Token {
        let start_pos = self.pos;
        let mut has_dot = false;

        while let Some(byte_char) = self.current_byte() {
            if byte_char.is_ascii_digit() {
                self.advance();
            } else if byte_char == b'.' && !has_dot {
                if self.is_next_char_a_number() {
                    has_dot = true;
                    self.advance();
                    continue;
                }
                break;
            } else {
                break;
            }
        }

        let slice = &self.source[start_pos..self.pos];
        let val_str = std::str::from_utf8(slice).expect("Invalid UTF-8 sequence");

        if has_dot {
            Token::Float(val_str.parse().unwrap())
        } else {
            Token::Integer(val_str.parse().unwrap())
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(b) = self.current_byte() {
            if b.is_ascii_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let byte_char = match self.current_byte() {
            Some(c) => c,
            None => return Token::EOF,
        };

        match byte_char {
            b'+' => { self.advance(); Token::Plus }
            b'-' => { self.advance(); Token::Minus }
            b'*' => { self.advance(); Token::Star }
            b'/' => { self.advance(); Token::Slash }
            b'(' => { self.advance(); Token::LeftParen }
            b')' => { self.advance(); Token::RightParen }

            b'0'..=b'9' => self.read_number(),

            _ => {
                self.advance();
                Token::Illegal(byte_char)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::Token;

    #[test]
    fn test_basic_arithmetic_tokens() {
        let input = "1 + 2 * (3 / 4) - 5";
        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token::Integer(1),
            Token::Plus,
            Token::Integer(2),
            Token::Star,
            Token::LeftParen,
            Token::Integer(3),
            Token::Slash,
            Token::Integer(4),
            Token::RightParen,
            Token::Minus,
            Token::Integer(5),
            Token::EOF,
        ];

        for token in expected {
            let next = lexer.next_token();
            assert_eq!(next, token);
        }
    }

    #[test]
    fn test_floats_and_decimals() {
        let input = "10.5 + 0.75";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Float(10.5));
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Float(0.75));
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_implicit_multiplication_lexing() {
        // The lexer doesn't "know" it's implicit multiplication,
        // it just needs to see the tokens correctly for the parser.
        let input = "2(3)";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Integer(2));
        assert_eq!(lexer.next_token(), Token::LeftParen);
        assert_eq!(lexer.next_token(), Token::Integer(3));
        assert_eq!(lexer.next_token(), Token::RightParen);
    }

    #[test]
    fn test_illegal_characters() {
        let input = "1 & @";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Integer(1));
        assert_eq!(lexer.next_token(), Token::Illegal(b'&'));
        assert_eq!(lexer.next_token(), Token::Illegal(b'@'));
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "   42  \n \t  + 7 ";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Integer(42));
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Integer(7));
        assert_eq!(lexer.next_token(), Token::EOF);
    }
}