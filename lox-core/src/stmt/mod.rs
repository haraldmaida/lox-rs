#[cfg(any(test, feature = "dsl"))]
pub use dsl::*;

use crate::expr::{Expr, Variable};
use crate::runtime::RuntimeContext;
use crate::token::Token;
use std::borrow::Borrow;
use std::ops::Deref;

pub trait StmtVisitor {
    type Output;

    fn visit_block_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &Block) -> Self::Output;
    fn visit_class_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &Class) -> Self::Output;
    fn visit_expression_stmt(
        &mut self,
        rtc: &mut RuntimeContext<'_>,
        stmt: &Expression,
    ) -> Self::Output;
    fn visit_function_stmt(
        &mut self,
        rtc: &mut RuntimeContext<'_>,
        stmt: &Function,
    ) -> Self::Output;
    fn visit_if_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &If) -> Self::Output;
    fn visit_print_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &Print) -> Self::Output;
    fn visit_return_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &Return) -> Self::Output;
    fn visit_var_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &Var) -> Self::Output;
    fn visit_while_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &While) -> Self::Output;
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
    Class(Class<'a>),
    Expression(Expression<'a>),
    Function(Function<'a>),
    If(If<'a>),
    Print(Print<'a>),
    Return(Return<'a>),
    Var(Var<'a>),
    While(While<'a>),
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
impl_stmt!(Class<'a>, Class, visit_class_stmt);
impl_stmt!(Expression<'a>, Expression, visit_expression_stmt);
impl_stmt!(Function<'a>, Function, visit_function_stmt);
impl_stmt!(If<'a>, If, visit_if_stmt);
impl_stmt!(Print<'a>, Print, visit_print_stmt);
impl_stmt!(Return<'a>, Return, visit_return_stmt);
impl_stmt!(Var<'a>, Var, visit_var_stmt);
impl_stmt!(While<'a>, While, visit_while_stmt);

impl StmtElement for Stmt<'_> {
    fn accept<V>(&self, visitor: &mut V, rtc: &mut RuntimeContext<'_>) -> <V as StmtVisitor>::Output
    where
        V: StmtVisitor,
    {
        match self {
            Self::Block(stmt) => visitor.visit_block_stmt(rtc, stmt),
            Self::Class(stmt) => visitor.visit_class_stmt(rtc, stmt),
            Self::Expression(stmt) => visitor.visit_expression_stmt(rtc, stmt),
            Self::Function(stmt) => visitor.visit_function_stmt(rtc, stmt),
            Self::If(stmt) => visitor.visit_if_stmt(rtc, stmt),
            Self::Print(stmt) => visitor.visit_print_stmt(rtc, stmt),
            Self::Return(stmt) => visitor.visit_return_stmt(rtc, stmt),
            Self::Var(stmt) => visitor.visit_var_stmt(rtc, stmt),
            Self::While(stmt) => visitor.visit_while_stmt(rtc, stmt),
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

impl<'a> FromIterator<Stmt<'a>> for Block<'a> {
    fn from_iter<T: IntoIterator<Item = Stmt<'a>>>(iter: T) -> Self {
        Block::new(iter.into_iter().collect())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class<'a> {
    name: Option<Token<'a>>,
    superclass: Option<Variable<'a>>,
    methods: Vec<Function<'a>>,
}

impl<'a> Class<'a> {
    pub const fn new(
        name: Option<Token<'a>>,
        superclass: Option<Variable<'a>>,
        methods: Vec<Function<'a>>,
    ) -> Self {
        Self {
            name,
            superclass,
            methods,
        }
    }

    pub const fn name(&self) -> Option<&Token<'a>> {
        self.name.as_ref()
    }

    pub const fn superclass(&self) -> Option<&Variable<'a>> {
        self.superclass.as_ref()
    }

    pub fn methods(&self) -> &[Function<'a>] {
        &self.methods
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
pub struct Function<'a> {
    name: Option<Token<'a>>,
    parameters: Vec<Token<'a>>,
    body: Vec<Stmt<'a>>,
}

impl<'a> Function<'a> {
    pub const fn new(
        name: Option<Token<'a>>,
        parameters: Vec<Token<'a>>,
        body: Vec<Stmt<'a>>,
    ) -> Self {
        Self {
            name,
            parameters,
            body,
        }
    }

    pub const fn name(&self) -> Option<&Token<'a>> {
        self.name.as_ref()
    }

    pub fn parameters(&self) -> &[Token<'a>] {
        &self.parameters
    }

    pub fn body(&self) -> &[Stmt<'a>] {
        &self.body
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct If<'a> {
    condition: Expr<'a>,
    then_branch: Box<Stmt<'a>>,
    else_branch: Option<Box<Stmt<'a>>>,
}

impl<'a> If<'a> {
    pub fn new(condition: Expr<'a>, then_branch: Stmt<'a>, else_branch: Option<Stmt<'a>>) -> Self {
        Self {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        }
    }

    pub const fn condition(&self) -> &Expr<'a> {
        &self.condition
    }

    pub const fn then_branch(&self) -> &Stmt<'a> {
        &self.then_branch
    }

    pub fn else_branch(&self) -> Option<&Stmt<'a>> {
        self.else_branch.as_deref()
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
pub struct Return<'a> {
    keyword: Token<'a>,
    value: Option<Expr<'a>>,
}

impl<'a> Return<'a> {
    pub const fn new(keyword: Token<'a>, value: Option<Expr<'a>>) -> Self {
        Self { keyword, value }
    }

    pub const fn keyword(&self) -> &Token<'a> {
        &self.keyword
    }

    pub const fn value(&self) -> Option<&Expr<'a>> {
        self.value.as_ref()
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

#[derive(Debug, Clone, PartialEq)]
pub struct While<'a> {
    condition: Expr<'a>,
    body: Box<Stmt<'a>>,
}

impl<'a> While<'a> {
    pub fn new(condition: Expr<'a>, body: Stmt<'a>) -> Self {
        Self {
            condition,
            body: Box::new(body),
        }
    }

    pub const fn condition(&self) -> &Expr<'a> {
        &self.condition
    }

    pub fn body(&self) -> &Stmt<'a> {
        &self.body
    }
}

#[cfg(any(test, feature = "dsl"))]
mod dsl;
