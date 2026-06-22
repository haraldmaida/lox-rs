#[cfg(any(test, feature = "dsl"))]
pub use dsl::*;

use crate::expr::{Expr, Variable};
use crate::token::Token;
use std::borrow::Borrow;
use std::ops::Deref;

pub trait StmtVisitor {
    type Context<'c>;
    type Output;

    fn visit_block_stmt(&mut self, ctx: &mut Self::Context<'_>, stmt: &Block) -> Self::Output;
    fn visit_class_stmt(&mut self, ctx: &mut Self::Context<'_>, stmt: &Class) -> Self::Output;
    fn visit_expression_stmt(
        &mut self,
        ctx: &mut Self::Context<'_>,
        stmt: &Expression,
    ) -> Self::Output;
    fn visit_function_stmt(&mut self, ctx: &mut Self::Context<'_>, stmt: &Function)
    -> Self::Output;
    fn visit_if_stmt(&mut self, ctx: &mut Self::Context<'_>, stmt: &If) -> Self::Output;
    fn visit_print_stmt(&mut self, ctx: &mut Self::Context<'_>, stmt: &Print) -> Self::Output;
    fn visit_return_stmt(&mut self, ctx: &mut Self::Context<'_>, stmt: &Return) -> Self::Output;
    fn visit_var_stmt(&mut self, ctx: &mut Self::Context<'_>, stmt: &Var) -> Self::Output;
    fn visit_while_stmt(&mut self, ctx: &mut Self::Context<'_>, stmt: &While) -> Self::Output;
}

pub trait StmtElement {
    fn accept<V>(
        &self,
        visitor: &mut V,
        ctx: &mut <V as StmtVisitor>::Context<'_>,
    ) -> <V as StmtVisitor>::Output
    where
        V: StmtVisitor;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Block(Block),
    Class(Class),
    Expression(Expression),
    Function(Function),
    If(If),
    Print(Print),
    Return(Return),
    Var(Var),
    While(While),
}

macro_rules! impl_stmt {
    ($stmt_type:ty, $variant:ident, $visitor_method:ident) => {
        #[allow(single_use_lifetimes)]
        impl From<$stmt_type> for Stmt {
            fn from(stmt: $stmt_type) -> Self {
                Self::$variant(stmt)
            }
        }

        #[allow(single_use_lifetimes, unused_lifetimes)]
        impl StmtElement for $stmt_type {
            fn accept<V>(
                &self,
                visitor: &mut V,
                ctx: &mut <V as StmtVisitor>::Context<'_>,
            ) -> <V as StmtVisitor>::Output
            where
                V: StmtVisitor,
            {
                visitor.$visitor_method(ctx, self)
            }
        }
    };
}

impl_stmt!(Block, Block, visit_block_stmt);
impl_stmt!(Class, Class, visit_class_stmt);
impl_stmt!(Expression, Expression, visit_expression_stmt);
impl_stmt!(Function, Function, visit_function_stmt);
impl_stmt!(If, If, visit_if_stmt);
impl_stmt!(Print, Print, visit_print_stmt);
impl_stmt!(Return, Return, visit_return_stmt);
impl_stmt!(Var, Var, visit_var_stmt);
impl_stmt!(While, While, visit_while_stmt);

impl StmtElement for Stmt {
    fn accept<V>(
        &self,
        visitor: &mut V,
        ctx: &mut <V as StmtVisitor>::Context<'_>,
    ) -> <V as StmtVisitor>::Output
    where
        V: StmtVisitor,
    {
        match self {
            Self::Block(stmt) => visitor.visit_block_stmt(ctx, stmt),
            Self::Class(stmt) => visitor.visit_class_stmt(ctx, stmt),
            Self::Expression(stmt) => visitor.visit_expression_stmt(ctx, stmt),
            Self::Function(stmt) => visitor.visit_function_stmt(ctx, stmt),
            Self::If(stmt) => visitor.visit_if_stmt(ctx, stmt),
            Self::Print(stmt) => visitor.visit_print_stmt(ctx, stmt),
            Self::Return(stmt) => visitor.visit_return_stmt(ctx, stmt),
            Self::Var(stmt) => visitor.visit_var_stmt(ctx, stmt),
            Self::While(stmt) => visitor.visit_while_stmt(ctx, stmt),
        }
    }
}

impl From<Expr> for Stmt {
    fn from(expr: Expr) -> Self {
        Self::Expression(Expression::new(expr))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    statements: Vec<Stmt>,
}

impl Block {
    pub const fn new(statements: Vec<Stmt>) -> Self {
        Self { statements }
    }

    pub fn statements(&self) -> &[Stmt] {
        &self.statements
    }
}

impl FromIterator<Stmt> for Block {
    fn from_iter<T: IntoIterator<Item = Stmt>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    name: Option<Token>,
    superclass: Option<Variable>,
    methods: Vec<Function>,
}

impl Class {
    pub const fn new(
        name: Option<Token>,
        superclass: Option<Variable>,
        methods: Vec<Function>,
    ) -> Self {
        Self {
            name,
            superclass,
            methods,
        }
    }

    pub const fn name(&self) -> Option<Token> {
        self.name
    }

    pub const fn superclass(&self) -> Option<&Variable> {
        self.superclass.as_ref()
    }

    pub fn methods(&self) -> &[Function] {
        &self.methods
    }
}

/// An expression statement.
#[derive(Debug, Clone, PartialEq)]
pub struct Expression(Expr);

impl Expression {
    pub const fn new(expr: Expr) -> Self {
        Self(expr)
    }

    pub const fn expression(&self) -> &Expr {
        &self.0
    }
}

impl Deref for Expression {
    type Target = Expr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Borrow<Expr> for Expression {
    fn borrow(&self) -> &Expr {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    name: Token,
    parameters: Vec<Token>,
    body: Vec<Stmt>,
}

impl Function {
    pub const fn new(name: Token, parameters: Vec<Token>, body: Vec<Stmt>) -> Self {
        Self {
            name,
            parameters,
            body,
        }
    }

    pub const fn name(&self) -> Token {
        self.name
    }

    pub fn parameters(&self) -> &[Token] {
        &self.parameters
    }

    pub fn body(&self) -> &[Stmt] {
        &self.body
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct If {
    condition: Expr,
    then_branch: Box<Stmt>,
    else_branch: Option<Box<Stmt>>,
}

impl If {
    pub fn new(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Self {
        Self {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        }
    }

    pub const fn condition(&self) -> &Expr {
        &self.condition
    }

    pub const fn then_branch(&self) -> &Stmt {
        &self.then_branch
    }

    pub fn else_branch(&self) -> Option<&Stmt> {
        self.else_branch.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Print {
    expression: Expr,
}

impl Print {
    pub const fn new(expr: Expr) -> Self {
        Self { expression: expr }
    }

    pub const fn expression(&self) -> &Expr {
        &self.expression
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    keyword: Token,
    value: Option<Expr>,
}

impl Return {
    pub const fn new(keyword: Token, value: Option<Expr>) -> Self {
        Self { keyword, value }
    }

    pub const fn keyword(&self) -> Token {
        self.keyword
    }

    pub const fn value(&self) -> Option<&Expr> {
        self.value.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    name: Token,
    initializer: Option<Expr>,
}

impl Var {
    pub const fn new(name: Token, initializer: Option<Expr>) -> Self {
        Self { name, initializer }
    }

    pub const fn name(&self) -> Token {
        self.name
    }

    pub const fn initializer(&self) -> Option<&Expr> {
        self.initializer.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct While {
    condition: Expr,
    body: Box<Stmt>,
}

impl While {
    pub fn new(condition: Expr, body: Stmt) -> Self {
        Self {
            condition,
            body: Box::new(body),
        }
    }

    pub const fn condition(&self) -> &Expr {
        &self.condition
    }

    pub fn body(&self) -> &Stmt {
        &self.body
    }
}

#[cfg(any(test, feature = "dsl"))]
mod dsl;
