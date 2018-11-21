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
    sym_table: SymbolTable,
    builder: ir::builder::FunctionBuilder,
}

impl FunctionConvertor {
    pub fn new(name: &str, params: &[(String, ast::Type)]) -> Self {
        let mut param_tys = Vec::new();
        for &(_, ref ty) in params {
            param_tys.push(ty_ast2ir(ty.clone()));
        }

        let mut sym_table = SymbolTable::new();
        sym_table.begin_scope(); // TODO
        let mut builder = ir::builder::FunctionBuilder::new(name.to_string(), param_tys.clone());

        for (index, &(ref param, _)) in params.iter().enumerate() {
            let var = builder.create_variable_param(index);
            sym_table.register_variable(param.to_string(), param_tys[index].clone(), var);
        }

        FunctionConvertor {
            sym_table: SymbolTable::new(),
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

#[derive(Debug, Clone)]
struct SymbolTable {
    scopes: Vec<HashMap<String, TypedVariable>>,
}

#[derive(Debug, Clone)]
struct TypedVariable {
    ty: ir::Type,
    var: ir::Variable,
}

impl SymbolTable {
    fn new() -> Self {
        SymbolTable { scopes: Vec::new() }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn register_variable(&mut self, name: String, ty: ir::Type, var: ir::Variable) {
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name, TypedVariable { ty, var });
    }

    fn lookup_variable(&mut self, name: &str) -> Option<TypedVariable> {
        for scope in self.scopes.iter().rev() {
            if let Some(tv) = scope.get(name) {
                return Some(tv.clone());
            }
        }
        None
    }
}
