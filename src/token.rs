#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Eof,
    Illegal,
    // identifiers and literals
    Ident,
    Int,
    // operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LThan,
    GThan,
    Equal,
    NotEqual,
    // delimeters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    // keywords
    Let,
    Function,
    True,
    False,
    If,
    Else,
    Return,
}

static KEYWORDS: &[(&str, TokenType)] = &[
    ("let", TokenType::Let),
    ("fn", TokenType::Function),
    ("true", TokenType::True),
    ("false", TokenType::False),
    ("if", TokenType::If),
    ("else", TokenType::Else),
    ("return", TokenType::Return),
];

pub struct Token {
    pub ttype: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(ttype: TokenType, literal: String) -> Self {
        Self { ttype, literal }
    }

    pub fn lookup_ident(ident: &String) -> TokenType {
        for (keyword, tok) in KEYWORDS.iter() {
            if ident == keyword {
                return tok.clone();
            }
        }
        TokenType::Ident
    }
}
