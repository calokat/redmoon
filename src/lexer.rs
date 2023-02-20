use crate::Token;
use crate::Value;
pub struct Lexer<'a> {
    expr_str: &'a str,
    current: usize
}

const RESERVED_WORDS: [&str; 22] = [
    "and",
    "break",
    "do",
    "else",
    "elseif",
    "end",
    "false",
    "for",
    "function",
    "goto",
    "if",
    "in",
    "local",
    "nil",
    "not",
    "or",
    "return",
    "repeat",
    "then",
    "true",
    "until",
    "while",
];

const RESERVED_WORDS_TOKENS: [Token; 22] = [
    Token::And,
    Token::Break,
    Token::Do,
    Token::Else,
    Token::Elseif,
    Token::End,
    Token::False,
    Token::For,
    Token::Function,
    Token::Goto,
    Token::If,
    Token::In,
    Token::Local,
    Token::Nil,
    Token::Not,
    Token::Or,
    Token::Return,
    Token::Repeat,
    Token::Then,
    Token::True,
    Token::Until,
    Token::While,
];

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {    
        return Self { expr_str: s, current: 0 };
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut ret: Vec<Token> = vec![];
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
            } else if c == ',' {
                ret.push(Token::Comma);
                self.advance();
            } else if c.is_alphabetic() ||
            c == '_' {
                ret.push(self.lex_identifier());
            } else if c == ';' {
                ret.push(Token::Semicolon);
                self.advance();
            } else if c == '"' {
                self.advance();
                ret.push(self.lex_string());
            } else if c == '.' {
                if let Some(nc) = self.peek_next_char() {
                    if nc.is_numeric() {
                        ret.push(self.lex_number());
                    } else if nc == '.' {
                        self.advance();
                        self.advance();
                        if self.current_char() == '.' {
                            panic!("Varargs are coming soon!");
                        } else {
                            ret.push(Token::Concatenation);
                        }
                    } else {
                        ret.push(Token::Period);
                        self.advance();
                    }
                }
            } else if c == '{' {
                ret.push(Token::LeftCurlyBrace);
                self.advance();
            } else if c == '}' {
                ret.push(Token::RightCurlyBrace);
                self.advance();
            } else if c == '[' {
                ret.push(Token::LeftSquareBracket);
                self.advance();
            } else if c == ']' {
                ret.push(Token::RightSquareBracket);
                self.advance();
            } else {
                panic!("Cannot lex current sequence");
            }
        }
        return ret;
    }

    fn lex_string(&mut self) -> Token {
        let scan_start = self.current;
        while self.current_char() != '"' && self.current < self.expr_str.len() {
            self.advance();
        }
        assert!(self.current_char() == '"', "Missing closing '\"'");
        let ret = Token::Literal(Value::String(self.expr_str[scan_start..self.current].into()));
        self.advance();
        return ret;
    }

    fn lex_number(&mut self) -> Token {
        let scan_start = self.current;
        while self.current < self.expr_str.len() {
            let c = self.current_char();
            if c.is_numeric() || c == '.' {
                self.advance();
                continue;
            }
            break;
        };
            return Token::Literal(Value::Number(self.expr_str[scan_start..self.current].parse().expect("lex_number(): Above code should ensure a valid scan")));
    }

    fn lex_identifier(&mut self) -> Token {
        let scan_start = self.current;
        while self.current < self.expr_str.len() {
            let c = self.current_char();
            if c.is_alphanumeric() ||
             c == '_' {
                self.advance();
                continue;
            }
            break;
        };

        if let Ok(r_idx) = RESERVED_WORDS.binary_search(&&self.expr_str[scan_start..self.current]) {
            return match r_idx {
                6 => Token::Literal(Value::Boolean(false)),
                13 => Token::Literal(Value::Nil),
                19 => Token::Literal(Value::Boolean(true)),
                _ => RESERVED_WORDS_TOKENS[r_idx].clone()
            }
        }
        return Token::Identifier(self.expr_str[scan_start..self.current].into());
    }

    fn lex_operator(&mut self, c: char) -> Token {
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

    fn at_eof(&self) -> bool {
        return self.current == (self.expr_str.len() - 1);
    }

    fn peek_next_char(&self) -> Option<char> {
        if self.at_eof() {
            return None;
        }
        return self.expr_str.chars().nth(self.current + 1);
    }
}
