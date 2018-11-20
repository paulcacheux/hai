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
        let statement = convert_statement(pair, &mut program);
        program.create_append_statement(statement);
    }

    program
}

pub fn convert_statement<'i>(pair: Pair<'i, Rule>, program: &mut ast::Program) -> ast::Statement {
    let statement_pair = pair.into_inner().next().unwrap();
    match statement_pair.as_rule() {
        Rule::expression_statement => {
            let pair = statement_pair.into_inner().next().unwrap();
            let expression = convert_expression(pair, program);
            let id = program.create_expression(expression);
            ast::Statement::ExpressionStatement(id)
        },
        Rule::let_statement => {
            let mut inner = statement_pair.into_inner();
            let identifier = String::from(inner.next().unwrap().as_str());
            let expression = convert_expression(inner.next().unwrap(), program);
            let expression = program.create_expression(expression);
            ast::Statement::LetStatement {
                identifier,
                expression,
            }
        },
        _ => unreachable!()
    }
}

pub fn convert_expression<'i>(pair: Pair<'i, Rule>, program: &mut ast::Program) -> ast::Expression {
    assert_eq!(pair.as_rule(), Rule::expression);

    let pairs = pair.into_inner();

    let climber = PrecClimber::new(vec![
        Operator::new(Rule::plus, Assoc::Left) | Operator::new(Rule::minus, Assoc::Left),
        Operator::new(Rule::star, Assoc::Left) | Operator::new(Rule::slash, Assoc::Left)
    ]);

    let primary = |pair| {
        convert_term(pair)
    };

    let infix = |lhs, op: Pair<'i, Rule>, rhs| {
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

pub fn convert_term<'i>(pair: Pair<'i, Rule>) -> ast::Expression {
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