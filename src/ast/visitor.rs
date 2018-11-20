use super::*;

pub trait Visitor: Sized {
    fn visit_program(&mut self, program: &Program, statements: &[StatementId]) {
        for &s in statements {
            program.accept_statement_visitor(self, s)
        }
    }

    fn visit_let_statement(&mut self, program: &Program, _identifier: &str, expression: ExpressionId) {
        program.accept_expression_visitor(self, expression);
    }

    fn visit_expression_statement(&mut self, program: &Program, expression: ExpressionId) {
        program.accept_expression_visitor(self, expression)
    }

    fn visit_binop_expression(&mut self, program: &Program, _op: BinOp, lhs: ExpressionId, rhs: ExpressionId) {
        program.accept_expression_visitor(self, lhs);
        program.accept_expression_visitor(self, rhs);
    }

    fn visit_integer(&mut self, _program: &Program, _i: i32) {
        // do nothing
    }

    fn visit_identifier(&mut self, _program: &Program, _id: &str) {
        // do nothing
    }
}