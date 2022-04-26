mod comments;
mod operations;
mod types;

use comments::strip_comments;
use types::Type;

use self::operations::Operations;

/// The main interpreter struct which contains all of the interpreter data
pub struct Interpreter {}

impl Interpreter {
    pub fn run(source: String) -> Result<(), ()> {
        let mut s = Self {};

        s.eval(source)
    }

    fn eval(&mut self, source: String) -> Result<(), ()> {
        for (line_number, line) in source.lines().enumerate() {
            self.eval_line(line, line_number)?;
        }
        Ok(())
    }

    fn eval_line(&mut self, line: &str, line_number: usize) -> Result<(), ()> {
        let line = strip_comments(line);
        Ok(())
    }

    fn eval_operations(&mut self, operations: Vec<Operations>) -> Result<(), ()> {
        Ok(())
    }
}
