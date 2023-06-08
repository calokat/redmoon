use crate::{Token, Value};
#[derive(Clone, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Box<Expr>, Token),
    Literal(Value),
    Grouping(Box<Expr>),
    Var(String),
    Exprlist(Vec<Expr>),
    FunctionCall(Box<Expr>, Vec<Expr>),
    Accessor(Box<Expr> /* Table being accessed */, Box<Expr> /* Expr that is accessing */),
    FieldList(Vec<(Box<Expr>, Box<Expr>)>),
    Varargs,
}
