use super::{Block, Class, Expression, Function, If, Print, Return, Stmt, Var, While};
use crate::expr::{Expr, Variable};
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

pub fn class<'a>(
    name: impl Into<Option<Token<'a>>>,
    superclass: impl Into<Option<Variable<'a>>>,
    methods: impl IntoIterator<Item = Function<'a>>,
) -> Class<'a> {
    Class::new(
        name.into(),
        superclass.into(),
        methods.into_iter().collect(),
    )
}

pub fn expression<'a>(expr: impl Into<Expression<'a>>) -> Expression<'a> {
    expr.into()
}

pub fn function<'a>(
    name: impl Into<Option<Token<'a>>>,
    params: impl IntoIterator<Item = Token<'a>>,
    body: impl IntoIterator<Item = Stmt<'a>>,
) -> Function<'a> {
    Function::new(
        name.into(),
        params.into_iter().collect(),
        body.into_iter().collect(),
    )
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

pub fn return_<'a>(keyword: Token<'a>, value: impl Into<Option<Expr<'a>>>) -> Return<'a> {
    Return::new(keyword, value.into())
}

pub fn var<'a>(name: Token<'a>, initializer: impl Into<Option<Expr<'a>>>) -> Var<'a> {
    Var::new(name, initializer.into())
}

pub fn while_<'a>(condition: impl Into<Expr<'a>>, body: impl Into<Stmt<'a>>) -> While<'a> {
    While::new(condition.into(), body.into())
}
