use crate::Value;
#[derive(PartialEq, Clone)]
pub enum Token {
    Assign,
    Comma,
    Literal(Value),
    Plus,
    Minus,
    Star,
    ForwardSlash,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Not,
    LeftParens,
    RightParens,
    Identifier(String),
    Semicolon,
    Concatenation,
}
