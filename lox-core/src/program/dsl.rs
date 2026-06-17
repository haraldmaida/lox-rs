use crate::program::Program;
use crate::stmt::Stmt;

pub fn program<'a>(statements: impl IntoIterator<Item = Stmt<'a>>) -> Program<'a> {
    Program::from_iter(statements)
}
