#[cfg(any(test, feature = "dsl"))]
pub use dsl::*;

use crate::data::Symbol;
use crate::token::Token;

pub trait ExprVisitor {
    type Context<'c>;
    type Output;

    fn visit_assign_expr(&mut self, ctx: &mut Self::Context<'_>, expr: &Assign) -> Self::Output;
    fn visit_binary_expr(&mut self, ctx: &mut Self::Context<'_>, expr: &Binary) -> Self::Output;
    fn visit_call_expr(&mut self, ctx: &mut Self::Context<'_>, expr: &Call) -> Self::Output;
    fn visit_get_expr(&mut self, ctx: &mut Self::Context<'_>, expr: &Get) -> Self::Output;
    fn visit_grouping_expr(&mut self, ctx: &mut Self::Context<'_>, expr: &Grouping)
    -> Self::Output;
    fn visit_literal_expr(&mut self, ctx: &mut Self::Context<'_>, expr: &Literal) -> Self::Output;
    fn visit_logical_expr(&mut self, ctx: &mut Self::Context<'_>, expr: &Logical) -> Self::Output;
    fn visit_set_expr(&mut self, ctx: &mut Self::Context<'_>, expr: &Set) -> Self::Output;
    fn visit_super_expr(&mut self, ctx: &mut Self::Context<'_>, expr: &Super) -> Self::Output;
    fn visit_this_expr(&mut self, ctx: &mut Self::Context<'_>, expr: &This) -> Self::Output;
    fn visit_unary_expr(&mut self, ctx: &mut Self::Context<'_>, expr: &Unary) -> Self::Output;
    fn visit_variable_expr(&mut self, ctx: &mut Self::Context<'_>, expr: &Variable)
    -> Self::Output;
}

pub trait ExprElement {
    fn accept<V>(
        &self,
        visitor: &mut V,
        ctx: &mut <V as ExprVisitor>::Context<'_>,
    ) -> <V as ExprVisitor>::Output
    where
        V: ExprVisitor;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Assign(Assign),
    Binary(Binary),
    Call(Call),
    Get(Get),
    Grouping(Grouping),
    Literal(Literal),
    Logical(Logical),
    Set(Set),
    Super(Super),
    This(This),
    Unary(Unary),
    Variable(Variable),
}

macro_rules! impl_expr {
    ($expr_type:ty, $variant:ident, $visitor_method:ident) => {
        #[allow(single_use_lifetimes)]
        impl From<$expr_type> for Expr {
            fn from(expr: $expr_type) -> Self {
                Self::$variant(expr)
            }
        }

        #[allow(single_use_lifetimes, unused_lifetimes)]
        impl ExprElement for $expr_type {
            fn accept<V>(
                &self,
                visitor: &mut V,
                ctx: &mut <V as ExprVisitor>::Context<'_>,
            ) -> <V as ExprVisitor>::Output
            where
                V: ExprVisitor,
            {
                visitor.$visitor_method(ctx, self)
            }
        }
    };
}

impl_expr!(Assign, Assign, visit_assign_expr);
impl_expr!(Binary, Binary, visit_binary_expr);
impl_expr!(Call, Call, visit_call_expr);
impl_expr!(Get, Get, visit_get_expr);
impl_expr!(Grouping, Grouping, visit_grouping_expr);
impl_expr!(Literal, Literal, visit_literal_expr);
impl_expr!(Logical, Logical, visit_logical_expr);
impl_expr!(Set, Set, visit_set_expr);
impl_expr!(Super, Super, visit_super_expr);
impl_expr!(This, This, visit_this_expr);
impl_expr!(Unary, Unary, visit_unary_expr);
impl_expr!(Variable, Variable, visit_variable_expr);

impl ExprElement for Expr {
    fn accept<V>(
        &self,
        visitor: &mut V,
        ctx: &mut <V as ExprVisitor>::Context<'_>,
    ) -> <V as ExprVisitor>::Output
    where
        V: ExprVisitor,
    {
        match self {
            Self::Assign(expr) => visitor.visit_assign_expr(ctx, expr),
            Self::Binary(expr) => visitor.visit_binary_expr(ctx, expr),
            Self::Call(expr) => visitor.visit_call_expr(ctx, expr),
            Self::Get(expr) => visitor.visit_get_expr(ctx, expr),
            Self::Grouping(expr) => visitor.visit_grouping_expr(ctx, expr),
            Self::Literal(expr) => visitor.visit_literal_expr(ctx, expr),
            Self::Logical(expr) => visitor.visit_logical_expr(ctx, expr),
            Self::Set(expr) => visitor.visit_set_expr(ctx, expr),
            Self::Super(expr) => visitor.visit_super_expr(ctx, expr),
            Self::This(expr) => visitor.visit_this_expr(ctx, expr),
            Self::Unary(expr) => visitor.visit_unary_expr(ctx, expr),
            Self::Variable(expr) => visitor.visit_variable_expr(ctx, expr),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assign {
    name: Token,
    value: Box<Expr>,
}

impl Assign {
    pub fn new(name: Token, value: Expr) -> Self {
        Self {
            name,
            value: Box::new(value),
        }
    }

    pub const fn name(&self) -> Token {
        self.name
    }

    pub fn value(&self) -> &Expr {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub const fn left(&self) -> &Expr {
        &self.left
    }

    pub const fn operator(&self) -> Token {
        self.operator
    }

    pub const fn right(&self) -> &Expr {
        &self.right
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    callee: Box<Expr>,
    paren: Token,
    arguments: Vec<Expr>,
}

impl Call {
    pub fn new(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Self {
        Self {
            callee: Box::new(callee),
            paren,
            arguments,
        }
    }

    pub const fn callee(&self) -> &Expr {
        &self.callee
    }

    pub const fn paren(&self) -> Token {
        self.paren
    }

    pub fn arguments(&self) -> &[Expr] {
        &self.arguments
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Get {
    object: Box<Expr>,
    name: Token,
}

impl Get {
    pub fn new(object: Expr, name: Token) -> Self {
        Self {
            object: Box::new(object),
            name,
        }
    }

    pub const fn object(&self) -> &Expr {
        &self.object
    }

    pub const fn name(&self) -> Token {
        self.name
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grouping {
    expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Expr) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }

    pub const fn expression(&self) -> &Expr {
        &self.expression
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal {
    Nil,
    Bool(bool),
    Number(f64),
    String(Symbol),
}

impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<f32> for Literal {
    fn from(value: f32) -> Self {
        Self::Number(value.into())
    }
}

impl From<i32> for Literal {
    fn from(value: i32) -> Self {
        Self::Number(value.into())
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Self::String(value.into())
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Logical {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl Logical {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub const fn left(&self) -> &Expr {
        &self.left
    }

    pub const fn operator(&self) -> Token {
        self.operator
    }

    pub const fn right(&self) -> &Expr {
        &self.right
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Set {
    object: Box<Expr>,
    name: Token,
    value: Box<Expr>,
}

impl Set {
    pub fn new(object: Expr, name: Token, value: Expr) -> Self {
        Self {
            object: Box::new(object),
            name,
            value: Box::new(value),
        }
    }

    pub const fn object(&self) -> &Expr {
        &self.object
    }

    pub const fn name(&self) -> Token {
        self.name
    }

    pub const fn value(&self) -> &Expr {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Super {
    keyword: Token,
    method: Token,
}

impl Super {
    pub const fn new(keyword: Token, method: Token) -> Self {
        Self { keyword, method }
    }

    pub const fn keyword(&self) -> Token {
        self.keyword
    }

    pub const fn method(&self) -> Token {
        self.method
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct This {
    keyword: Token,
}

impl This {
    pub const fn new(keyword: Token) -> Self {
        Self { keyword }
    }

    pub const fn keyword(&self) -> Token {
        self.keyword
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unary {
    operator: Token,
    right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }

    pub const fn operator(&self) -> Token {
        self.operator
    }

    pub const fn right(&self) -> &Expr {
        &self.right
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Variable {
    name: Token,
}

impl Variable {
    pub const fn new(name: Token) -> Self {
        Self { name }
    }

    pub const fn name(&self) -> Token {
        self.name
    }

    pub const fn take_name(self) -> Token {
        self.name
    }
}

#[cfg(any(test, feature = "dsl"))]
mod dsl;
