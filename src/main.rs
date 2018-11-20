extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate id_arena;

use pest::Parser;

mod parser;
mod ast;

fn main() {
    let input_path = std::env::args().nth(1).expect("No input path");
    let input = std::fs::read_to_string(input_path).expect("Can't read input");
    let mut pairs = parser::HaiParser::parse(parser::Rule::program, &input).unwrap_or_else(|e| panic!("{}", e));
    let program_pair = pairs.next().unwrap();
    let program = parser::convert_program(program_pair);

    let mut visitor = ast::pretty_print_visitor::PrettyPrintVisitor::new();

    program.accept_program_visitor(&mut visitor);
}
