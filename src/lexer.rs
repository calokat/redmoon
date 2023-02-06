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
            let c = self.current_char();
            if c.is_whitespace() {
                self.advance();
            } else if c.is_numeric() {
                ret.push(self.lex_number());
            } else if self.is_operator(c) {
                ret.push(self.lex_operator(c));
            } else if c == '(' {
                ret.push(Token::LeftParens);
                self.advance();
            } else if c == ')' {
                ret.push(Token::RightParens);
                self.advance();
            }
        }
        return ret;
    }

    fn lex_number(&mut self) -> Token<'a> {
        let scan_start = self.current;
        while self.current < self.expr_str.len() {
            let c = self.current_char();
            if c.is_numeric() || c == '.' {
                self.advance();
                continue;
            }
            break;
        };
            return Token::LiteralNumber(self.expr_str[scan_start..self.current].parse().expect("lex_number(): Above code should ensure a valid parse"));
    }

    fn lex_operator(&mut self, c: char) -> Token<'a> {
         match c {
            '+' => {self.advance(); Token::Plus},
            '-' => {self.advance(); Token::Minus},
            '/' => {self.advance(); Token::ForwardSlash},
            '*' => {self.advance(); Token::Star},
            '=' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    return Token::Equals;
                }
                return Token::Assign;
            },
            '<' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    return Token::LessThanOrEqual;
                }
                return Token::LessThan;
            },
            '>' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    return Token::GreaterThanOrEqual;
                }
                return Token::GreaterThan;
            }
            _ => panic!("Unknown symbol")
        }
    }

    fn is_operator(&self, c: char) -> bool {
        match c {
            '+' => true,
            '-' => true,
            '/' => true,
            '*' => true,
            '<' => true,
            '>' => true,
            '=' => true,
            _ => false
        }
    }

    fn advance(&mut self) {
        if self.current < self.expr_str.len() {
            self.current += 1;
        } else {
            println!("Lexer internal error: reading past end of buffer");
        }
    }

    fn current_char(&self) -> char {
        return self.expr_str.chars().nth(self.current).expect("Lexer should not be out of bounds");
    }
}
