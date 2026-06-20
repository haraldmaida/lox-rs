#[cfg(any(test, feature = "dsl"))]
pub use dsl::*;

use crate::stmt::Stmt;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Program(Vec<Stmt>);

impl Program {
    pub fn new(statements: impl IntoIterator<Item = Stmt>) -> Self {
        Self(Vec::from_iter(statements))
    }

    pub fn statements(&self) -> &[Stmt] {
        &self.0
    }

    pub fn add_stmt(&mut self, stmt: impl Into<Stmt>) {
        self.0.push(stmt.into());
    }
}

impl FromIterator<Stmt> for Program {
    fn from_iter<T: IntoIterator<Item = Stmt>>(statements: T) -> Self {
        Self(Vec::from_iter(statements))
    }
}

impl AsRef<[Stmt]> for Program {
    fn as_ref(&self) -> &[Stmt] {
        &self.0
    }
}

#[cfg(any(test, feature = "dsl"))]
mod dsl;
