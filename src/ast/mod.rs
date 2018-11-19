use id_arena::{Arena, Id};

type StatementId = Id<Statement>;
type ExpressionId = Id<Expression>;

#[derive(Debug)]
pub struct Program {
    statement_arena: Arena<Statement>,
    expression_arena: Arena<Expression>,

    pub statements: Vec<StatementId>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statement_arena: Arena::new(),
            expression_arena: Arena::new(),
            statements: Vec::new(),
        }
    }

    pub fn create_statement(&mut self, statement: Statement) -> StatementId {
        self.statement_arena.alloc(statement)
    }

    pub fn create_expression(&mut self, expression: Expression) -> ExpressionId {
        self.expression_arena.alloc(expression)
    }

    pub fn append_statement(&mut self, id: StatementId) {
        self.statements.push(id);
    }

    pub fn create_append_statement(&mut self, statement: Statement) {
        let id = self.create_statement(statement);
        self.append_statement(id)
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    LetStatement {
        identifier: String,
        expression: ExpressionId,
    },
    ExpressionStatement(ExpressionId),
}

#[derive(Debug, Clone)]
pub enum Expression {
    BinOp {
        op: BinOp,
        lhs: ExpressionId,
        rhs: ExpressionId,
    },
    Integer(i32),
    Identifier(String),
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Divide,
}