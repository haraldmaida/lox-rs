use crate::stmt::Stmt;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Program<'a>(Vec<Stmt<'a>>);

impl<'a> Program<'a> {
    pub fn new(statements: impl IntoIterator<Item = Stmt<'a>>) -> Self {
        Self(Vec::from_iter(statements))
    }

    pub fn statements(&self) -> &[Stmt<'a>] {
        &self.0
    }

    pub fn add_stmt(&mut self, stmt: impl Into<Stmt<'a>>) {
        self.0.push(stmt.into());
    }
}

impl<'a> FromIterator<Stmt<'a>> for Program<'a> {
    fn from_iter<T: IntoIterator<Item = Stmt<'a>>>(statements: T) -> Self {
        Self(Vec::from_iter(statements))
    }
}
