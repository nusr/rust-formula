use core::panic;

use crate::error::ErrorTypes;
use crate::token::{Token, TokenType};
use regex::Regex;

const EMPTY_DATA: char = '\0';

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn is_alpha(c: char) -> bool {
    c.is_alphabetic()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scanner {
    tokens: Vec<Token>,
    list: Vec<char>,
    start: usize,
    current: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            tokens: Vec::new(),
            list: source.chars().collect(),
            start: 0,
            current: 0,
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::Eof, "".to_string()));
        self.tokens.clone()
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.list.len()
    }
    fn next(&mut self) -> char {
        if self.is_at_end() {
            return EMPTY_DATA;
        }
        let c = self.list[self.current];
        self.current += 1;
        return c;
    }
    fn get_alphas(&mut self) {
        while !self.is_at_end() && is_alpha(self.peek()) {
            self.next();
        }
    }
    fn match_r1_c1(&mut self) {
        if self.match_char('[') {
            self.match_char('-');
            if !is_digit(self.peek()) {
                panic!("{}", ErrorTypes::Value.to_string())
            }
            self.get_digits();
            if self.peek() != ']' {
                panic!("{}", ErrorTypes::Value.to_string())
            }
            self.next();
        } else {
            self.get_digits();
        }
    }
    fn scan_token(&mut self) {
        let c = self.next();
        match c {
            '$' => {
                if is_alpha(self.peek()) {
                    self.get_alphas();
                    if self.match_char('$') {
                        if is_digit(self.peek()) {
                            self.get_digits();
                            self.add_token(TokenType::AbsoluteCell);
                        } else {
                            self.add_identifier();
                        }
                    } else if is_digit(self.peek()) {
                        self.get_digits();
                        self.add_token(TokenType::MixedCell);
                    } else {
                        self.add_token(TokenType::AbsoluteCell);
                    }
                } else if is_digit(self.peek()) {
                    self.get_digits();
                    self.add_token(TokenType::AbsoluteCell);
                } else {
                    self.add_identifier();
                }
            }
            'r' | 'R' => {
                self.match_r1_c1();
                if self.match_char('C') || self.match_char('c') {
                    self.match_r1_c1();
                    let text: String = self.list[self.start..self.current].iter().collect();
                    self.tokens
                        .push(Token::new(TokenType::R1C1, text.to_uppercase()));
                } else {
                    self.add_identifier();
                }
            }
            '(' => self.add_token(TokenType::LeftBracket),
            ')' => self.add_token(TokenType::RightBracket),
            ',' => self.add_token(TokenType::Comma),
            ':' => self.add_token(TokenType::Colon),
            '=' => self.add_token(TokenType::Equal),
            '<' => self.add_token(TokenType::Less),
            '>' => self.add_token(TokenType::Greater),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            '/' => self.add_token(TokenType::Slash),
            '^' => self.add_token(TokenType::Exponent),
            '&' => self.add_token(TokenType::Concatenate),
            '%' => self.add_token(TokenType::Percent),
            ';' => self.add_token(TokenType::Semicolon),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ' ' | '\r' | '\t' | '\n' => {}
            '"' | '\'' => {
                while !self.is_at_end() && self.peek() != c {
                    self.next();
                }
                if self.peek() != c {
                    panic!("{}", ErrorTypes::Value.to_string())
                }
                self.next();
                let text: String = self.list[self.start + 1..self.current - 1].iter().collect();
                self.tokens.push(Token::new(TokenType::String, text));
            }
            _ => {
                if is_digit(c) {
                    self.number();
                } else {
                    self.add_identifier();
                }
            }
        }
    }
    fn add_identifier(&mut self) {
        while !self.is_at_end() && self.any_char(self.peek()) {
            self.next();
        }
        let text: String = self.list[self.start..self.current].iter().collect();
        let t = text.to_uppercase();
        if t == "TRUE" {
            self.tokens.push(Token::new(TokenType::True, t));
            return;
        }
        if t == "FALSE" {
            self.tokens.push(Token::new(TokenType::False, t));
            return;
        }
        let re = Regex::new(r"^[A-Z]+\$\d+$").unwrap();
        if re.is_match(&text) {
            self.tokens.push(Token::new(TokenType::MixedCell, t));
            return;
        }
        self.tokens.push(Token::new(TokenType::Identifier, text));
    }
    fn any_char(&self, c: char) -> bool {
        let text = "()=:<>+-*/^&%\"{}!";
        !text.contains(c) && !self.is_whitespace(c)
    }
    fn is_whitespace(&self, c: char) -> bool {
        c == ' ' || c == '\r' || c == '\t' || c == '\n'
    }

    fn number(&mut self) {
        self.get_digits();
        let check = self.match_scientific_counting(false);
        if check {
            return;
        }
        let mut is_float = false;
        if self.match_char('.') {
            is_float = true;
            self.get_digits();
        }
        let check = self.match_scientific_counting(true);
        if check {
            return;
        }
        self.add_number_token(is_float)
    }
    fn add_number_token(&mut self, is_float: bool) {
        if is_float {
            self.add_token(TokenType::Float);
        } else {
            self.add_token(TokenType::Integer);
        }
    }
    fn match_scientific_counting(&mut self, is_float: bool) -> bool {
        if self.match_char('E') || self.match_char('e') {
            if self.match_char('+') || self.match_char('-') {
                self.get_digits();
                self.add_number_token(is_float);
                return true;
            }
            if is_digit(self.peek()) {
                self.get_digits();
                self.add_number_token(is_float);
                return true;
            }
            panic!("{}", ErrorTypes::Value.to_string())
        }
        return false;
    }
    fn match_char(&mut self, c: char) -> bool {
        if self.peek() == c {
            self.next();
            return true;
        }
        return false;
    }
    fn get_digits(&mut self) {
        while !self.is_at_end() && is_digit(self.peek()) {
            self.next();
        }
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return EMPTY_DATA;
        }
        return self.list[self.current];
    }
    fn add_token(&mut self, token_type: TokenType) {
        let text: String = self.list[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(token_type, text));
    }
}
