use super::{Block, Expression, If, Print, Stmt, Var};
use crate::expr::Expr;
use crate::token::Token;

pub trait StmtExt<'a> {
    fn stmt(self) -> Stmt<'a>;
}

impl<'a, T> StmtExt<'a> for T
where
    T: Into<Stmt<'a>>,
{
    fn stmt(self) -> Stmt<'a> {
        self.into()
    }
}

pub fn stmt<'a>(stmt: impl Into<Stmt<'a>>) -> Stmt<'a> {
    stmt.into()
}

pub fn block<'a>(statements: impl IntoIterator<Item = Stmt<'a>>) -> Block<'a> {
    Block::new(statements.into_iter().collect())
}

pub fn expression<'a>(expr: impl Into<Expression<'a>>) -> Expression<'a> {
    expr.into()
}

pub fn if_<'a>(condition: impl Into<Expr<'a>>, then_branch: impl Into<Stmt<'a>>) -> If<'a> {
    If::new(condition.into(), then_branch.into(), None)
}

pub trait IfExt<'a> {
    #[must_use = "The `else_` method returns a new `If` with the `else_branch` set"]
    fn else_(self, else_branch: impl Into<Stmt<'a>>) -> Self;
}

impl<'a> IfExt<'a> for If<'a> {
    fn else_(mut self, else_branch: impl Into<Stmt<'a>>) -> Self {
        self.else_branch = Some(Box::new(else_branch.into()));
        self
    }
}

pub fn print<'a>(expr: impl Into<Expr<'a>>) -> Print<'a> {
    Print::new(expr.into())
}

pub fn var<'a>(name: Token<'a>, initializer: impl Into<Option<Expr<'a>>>) -> Var<'a> {
    Var::new(name, initializer.into())
}
