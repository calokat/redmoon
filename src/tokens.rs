#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum Token<'a> {
    Assign,
    LiteralNumber(f32),
    LiteralString(&'a str),
    True,
    False,
    Nil,
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
    RightParens
}
