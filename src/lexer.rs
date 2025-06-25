use crate::token::Token;
use crate::token::TokenType;

pub struct Lexer {
    pub input: String,
    pub position: usize, // current char position
    pub read_pos: usize, // next char position
    pub ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer: Self = Self {
            input: input,
            position: 0,
            read_pos: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = '\0';
        } else {
            // unicode not supported
            self.ch = self.input.as_bytes()[self.read_pos] as char;
        }
        self.position = self.read_pos;
        self.read_pos += 1;
    }

    pub fn peek_char(&self) -> char {
        if self.read_pos >= self.input.len() {
            return '\0';
        } else {
            return self.input.as_bytes()[self.read_pos] as char;
        }
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;
        self.skip_whitespace();
        match self.ch {
            // operators
            '=' => {
                if self.peek_char() == '=' {
                    let mut literal: String = String::from(self.ch);
                    self.read_char();
                    literal.push(self.ch);
                    let ttype: TokenType = TokenType::Equal;
                    tok = Token::new(ttype, literal);
                } else {
                    tok = Token::new(TokenType::Assign, self.ch.to_string());
                }
            }
            '+' => {
                tok = Token::new(TokenType::Plus, self.ch.to_string());
            }
            '-' => {
                tok = Token::new(TokenType::Minus, self.ch.to_string());
            }
            '!' => {
                if self.peek_char() == '=' {
                    let mut literal: String = String::from(self.ch);
                    self.read_char();
                    literal.push(self.ch);
                    let ttype: TokenType = TokenType::NotEqual;
                    tok = Token::new(ttype, literal);
                } else {
                    tok = Token::new(TokenType::Bang, self.ch.to_string());
                }
            }
            '*' => {
                tok = Token::new(TokenType::Asterisk, self.ch.to_string());
            }
            '/' => {
                tok = Token::new(TokenType::Slash, self.ch.to_string());
            }
            '<' => {
                tok = Token::new(TokenType::LThan, self.ch.to_string());
            }
            '>' => {
                tok = Token::new(TokenType::GThan, self.ch.to_string());
            }
            // delimeters
            ',' => {
                tok = Token::new(TokenType::Comma, self.ch.to_string());
            }
            ';' => {
                tok = Token::new(TokenType::Semicolon, self.ch.to_string());
            }
            '(' => {
                tok = Token::new(TokenType::LParen, self.ch.to_string());
            }
            ')' => {
                tok = Token::new(TokenType::RParen, self.ch.to_string());
            }
            '{' => {
                tok = Token::new(TokenType::LBrace, self.ch.to_string());
            }
            '}' => {
                tok = Token::new(TokenType::RBrace, self.ch.to_string());
            }
            // end of file
            '\0' => {
                tok = Token::new(TokenType::Eof, self.ch.to_string());
            }
            _ => {
                // identifiers and literals
                if is_letter(self.ch) {
                    let literal: String = self.read_identifier();
                    let ttype: TokenType = Token::lookup_ident(&literal);
                    return Token::new(ttype, literal);
                } else if is_digit(self.ch) {
                    let literal: String = self.read_number();
                    let ttype: TokenType = TokenType::Int;
                    return Token::new(ttype, literal);
                } else {
                    // Illegal character
                    tok = Token::new(TokenType::Illegal, self.ch.to_string());
                }
            }
        }
        self.read_char();
        tok
    }

    pub fn read_identifier(&mut self) -> String {
        let mut ident: String = String::new();
        while is_letter(self.ch) {
            ident.push(self.ch);
            self.read_char();
        }
        ident
    }

    pub fn read_number(&mut self) -> String {
        let mut number: String = String::new();
        while is_digit(self.ch) {
            number.push(self.ch);
            self.read_char();
        }
        number
    }

    pub fn skip_whitespace(&mut self) {
        while is_whitespace(self.ch) {
            self.read_char();
        }
    }
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_letter(ch: char) -> bool {
    ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') || ch == '_'
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}
