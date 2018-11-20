use super::*;
use super::visitor::*;

#[derive(Debug, Clone)]
pub struct PrettyPrintVisitor {
    current_tab: usize,
}

macro_rules! tab_pr {
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
    fn visit_let_statement(&mut self, program: &Program, identifier: &str, expression: ExpressionId) {
        tab_pr!(self, "LETSTATEMENT (id: {}) :", identifier);
        self.current_tab += 1;
        program.accept_expression_visitor(self, expression);
        self.current_tab -= 1;
    }
}