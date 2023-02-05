use crate::Token;

pub enum Expr<'a> {
    Binary(Box<Expr<'a>>, Token<'a>, Box<Expr<'a>>),
    Unary(Box<Expr<'a>>, Token<'a>),
    Literal(Token<'a>),
    Grouping(Box<Expr<'a>>),
}
