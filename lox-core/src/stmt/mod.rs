use crate::expr::Expr;
use crate::token::Token;
use std::borrow::Borrow;
use std::ops::Deref;

pub trait StmtVisitor {
    type Output;

    fn visit_block_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &Block) -> Self::Output;
    fn visit_expression_stmt(
        &mut self,
        rtc: &mut RuntimeContext<'_>,
        stmt: &Expression,
    ) -> Self::Output;
    fn visit_print_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &Print) -> Self::Output;
    fn visit_var_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &Var) -> Self::Output;
}

pub trait StmtElement {
    fn accept<V>(
        &self,
        visitor: &mut V,
        rtc: &mut RuntimeContext<'_>,
    ) -> <V as StmtVisitor>::Output
    where
        V: StmtVisitor;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<'a> {
    Block(Block<'a>),
    Expression(Expression<'a>),
    Print(Print<'a>),
    Var(Var<'a>),
}

macro_rules! impl_stmt {
    ($stmt_type:ty, $variant:ident, $visitor_method:ident) => {
        #[allow(single_use_lifetimes)]
        impl<'a> From<$stmt_type> for Stmt<'a> {
            fn from(stmt: $stmt_type) -> Self {
                Self::$variant(stmt)
            }
        }

        #[allow(single_use_lifetimes, unused_lifetimes)]
        impl<'a> StmtElement for $stmt_type {
            fn accept<V>(
                &self,
                visitor: &mut V,
                rtc: &mut RuntimeContext<'_>,
            ) -> <V as StmtVisitor>::Output
            where
                V: StmtVisitor,
            {
                visitor.$visitor_method(rtc, self)
            }
        }
    };
}

impl_stmt!(Block<'a>, Block, visit_block_stmt);
impl_stmt!(Expression<'a>, Expression, visit_expression_stmt);
impl_stmt!(Print<'a>, Print, visit_print_stmt);
impl_stmt!(Var<'a>, Var, visit_var_stmt);

impl StmtElement for Stmt<'_> {
    fn accept<V>(&self, visitor: &mut V, rtc: &mut RuntimeContext<'_>) -> <V as StmtVisitor>::Output
    where
        V: StmtVisitor,
    {
        match self {
            Self::Block(stmt) => visitor.visit_block_stmt(rtc, stmt),
            Self::Expression(stmt) => visitor.visit_expression_stmt(rtc, stmt),
            Self::Print(stmt) => visitor.visit_print_stmt(rtc, stmt),
            Self::Var(stmt) => visitor.visit_var_stmt(rtc, stmt),
        }
    }
}

impl<'a> From<Expr<'a>> for Stmt<'a> {
    fn from(expr: Expr<'a>) -> Self {
        Self::Expression(Expression::new(expr))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block<'a> {
    statements: Vec<Stmt<'a>>,
}

impl<'a> Block<'a> {
    pub const fn new(statements: Vec<Stmt<'a>>) -> Self {
        Self { statements }
    }

    pub fn statements(&self) -> &[Stmt<'a>] {
        &self.statements
    }
}

/// An expression statement.
#[derive(Debug, Clone, PartialEq)]
pub struct Expression<'a>(Expr<'a>);

impl<'a> Expression<'a> {
    pub const fn new(expr: Expr<'a>) -> Self {
        Self(expr)
    }

    pub const fn expression(&self) -> &Expr<'a> {
        &self.0
    }
}

impl<'a> Deref for Expression<'a> {
    type Target = Expr<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Borrow<Expr<'a>> for Expression<'a> {
    fn borrow(&self) -> &Expr<'a> {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Print<'a> {
    expression: Expr<'a>,
}

impl<'a> Print<'a> {
    pub const fn new(expr: Expr<'a>) -> Self {
        Self { expression: expr }
    }

    pub const fn expression(&self) -> &Expr<'a> {
        &self.expression
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var<'a> {
    name: Token<'a>,
    initializer: Option<Expr<'a>>,
}

impl<'a> Var<'a> {
    pub const fn new(name: Token<'a>, initializer: Option<Expr<'a>>) -> Self {
        Self { name, initializer }
    }

    pub const fn name(&self) -> &Token<'a> {
        &self.name
    }

    pub const fn initializer(&self) -> Option<&Expr<'a>> {
        self.initializer.as_ref()
    }
}

use crate::runtime::RuntimeContext;
#[cfg(any(test, feature = "dsl"))]
pub use dsl::*;

#[cfg(any(test, feature = "dsl"))]
mod dsl;
