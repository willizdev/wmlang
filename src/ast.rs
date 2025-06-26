use crate::token::Token;

// traits
pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {}
pub trait Expression: Node {}

// ast root program
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if let Some(stmt) = self.statements.first() {
            stmt.token_literal()
        } else {
            "".to_string()
        }
    }
}

// statements
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for LetStatement {}

// expressions
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for Identifier {}
