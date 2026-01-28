use crate::lexer::token::Token;

#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Grouping (Box<Expr>),
}