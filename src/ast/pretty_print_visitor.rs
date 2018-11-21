use super::visitor::*;
use super::*;

#[derive(Debug, Clone)]
pub struct PrettyPrintVisitor {
    current_tab: usize,
}

macro_rules! tab_pr {
    ($s:expr, $fmt:expr) => {
        println!(concat!("{}", $fmt), "  ".repeat($s.current_tab))
    };
    ($s:expr, $fmt:expr, $($arg:tt)*) => {
        println!(concat!("{}", $fmt), "  ".repeat($s.current_tab), $($arg)*)
    };
}

impl PrettyPrintVisitor {
    pub fn new() -> Self {
        PrettyPrintVisitor { current_tab: 0 }
    }
}

macro_rules! tab_block {
    ($s:expr, $block:stmt) => {
        $s.current_tab += 1;
        $block
        $s.current_tab -= 1;
    };
}

impl Visitor for PrettyPrintVisitor {
    type ProgramItem = ();
    type DeclarationItem = ();
    type StatementItem = ();
    type ExpressionItem = ();

    fn visit_program(&mut self, program: &Program, declarations: &[Declaration]) -> Option<()> {
        tab_pr!(self, "program");
        self.current_tab += 1;
        for decl in declarations {
            program.accept_declaration_visitor(self, decl);
        }
        self.current_tab -= 1;
        Some(())
    }

    fn visit_block_statement(
        &mut self,
        program: &Program,
        statements: &[StatementId],
    ) -> Option<()> {
        tab_pr!(self, "block_stmt");

        tab_block!(self, {
            for &s in statements {
                program.accept_statement_visitor(self, s);
            }
        });

        Some(())
    }

    fn visit_function_declaration(
        &mut self,
        program: &Program,
        name: &str,
        parameters: &[(String, Type)],
        statement: StatementId,
    ) -> Option<()> {
        tab_pr!(self, "function_def (name: {})", name);

        tab_block!(self, {
            for &(ref param, ref ty) in parameters {
                tab_pr!(self, "param ({}, {:?})", param, ty);
            }

            program.accept_statement_visitor(self, statement);
        });

        Some(())
    }

    fn visit_let_statement(
        &mut self,
        program: &Program,
        identifier: &str,
        expression: ExpressionId,
    ) -> Option<()> {
        tab_pr!(self, "let_stmt (id: {}) :", identifier);

        tab_block!(self, {
            program.accept_expression_visitor(self, expression);
        });

        Some(())
    }

    fn visit_expression_statement(
        &mut self,
        program: &Program,
        expression: ExpressionId,
    ) -> Option<()> {
        tab_pr!(self, "expr_stmt:");

        tab_block!(self, {
            program.accept_expression_visitor(self, expression);
        });

        Some(())
    }

    fn visit_binop_expression(
        &mut self,
        program: &Program,
        op: BinOp,
        lhs: ExpressionId,
        rhs: ExpressionId,
    ) -> Option<()> {
        tab_pr!(self, "binop_expr (op: {:?})", op);

        tab_block!(self, {
            program.accept_expression_visitor(self, lhs);
            program.accept_expression_visitor(self, rhs);
        });

        Some(())
    }

    fn visit_func_call(
        &mut self,
        program: &Program,
        func: ExpressionId,
        args: &[ExpressionId],
    ) -> Option<()> {
        tab_pr!(self, "func_call");

        tab_block!(self, {
            program.accept_expression_visitor(self, func);
            for e in args {
                program.accept_expression_visitor(self, *e);
            }
        });

        Some(())
    }

    fn visit_integer(&mut self, _program: &Program, i: i32) -> Option<()> {
        tab_pr!(self, "integer: {}", i);

        Some(())
    }

    fn visit_identifier(&mut self, _program: &Program, id: &str) -> Option<()> {
        tab_pr!(self, "identifier: {}", id);

        Some(())
    }
}
