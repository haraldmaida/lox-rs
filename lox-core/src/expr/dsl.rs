use super::{
    Assign, Binary, Call, Expr, Get, Grouping, Literal, Logical, Set, Super, This, Unary, Variable,
};
use crate::token::Token;

pub trait ExprExt<'a> {
    fn expr(self) -> Expr<'a>;
}

impl<'a, T> ExprExt<'a> for T
where
    T: Into<Expr<'a>>,
{
    fn expr(self) -> Expr<'a> {
        self.into()
    }
}

pub fn expr<'a>(expr: impl Into<Expr<'a>>) -> Expr<'a> {
    expr.into()
}

pub fn assign<'a>(name: Token<'a>, value: impl Into<Expr<'a>>) -> Assign<'a> {
    Assign::new(name, value.into())
}

pub fn binary<'a>(
    left: impl Into<Expr<'a>>,
    operator: Token<'a>,
    right: impl Into<Expr<'a>>,
) -> Binary<'a> {
    Binary::new(left.into(), operator, right.into())
}

pub fn call<'a>(
    callee: impl Into<Expr<'a>>,
    paren: Token<'a>,
    arguments: Vec<Expr<'a>>,
) -> Call<'a> {
    Call::new(callee.into(), paren, arguments)
}

pub fn get<'a>(object: impl Into<Expr<'a>>, name: Token<'a>) -> Get<'a> {
    Get::new(object.into(), name)
}

pub fn grouping<'a>(expression: impl Into<Expr<'a>>) -> Grouping<'a> {
    Grouping::new(expression.into())
}

pub fn literal(value: impl Into<Literal>) -> Literal {
    value.into()
}

pub const fn nil() -> Literal {
    Literal::Nil
}

pub fn logical<'a>(
    left: impl Into<Expr<'a>>,
    operator: Token<'a>,
    right: impl Into<Expr<'a>>,
) -> Logical<'a> {
    Logical::new(left.into(), operator, right.into())
}

pub fn set<'a>(
    object: impl Into<Expr<'a>>,
    name: Token<'a>,
    value: impl Into<Expr<'a>>,
) -> Set<'a> {
    Set::new(object.into(), name, value.into())
}

pub const fn super_<'a>(keyword: Token<'a>, method: Token<'a>) -> Super<'a> {
    Super::new(keyword, method)
}

pub const fn this_(keyword: Token<'_>) -> This<'_> {
    This::new(keyword)
}

pub fn unary<'a>(operator: Token<'a>, right: impl Into<Expr<'a>>) -> Unary<'a> {
    Unary::new(operator, right.into())
}

pub const fn variable(name: Token<'_>) -> Variable<'_> {
    Variable::new(name)
}
