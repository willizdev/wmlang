use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::Program;
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    pub lexer: Rc<RefCell<Lexer>>,
    pub current_token: Option<Token>,
    pub peek_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let parser: Self = Self {
            lexer: Rc::new(RefCell::new(lexer)),
            current_token: None,
            peek_token: None,
        };
        parser
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = Some(self.lexer.borrow_mut().next_token());
    }

    pub fn parse_program(&self) -> Program {
        todo!()
    }
}
