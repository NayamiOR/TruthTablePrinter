use std::io;
use std::io::Write;

use crate::lexer;
use crate::parser;
use crate::truth_table_printer::TruthTablePrinter;

pub struct Boolean;

impl Boolean {
    pub fn run() {
        loop {
            print!("$ ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim().is_empty() {
                Boolean::sim_report("Error: No input provided.");
                continue
            }

            let mut lexer = lexer::Lexer::new(input);
            let tokens = lexer.scan_tokens().clone();

            let mut parser = parser::Parser::new(tokens);
            let statements = parser.parse();
            if let Err(e) = statements {
                Boolean::sim_error(e);
                continue;
            }
            let statements = statements.unwrap();
            let statement = &statements[0];
            let mut printer = TruthTablePrinter::new();
            printer.print(statement.clone());
        }
    }

    pub fn sim_error(message: &str) {
        Self::sim_report(message);
    }

    pub fn sim_report(message: &str) {
        eprintln!("Error: {}", message);
    }
}