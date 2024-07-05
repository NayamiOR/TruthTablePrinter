use prettytable::{Cell, Row, Table};
use crate::assign_parser::AssignParser;
use crate::assign_parser::AnalysisResult;
use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::token::Literal;

pub struct TruthTablePrinter {
    parser: AssignParser,
    interpreter: Interpreter,
    environment: Environment,
}

impl TruthTablePrinter {
    pub fn new() -> Self {
        let parser = AssignParser::new();
        let interpreter = Interpreter::new();

        TruthTablePrinter {
            parser,
            interpreter,
            environment: Environment::new(),
        }
    }
    pub fn print(&mut self, stmt: crate::stmt::Stmt) {
        let AnalysisResult {
            dependent_var,
            mut independent_vars,
            mut environment,
            expr
        } = self.parser.analyze(stmt);
        independent_vars.sort();
        self.interpreter.update_environment(environment.clone());

        let mut table = Table::new();
        let mut first_fow = independent_vars.clone();
        first_fow.push(dependent_var.clone());
        table.add_row(Row::new(first_fow.iter().map(|x| Cell::new(x)).collect()));

        let count = 2u32.pow(independent_vars.len() as u32);
        for t in 0..count {
            let mut new_row = Vec::new();
            for (i, var) in independent_vars.iter().rev().enumerate() {
                let value = (t >> i) & 1;
                self.interpreter.environment.assign(var.clone(), Literal::Boolean(value == 1));
            }
            let value = self.interpreter.evaluate(&expr);
            self.interpreter.environment.assign(dependent_var.clone(), Literal::Boolean(value));
            for var in independent_vars.iter() {
                new_row.push(Cell::new(if self.interpreter.environment.get(var.clone()).evaluate() { "1" } else { "0" }));
            }
            new_row.push(Cell::new(if value { "1" } else { "0" }));
            table.add_row(Row::new(new_row));
        }
        table.printstd();
    }
}