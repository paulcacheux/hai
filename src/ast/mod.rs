use id_arena::{Arena, Id};

pub mod visitor;
pub mod pretty_print_visitor;

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

    pub fn get_statement(&self, id: StatementId) -> Option<&Statement> {
        self.statement_arena.get(id)
    }
    
    pub fn get_expression(&self, id: ExpressionId) -> Option<&Expression> {
        self.expression_arena.get(id)
    }

    pub fn accept_program_visitor<V: visitor::Visitor>(&self, visitor: &mut V) {
        visitor.visit_progam(self, &self.statements)
    }

    pub fn accept_statement_visitor<V: visitor::Visitor>(&self, visitor: &mut V, id: StatementId) {
        if let Some(stmt) = self.get_statement(id) {
            match *stmt {
                Statement::LetStatement { ref identifier, expression } => visitor.visit_let_statement(self, identifier, expression),
                Statement::ExpressionStatement(expr) => visitor.visit_expression_statement(self, expr)
            }
        }
    }

    pub fn accept_expression_visitor<V: visitor::Visitor>(&self, visitor: &mut V, id: ExpressionId) {
        if let Some(expr) = self.get_expression(id) {
            match *expr {
                Expression::BinOp { op, lhs, rhs } => visitor.visit_binop_expression(self, op, lhs, rhs),
                Expression::Integer(i) => visitor.visit_integer(self, i),
                Expression::Identifier(ref id) => visitor.visit_identifier(self, id),
            }
        }
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