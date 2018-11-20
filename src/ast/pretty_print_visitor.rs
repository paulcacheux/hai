use super::*;
use super::visitor::*;

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
        PrettyPrintVisitor {
            current_tab: 0,
        }
    }
}

impl Visitor for PrettyPrintVisitor {

    fn visit_program(&mut self, program: &Program, statements: &[StatementId]) {
        tab_pr!(self, "program");
        self.current_tab += 1;
        for &s in statements {
            program.accept_statement_visitor(self, s);
        }
        self.current_tab -= 1;
    }

    fn visit_let_statement(&mut self, program: &Program, identifier: &str, expression: ExpressionId) {
        tab_pr!(self, "let_stmt (id: {}) :", identifier);
        self.current_tab += 1;
        program.accept_expression_visitor(self, expression);
        self.current_tab -= 1;
    }
    
    fn visit_expression_statement(&mut self, program: &Program, expression: ExpressionId) {
        tab_pr!(self, "expr_stmt:");
        self.current_tab += 1;
        program.accept_expression_visitor(self, expression);
        self.current_tab -= 1;
    }

    fn visit_binop_expression(&mut self, program: &Program, op: BinOp, lhs: ExpressionId, rhs: ExpressionId) {
        tab_pr!(self, "binop_expr (op: {:?})", op);
        self.current_tab += 1;
        program.accept_expression_visitor(self, lhs);
        program.accept_expression_visitor(self, rhs);
        self.current_tab -= 1;
    }

    fn visit_integer(&mut self, _program: &Program, i: i32) {
        tab_pr!(self, "integer: {}", i);
    }

    fn visit_identifier(&mut self, _program: &Program, id: &str) {
        tab_pr!(self, "identifier: {}", id);
    }
}