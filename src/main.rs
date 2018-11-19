extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate id_arena;

use pest::Parser;

mod parser;
mod ast;

static INPUT: &str = "
12 * 13 * 15;
let a = 12;
let b = 45 * b;
";

fn main() {
    let mut pairs = parser::HaiParser::parse(parser::Rule::program, INPUT).unwrap_or_else(|e| panic!("{}", e));
    let program_pair = pairs.next().unwrap();
    let program = parser::convert_program(program_pair);

    println!("{:#?}", program);
}
