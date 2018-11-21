use super::*;

pub trait Visitor: Sized {
    type ProgramItem;
    type DeclarationItem;
    type StatementItem;
    type ExpressionItem;

    fn visit_program(
        &mut self,
        program: &Program,
        declarations: &[Declaration],
    ) -> Option<Self::ProgramItem> {
        for decl in declarations {
            program.accept_declaration_visitor(self, decl);
        }
        None
    }

    fn visit_function_declaration(
        &mut self,
        program: &Program,
        _name: &str,
        _parameters: &[(String, Type)],
        statement: StatementId,
    ) -> Option<Self::DeclarationItem> {
        program.accept_statement_visitor(self, statement);
        None
    }

    fn visit_block_statement(
        &mut self,
        program: &Program,
        statements: &[StatementId],
    ) -> Option<Self::StatementItem> {
        for &s in statements {
            program.accept_statement_visitor(self, s);
        }
        None
    }

    fn visit_let_statement(
        &mut self,
        program: &Program,
        _identifier: &str,
        expression: ExpressionId,
    ) -> Option<Self::StatementItem> {
        program.accept_expression_visitor(self, expression);
        None
    }

    fn visit_expression_statement(
        &mut self,
        program: &Program,
        expression: ExpressionId,
    ) -> Option<Self::StatementItem> {
        program.accept_expression_visitor(self, expression);
        None
    }

    fn visit_binop_expression(
        &mut self,
        program: &Program,
        _op: BinOp,
        lhs: ExpressionId,
        rhs: ExpressionId,
    ) -> Option<Self::ExpressionItem> {
        program.accept_expression_visitor(self, lhs);
        program.accept_expression_visitor(self, rhs);
        None
    }

    fn visit_func_call(
        &mut self,
        program: &Program,
        func: ExpressionId,
        args: &[ExpressionId],
    ) -> Option<Self::ExpressionItem> {
        program.accept_expression_visitor(self, func);
        for e in args {
            program.accept_expression_visitor(self, *e);
        }
        None
    }

    fn visit_integer(&mut self, _program: &Program, _i: i32) -> Option<Self::ExpressionItem> {
        // do nothing
        None
    }

    fn visit_identifier(&mut self, _program: &Program, _id: &str) -> Option<Self::ExpressionItem> {
        // do nothing
        None
    }
}
