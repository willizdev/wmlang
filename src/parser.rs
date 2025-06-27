use std::cell::RefCell;
use std::rc::Rc;

use crate::lexer::Lexer;
use crate::ast::{Expression, Identifier, LetStatement, Program, ReturnStatement, Statement};
use crate::token::{Token, TokenType};

pub struct Parser {
    pub lexer: Rc<RefCell<Lexer>>,
    pub current_token: Option<Token>,
    pub peek_token: Option<Token>,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser: Self = Self {
            lexer: Rc::new(RefCell::new(lexer)),
            current_token: None,
            peek_token: None,
            errors: Vec::new(),
        };

        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn peek_error(&mut self, token: TokenType) {
        let message = format!(
            "expected next token to be {:?}, got {:?} instead",
            token,
            self.peek_token.as_ref().unwrap().ttype
        );
        self.errors.push(message);
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.borrow_mut().next_token());
    }

    pub fn match_current_token(&self, token: TokenType) -> bool {
        self.current_token.as_ref().unwrap().ttype == token
    }

    pub fn match_peek_token(&self, token: TokenType) -> bool {
        self.peek_token.as_ref().unwrap().ttype == token
    }

    pub fn expect_peek(&mut self, token: TokenType) -> bool {
        let expect: bool = self.match_peek_token(token.clone());
        if expect {
            self.next_token();
        } else {
            self.peek_error(token);
        }
        expect
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program: Program = Program::new();

        while self.current_token.as_ref().unwrap().ttype != TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.current_token.as_ref().unwrap().ttype {
            TokenType::Let => Some(Box::new(self.parse_let_stmt()?) as Box<dyn Statement>),
            TokenType::Return => Some(Box::new(self.parse_return_stmt()?) as Box<dyn Statement>),
            _ => None,
        }
    }

    pub fn parse_expression(&self) -> Option<Box<dyn Expression>> {
        todo!()
    }

    pub fn parse_return_stmt(&mut self) -> Option<ReturnStatement> {
        let return_token: Token = self.current_token.clone().unwrap();
        self.next_token();

        let expr: Box<dyn Expression> = self.parse_expression()?;

        if !self.expect_peek(TokenType::Semicolon) {
            return None;
        }

        let stmt: ReturnStatement = ReturnStatement {
            token: return_token,
            value: expr,
        };

        Some(stmt)
    }

    pub fn parse_let_stmt(&mut self) -> Option<LetStatement> {
        let let_token: Token = self.current_token.clone().unwrap();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name: Identifier = Identifier {
            token: self.current_token.clone().unwrap(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        let expr: Box<dyn Expression> = self.parse_expression()?;

        if !self.expect_peek(TokenType::Semicolon) {
            return None;
        }

        let stmt: LetStatement = LetStatement {
            token: let_token,
            name: name,
            value: expr,
        };

        Some(stmt)
    }
}
