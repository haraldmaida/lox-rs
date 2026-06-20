use super::{
    Assign, Binary, Call, Expr, Get, Grouping, Literal, Logical, Set, Super, This, Unary, Variable,
};
use crate::token::Token;

pub trait ExprExt {
    fn expr(self) -> Expr;
}

impl<T> ExprExt for T
where
    T: Into<Expr>,
{
    fn expr(self) -> Expr {
        self.into()
    }
}

pub fn expr(expr: impl Into<Expr>) -> Expr {
    expr.into()
}

pub fn assign(name: Token, value: impl Into<Expr>) -> Assign {
    Assign::new(name, value.into())
}

pub fn binary(left: impl Into<Expr>, operator: Token, right: impl Into<Expr>) -> Binary {
    Binary::new(left.into(), operator, right.into())
}

pub fn call(
    callee: impl Into<Expr>,
    paren: Token,
    arguments: impl IntoIterator<Item = Expr>,
) -> Call {
    Call::new(callee.into(), paren, arguments.into_iter().collect())
}

pub fn get(object: impl Into<Expr>, name: Token) -> Get {
    Get::new(object.into(), name)
}

pub fn grouping(expression: impl Into<Expr>) -> Grouping {
    Grouping::new(expression.into())
}

pub fn literal(value: impl Into<Literal>) -> Literal {
    value.into()
}

pub const fn nil() -> Literal {
    Literal::Nil
}

pub fn logical(left: impl Into<Expr>, operator: Token, right: impl Into<Expr>) -> Logical {
    Logical::new(left.into(), operator, right.into())
}

pub fn set(object: impl Into<Expr>, name: Token, value: impl Into<Expr>) -> Set {
    Set::new(object.into(), name, value.into())
}

pub const fn super_(keyword: Token, method: Token) -> Super {
    Super::new(keyword, method)
}

pub const fn this_(keyword: Token) -> This {
    This::new(keyword)
}

pub fn unary(operator: Token, right: impl Into<Expr>) -> Unary {
    Unary::new(operator, right.into())
}

pub const fn variable(name: Token) -> Variable {
    Variable::new(name)
}
