use id_arena::{Arena, Id};

pub mod pretty_print_visitor;
pub mod visitor;

type StatementId = Id<Statement>;
type ExpressionId = Id<Expression>;

#[derive(Debug)]
pub struct Program {
    statement_arena: Arena<Statement>,
    expression_arena: Arena<Expression>,

    pub declarations: Vec<Declaration>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statement_arena: Arena::new(),
            expression_arena: Arena::new(),
            declarations: Vec::new(),
        }
    }

    pub fn create_statement(&mut self, statement: Statement) -> StatementId {
        self.statement_arena.alloc(statement)
    }

    pub fn create_expression(&mut self, expression: Expression) -> ExpressionId {
        self.expression_arena.alloc(expression)
    }

    pub fn get_statement(&self, id: StatementId) -> Option<&Statement> {
        self.statement_arena.get(id)
    }

    pub fn get_expression(&self, id: ExpressionId) -> Option<&Expression> {
        self.expression_arena.get(id)
    }

    pub fn accept_program_visitor<V: visitor::Visitor>(
        &self,
        visitor: &mut V,
    ) -> Option<V::ProgramItem> {
        visitor.visit_program(self, &self.declarations)
    }

    pub fn accept_declaration_visitor<V: visitor::Visitor>(
        &self,
        visitor: &mut V,
        decl: &Declaration,
    ) -> Option<V::DeclarationItem> {
        match *decl {
            Declaration::FunctionDeclaration {
                ref name,
                ref parameters,
                statement,
            } => visitor.visit_function_declaration(self, name, parameters, statement),
        }
    }

    pub fn accept_statement_visitor<V: visitor::Visitor>(
        &self,
        visitor: &mut V,
        id: StatementId,
    ) -> Option<V::StatementItem> {
        if let Some(stmt) = self.get_statement(id) {
            match *stmt {
                Statement::BlockStatement(ref stmts) => visitor.visit_block_statement(self, stmts),
                Statement::LetStatement {
                    ref identifier,
                    expression,
                } => visitor.visit_let_statement(self, identifier, expression),
                Statement::ExpressionStatement(expr) => {
                    visitor.visit_expression_statement(self, expr)
                }
            }
        } else {
            None
        }
    }

    pub fn accept_expression_visitor<V: visitor::Visitor>(
        &self,
        visitor: &mut V,
        id: ExpressionId,
    ) -> Option<V::ExpressionItem> {
        if let Some(expr) = self.get_expression(id) {
            match *expr {
                Expression::BinOp { op, lhs, rhs } => {
                    visitor.visit_binop_expression(self, op, lhs, rhs)
                }
                Expression::FunctionCall { func, ref args } => {
                    visitor.visit_func_call(self, func, args)
                }
                Expression::Integer(i) => visitor.visit_integer(self, i),
                Expression::Identifier(ref id) => visitor.visit_identifier(self, id),
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum Declaration {
    FunctionDeclaration {
        name: String,
        parameters: Vec<(String, Type)>,
        statement: StatementId,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    BlockStatement(Vec<StatementId>),
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
    FunctionCall {
        func: ExpressionId,
        args: Vec<ExpressionId>,
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

#[derive(Debug, Clone)]
pub enum Type {
    Unit,
    Int,
    Boolean,
}
