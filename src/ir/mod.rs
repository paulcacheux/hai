pub mod builder;

#[derive(Debug, Clone)]
pub struct Module {
    functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Function {
    params: Vec<Type>,
    variables: Vec<Type>,
    basic_blocks: Vec<BasicBlock>,
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    statements: Vec<Statement>,
    terminator: Terminator,
}

#[derive(Debug, Clone)]
pub enum Statement {
    WithDestination(Value, Operation),
    Operation(Operation)
}

#[derive(Debug, Clone)]
pub enum Operation {
    BinaryAdd(Value, Value),
    BinarySub(Value, Value),
    BinaryMul(Value, Value),
    BinaryDivide(Value, Value),
    LoadParam(usize),
    LoadVariable(Variable),
    StoreVariable(Variable, Value),
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Return(Value)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BB(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Variable(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(u32);

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Unit,
    Int,
    Boolean,
}