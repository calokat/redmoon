use crate::Token;

#[derive(Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Box<Expr>, Token),
    Literal(Token),
    Grouping(Box<Expr>),
    Var(String)
}
