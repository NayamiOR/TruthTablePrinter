use crate::boolean::Boolean;

mod lexer;
mod parser;
mod token_type;
mod token;
mod expr;
mod boolean;
mod interpreter;
mod stmt;
mod environment;
mod assign_parser;
mod truth_table_printer;

fn main() {
    Boolean::run();
}

