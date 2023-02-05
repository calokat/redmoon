use crate::Token;

pub struct Lexer {
    expr_str: String,
    current: usize
}

impl<'a> Lexer {
    pub fn new(s: String) -> Self {
        return Self { expr_str: s, current: 0 };
    }

    pub fn tokenize(&mut self) -> Vec<Token<'a>> {
        let mut ret: Vec<Token<'a>> = vec![];
        while self.current < self.expr_str.len() {
            if let Some(c) = self.expr_str.chars().nth(self.current) {
                if c.is_whitespace() {
                    self.current += 1;
                } else if c.is_numeric() {
                    ret.push(self.lex_number());
                } else if self.is_operator(c) {
                    ret.push(self.lex_operator(c));
                    self.current += 1;
                } else if c == '(' {
                    ret.push(Token::LeftParens);
                    self.current += 1;
                } else if c == ')' {
                    ret.push(Token::RightParens);
                    self.current += 1;
            }
            }
        }
        return ret;
    }

    fn lex_number(&mut self) -> Token<'a> {
        let scan_start = self.current;
        while self.current < self.expr_str.len() {
            let c = self.expr_str.chars().nth(self.current).expect("lex_number(): Access should not be out of bounds");
            if c.is_numeric() || c == '.' {
                self.current += 1;
                continue;
            }
            break;
        };
            return Token::LiteralNumber(self.expr_str[scan_start..self.current].parse().expect("lex_number(): Above code should ensure a valid parse"));
    }

    fn lex_operator(&self, c: char) -> Token<'a> {
        match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => Token::ForwardSlash,
            '*' => Token::Star,
            _ => panic!("Unknown symbol")
        }
    }

    fn is_operator(&self, c: char) -> bool {
        match c {
            '+' => true,
            '-' => true,
            '/' => true,
            '*' => true,
            _ => false
        }
    }
}
