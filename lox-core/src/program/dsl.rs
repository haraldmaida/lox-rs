use crate::program::Program;
use crate::stmt::Stmt;

pub fn program(statements: impl IntoIterator<Item = Stmt>) -> Program {
    Program::from_iter(statements)
}
