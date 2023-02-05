#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum Token<'a> {
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
    Not,
    LeftParens,
    RightParens
}
