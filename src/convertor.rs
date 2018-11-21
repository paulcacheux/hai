use crate::ast;
use crate::ast::visitor::Visitor;
use crate::ir;

use std::collections::HashMap;

pub fn ast2ir(program: &ast::Program) -> ir::Module {
    let mut program_convertor = ProgramConvertor::default();
    let module = program
        .accept_program_visitor(&mut program_convertor)
        .unwrap();

    module
}

#[derive(Debug, Clone, Default)]
struct ProgramConvertor {}

impl Visitor for ProgramConvertor {
    type ProgramItem = ir::Module;
    type DeclarationItem = ir::Function;
    type StatementItem = ();
    type ExpressionItem = ();

    fn visit_program(
        &mut self,
        program: &ast::Program,
        declarations: &[ast::Declaration],
    ) -> Option<ir::Module> {
        let functions = declarations
            .iter()
            .map(|decl| program.accept_declaration_visitor(self, decl).unwrap())
            .collect();

        Some(ir::Module { functions })
    }

    fn visit_function_declaration(
        &mut self,
        program: &ast::Program,
        name: &str,
        parameters: &[(String, ast::Type)],
        statement: ast::StatementId,
    ) -> Option<ir::Function> {
        let mut function_convertor = FunctionConvertor::new(name, parameters);
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
struct FunctionConvertor {
    builder: ir::builder::FunctionBuilder,
}

impl FunctionConvertor {
    pub fn new(name: &str, params: &[(String, ast::Type)]) -> Self {
        let param_tys = params.iter().map(|p| ty_ast2ir(p.1.clone())).collect();
        FunctionConvertor {
            builder: ir::builder::FunctionBuilder::new(name.to_string(), param_tys),
        }
    }
}

fn ty_ast2ir(ty: ast::Type) -> ir::Type {
    match ty {
        ast::Type::Unit => ir::Type::Unit,
        ast::Type::Int => ir::Type::Int,
        ast::Type::Boolean => ir::Type::Boolean,
    }
}
