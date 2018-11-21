use pest::iterators::Pair;
use pest::prec_climber::*;

use crate::ast;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct HaiParser;

pub fn convert_program<'i>(pair: Pair<'i, Rule>) -> ast::Program {
    assert_eq!(pair.as_rule(), Rule::program);

    let mut program = ast::Program::new();

    for pair in pair.into_inner() {
        let declaration = convert_declaration(pair, &mut program);
        program.declarations.push(declaration);
    }

    program
}

pub fn convert_declaration<'i>(
    pair: Pair<'i, Rule>,
    program: &mut ast::Program,
) -> ast::Declaration {
    assert_eq!(pair.as_rule(), Rule::declaration);

    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::function_declaration => convert_function_declaration(pair, program),
        _ => unreachable!(),
    }
}

pub fn convert_function_declaration<'i>(
    pair: Pair<'i, Rule>,
    program: &mut ast::Program,
) -> ast::Declaration {
    assert_eq!(pair.as_rule(), Rule::function_declaration);

    let mut iter = pair.into_inner();

    let name = String::from(iter.next().unwrap().as_str());
    let parameters = convert_parameter_list(iter.next().unwrap());

    let statement = convert_block_statement(iter.next().unwrap(), program);
    let statement = program.create_statement(statement);

    ast::Declaration::FunctionDeclaration {
        name,
        parameters,
        statement,
    }
}

pub fn convert_parameter_list<'i>(pair: Pair<'i, Rule>) -> Vec<(String, ast::Type)> {
    assert_eq!(pair.as_rule(), Rule::parameter_list);
    pair.into_inner().map(|p| convert_parameter(p)).collect()
}

pub fn convert_parameter<'i>(pair: Pair<'i, Rule>) -> (String, ast::Type) {
    assert_eq!(pair.as_rule(), Rule::parameter);

    let mut iter = pair.into_inner();
    let name = String::from(iter.next().unwrap().as_str());
    let ty = convert_type(iter.next().unwrap());

    (name, ty)
}

pub fn convert_statement<'i>(pair: Pair<'i, Rule>, program: &mut ast::Program) -> ast::Statement {
    assert_eq!(pair.as_rule(), Rule::statement);

    let statement_pair = pair.into_inner().next().unwrap();
    match statement_pair.as_rule() {
        Rule::expression_statement => convert_expression_statement(statement_pair, program),
        Rule::let_statement => convert_let_statement(statement_pair, program),
        Rule::block_statement => convert_block_statement(statement_pair, program),
        _ => unreachable!(),
    }
}

pub fn convert_expression_statement<'i>(
    pair: Pair<'i, Rule>,
    program: &mut ast::Program,
) -> ast::Statement {
    assert_eq!(pair.as_rule(), Rule::expression_statement);
    let pair = pair.into_inner().next().unwrap();
    let expression = convert_expression(pair, program);
    let id = program.create_expression(expression);
    ast::Statement::ExpressionStatement(id)
}

pub fn convert_let_statement<'i>(
    pair: Pair<'i, Rule>,
    program: &mut ast::Program,
) -> ast::Statement {
    assert_eq!(pair.as_rule(), Rule::let_statement);

    let mut inner = pair.into_inner();
    let identifier = String::from(inner.next().unwrap().as_str());
    let expression = convert_expression(inner.next().unwrap(), program);
    let expression = program.create_expression(expression);
    ast::Statement::LetStatement {
        identifier,
        expression,
    }
}

pub fn convert_block_statement<'i>(
    pair: Pair<'i, Rule>,
    program: &mut ast::Program,
) -> ast::Statement {
    assert_eq!(pair.as_rule(), Rule::block_statement);

    let mut stmts = Vec::new();

    for s in pair.into_inner() {
        let s = convert_statement(s, program);
        let s = program.create_statement(s);
        stmts.push(s);
    }

    ast::Statement::BlockStatement(stmts)
}

pub fn convert_expression<'i>(pair: Pair<'i, Rule>, program: &mut ast::Program) -> ast::Expression {
    assert_eq!(pair.as_rule(), Rule::expression);

    let pairs = pair.into_inner();

    let program_mutex = std::sync::Mutex::new(program);

    let climber = PrecClimber::new(vec![
        Operator::new(Rule::plus, Assoc::Left) | Operator::new(Rule::minus, Assoc::Left),
        Operator::new(Rule::star, Assoc::Left) | Operator::new(Rule::slash, Assoc::Left),
    ]);

    let primary = |pair| {
        let program = &mut **program_mutex.lock().unwrap();

        convert_term(pair, program)
    };

    let infix = |lhs, op: Pair<'i, Rule>, rhs| {
        let program = &mut **program_mutex.lock().unwrap();

        let op = match op.as_rule() {
            Rule::star => ast::BinOp::Mul,
            Rule::slash => ast::BinOp::Divide,
            Rule::plus => ast::BinOp::Add,
            Rule::minus => ast::BinOp::Sub,
            _ => unreachable!(),
        };

        let lhs_id = program.create_expression(lhs);
        let rhs_id = program.create_expression(rhs);

        ast::Expression::BinOp {
            op,
            lhs: lhs_id,
            rhs: rhs_id,
        }
    };

    climber.climb(pairs, primary, infix)
}

pub fn convert_term<'i>(pair: Pair<'i, Rule>, program: &mut ast::Program) -> ast::Expression {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::leaf => convert_leaf(inner),
        Rule::paren_expression => convert_paren_expression(inner, program),
        Rule::function_call => convert_function_call(inner, program),
        _ => unreachable!(),
    }
}

pub fn convert_paren_expression<'i>(
    pair: Pair<'i, Rule>,
    program: &mut ast::Program,
) -> ast::Expression {
    assert_eq!(pair.as_rule(), Rule::paren_expression);
    let expression = pair.into_inner().next().unwrap();
    convert_expression(expression, program)
}

pub fn convert_function_call<'i>(
    pair: Pair<'i, Rule>,
    program: &mut ast::Program,
) -> ast::Expression {
    assert_eq!(pair.as_rule(), Rule::function_call);
    let mut iter = pair.into_inner();
    let func = convert_identifier(iter.next().unwrap());
    let args = convert_expression_list(iter.next().unwrap(), program);

    let func = program.create_expression(func);
    let args = args
        .into_iter()
        .map(|e| program.create_expression(e))
        .collect();

    ast::Expression::FunctionCall { func, args }
}

pub fn convert_expression_list<'i>(
    pair: Pair<'i, Rule>,
    program: &mut ast::Program,
) -> Vec<ast::Expression> {
    assert_eq!(pair.as_rule(), Rule::expression_list);
    pair.into_inner()
        .map(|p| convert_expression(p, program))
        .collect()
}

pub fn convert_leaf<'i>(pair: Pair<'i, Rule>) -> ast::Expression {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::integer => convert_integer(inner),
        Rule::identifier => convert_identifier(inner),
        _ => unreachable!(),
    }
}

pub fn convert_identifier<'i>(pair: Pair<'i, Rule>) -> ast::Expression {
    assert_eq!(pair.as_rule(), Rule::identifier);
    ast::Expression::Identifier(String::from(pair.as_str()))
}

pub fn convert_integer<'i>(pair: Pair<'i, Rule>) -> ast::Expression {
    assert_eq!(pair.as_rule(), Rule::integer);
    ast::Expression::Integer(pair.as_str().parse().unwrap())
}

pub fn convert_type<'i>(pair: Pair<'i, Rule>) -> ast::Type {
    assert_eq!(pair.as_rule(), Rule::ty);
    match pair.as_str() {
        "int" => ast::Type::Int,
        "bool" => ast::Type::Boolean,
        _ => match pair.into_inner().next().unwrap().as_rule() {
            Rule::unit_ty => ast::Type::Unit,
            _ => unreachable!(),
        },
    }
}
