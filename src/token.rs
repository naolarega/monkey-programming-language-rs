use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,

    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // KeyWords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Display for TokenType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        use TokenType::*;

        let string_token_type = match self {
            Illegal => "ILLEGAL",
            Eof => "EOF",
            Ident => "IDENT",
            Int => "INT",
            Assign => "=",
            Plus => "+",
            Minus => "-",
            Bang => "!",
            Asterisk => "*",
            Slash => "/",
            Lt => "<",
            Gt => ">",
            Eq => "==",
            NotEq => "!=",
            Comma => ",",
            Semicolon => ";",
            Lparen => "(",
            Rparen => ")",
            Lbrace => "{",
            Rbrace => "}",
            Function => "FUNCTION",
            Let => "LET",
            True => "TRUE",
            False => "FALSE",
            If => "IF",
            Else => "ELSE",
            Return => "RETURN",
        };

        write!(formatter, "{}", string_token_type)
    }
}

pub const KEYWORDS: [(&str, TokenType); 7] = [
    ("fn", TokenType::Function),
    ("let", TokenType::Let),
    ("true", TokenType::True),
    ("false", TokenType::False),
    ("if", TokenType::If),
    ("else", TokenType::Else),
    ("return", TokenType::Return),
];

pub fn look_up_ident(ident: &str) -> Option<TokenType> {
    KEYWORDS.iter().find_map(|(keyword, token_type)| {
        if *keyword == ident {
            Some(*token_type)
        } else {
            None
        }
    })
}

#[derive(Clone, PartialEq, Eq)]
pub struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type.clone()
    }

    pub fn literal(&self) -> &str {
        &self.literal
    }
}
