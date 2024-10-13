use crate::{
    token::{look_up_ident, Token, TokenType},
    utils::{is_digit, is_letter},
};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut new_lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: char::default(),
        };

        new_lexer.read_char();

        new_lexer
    }

    pub fn next_token(&mut self) -> Token {
        use crate::token::TokenType::*;

        self.skip_whitespace();

        let mut literal = None;
        let mut already_read = false;

        let token = Token::new(
            match self.ch {
                '=' => {
                    if self.peek_char() == '=' {
                        self.read_char();

                        literal = Some(String::from("=="));

                        Eq
                    } else {
                        Assign
                    }
                }
                ';' => Semicolon,
                '(' => Lparen,
                ')' => Rparen,
                ',' => Comma,
                '+' => Plus,
                '-' => Minus,
                '!' => {
                    if self.peek_char() == '=' {
                        self.read_char();

                        literal = Some(String::from("!="));

                        NotEq
                    } else {
                        Bang
                    }
                }
                '/' => Slash,
                '*' => Asterisk,
                '<' => Lt,
                '>' => Gt,
                '{' => Lbrace,
                '}' => Rbrace,
                '\0' => Eof,
                _ => {
                    if is_letter(&self.ch) {
                        literal = Some(self.read_identifier());
                        already_read = true;

                        if let Some(ref ident) = literal {
                            if let Some(token_type) = look_up_ident(ident) {
                                token_type
                            } else {
                                Ident
                            }
                        } else {
                            unreachable!()
                        }
                    } else if is_digit(&self.ch) {
                        literal = Some(self.read_number());
                        already_read = true;

                        Int
                    } else {
                        Illegal
                    }
                }
            },
            if let Some(ident) = literal {
                ident
            } else {
                self.ch.to_string()
            },
        );

        if !already_read {
            self.read_char();
        }

        token
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return char::default();
        }

        if let Some(ch) = self
            .input
            .chars()
            .collect::<Vec<char>>()
            .get(self.read_position)
        {
            *ch
        } else {
            char::default()
        }
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while is_digit(&self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while [' ', '\t', '\n', '\r'].contains(&self.ch) {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while is_letter(&self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    fn read_char(&mut self) {
        if self.read_position > self.input.len() {
            self.ch = char::default();
        } else {
            self.ch = match self
                .input
                .chars()
                .collect::<Vec<char>>()
                .get(self.read_position)
            {
                Some(ch) => *ch,
                None => char::default(),
            };
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();

        if let TokenType::Eof = token.token_type() {
            return None;
        }

        Some(token)
    }
}
