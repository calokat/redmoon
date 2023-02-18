use crate::{Token, Value};
#[derive(Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Box<Expr>, Token),
    Literal(Value),
    Grouping(Box<Expr>),
    Var(String),
    Exprlist(Vec<Expr>),
    FunctionCall(Box<Expr>, Vec<Expr>)
}
