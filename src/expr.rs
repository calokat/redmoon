use crate::{Token, Value};

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Box<Expr>, Token),
    Literal(Value),
    Grouping(Box<Expr>),
    Var(String)
}
