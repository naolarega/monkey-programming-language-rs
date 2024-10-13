use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> &str;
}

pub enum Statement {
    LetStatement {
        token: Token,
        name: Expression,
        value: Expression,
    },
}

impl Statement {
    fn statement_node(&self) {}
}

impl Node for Statement {
    fn token_literal(&self) -> &str {
        use Statement::*;

        match self {
            LetStatement { token, .. } => token.literal(),
        }
    }
}

pub enum Expression {
    Identifier { token: Token, value: String },
}

impl Expression {
    fn expression_node() {}
}

impl Node for Expression {
    fn token_literal(&self) -> &str {
        use Expression::*;

        match self {
            Identifier { token, .. } => token.literal(),
        }
    }
}

pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        if let Some(first_statement) = self.statements.get(0) {
            return first_statement.token_literal();
        }

        ""
    }
}
