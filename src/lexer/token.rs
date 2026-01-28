#[derive(Clone, Copy, Debug)]
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