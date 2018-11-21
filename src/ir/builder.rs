use super::*;

#[derive(Debug, Clone)]
pub struct PreBasicBlock {
    statements: Vec<Statement>,
    terminator: Option<Terminator>,
}

#[derive(Debug, Clone)]
pub struct FunctionBuilder {
    name: String,
    params: Vec<Type>,
    variables: Vec<Type>,
    bbs: Vec<PreBasicBlock>,
    current_bb: BB,
    value_counter: u32,
}

impl FunctionBuilder {
    pub fn new(name: String, params: Vec<Type>) -> Self {
        FunctionBuilder {
            name,
            params,
            variables: Vec::new(),
            bbs: vec![PreBasicBlock {
                statements: Vec::new(),
                terminator: None,
            }],
            current_bb: BB(0),
            value_counter: 0,
        }
    }

    pub fn into_function(self) -> Function {
        let basic_blocks = self
            .bbs
            .into_iter()
            .map(|bb| BasicBlock {
                statements: bb.statements,
                terminator: bb.terminator.unwrap(),
            })
            .collect();

        Function {
            name: self.name,
            params: self.params,
            variables: self.variables,
            basic_blocks,
        }
    }

    pub fn create_variable(&mut self, ty: Type) -> Variable {
        let index = self.variables.len();
        self.variables.push(ty);
        Variable(index as u32)
    }

    pub fn create_variable_param(&mut self, index: usize) -> Variable {
        self.create_variable(self.params[index])
    }

    pub fn create_new_bb(&mut self) -> BB {
        let index = self.bbs.len();
        self.bbs.push(PreBasicBlock {
            statements: Vec::new(),
            terminator: None,
        });
        BB(index)
    }

    pub fn switch_to_bb(&mut self, bb: BB) {
        self.current_bb = bb;
    }

    pub fn terminate_bb(&mut self, terminator: Terminator) {
        self.bbs[self.current_bb.0].terminator = Some(terminator);
    }

    pub fn append_value_operation(&mut self, operation: Operation) -> Value {
        let value = Value(self.value_counter);
        self.value_counter += 1;
        self.bbs[self.current_bb.0]
            .statements
            .push(Statement::WithDestination(value, operation));
        value
    }

    pub fn append_no_value_operation(&mut self, operation: Operation) {
        self.bbs[self.current_bb.0]
            .statements
            .push(Statement::Operation(operation));
    }
}
