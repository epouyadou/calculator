
pub enum Token {
    Integer(i64),
    Float(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
    EOF,
    Illegal(u8),
}

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

    fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
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
