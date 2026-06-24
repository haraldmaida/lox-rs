use super::{Block, Class, Expression, Function, If, Print, Return, Stmt, Var, While};
use crate::expr::{Expr, Variable};
use crate::token::Token;

pub trait StmtExt {
    fn stmt(self) -> Stmt;
}

impl<T> StmtExt for T
where
    T: Into<Stmt>,
{
    fn stmt(self) -> Stmt {
        self.into()
    }
}

pub fn stmt(stmt: impl Into<Stmt>) -> Stmt {
    stmt.into()
}

pub fn block(statements: impl IntoIterator<Item = Stmt>) -> Block {
    Block::new(statements.into_iter().collect())
}

pub fn class(
    name: Token,
    superclass: impl Into<Option<Variable>>,
    methods: impl IntoIterator<Item = Function>,
) -> Class {
    Class::new(name, superclass.into(), methods.into_iter().collect())
}

pub fn expression(expr: impl Into<Expression>) -> Expression {
    expr.into()
}

pub fn function(
    name: Token,
    params: impl IntoIterator<Item = Token>,
    body: impl IntoIterator<Item = Stmt>,
) -> Function {
    Function::new(
        name,
        params.into_iter().collect(),
        body.into_iter().collect(),
    )
}

pub fn if_(condition: impl Into<Expr>, then_branch: impl Into<Stmt>) -> If {
    If::new(condition.into(), then_branch.into(), None)
}

pub trait IfExt {
    #[must_use = "The `else_` method returns a new `If` with the `else_branch` set"]
    fn else_(self, else_branch: impl Into<Stmt>) -> Self;
}

impl IfExt for If {
    fn else_(mut self, else_branch: impl Into<Stmt>) -> Self {
        self.else_branch = Some(Box::new(else_branch.into()));
        self
    }
}

pub fn print(expr: impl Into<Expr>) -> Print {
    Print::new(expr.into())
}

pub fn return_(keyword: Token, value: impl Into<Option<Expr>>) -> Return {
    Return::new(keyword, value.into())
}

pub fn var(name: Token, initializer: impl Into<Option<Expr>>) -> Var {
    Var::new(name, initializer.into())
}

pub fn while_(condition: impl Into<Expr>, body: impl Into<Stmt>) -> While {
    While::new(condition.into(), body.into())
}
