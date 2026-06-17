use crate::data::Symbol;
use crate::token::Token;

pub trait ExprVisitor {
    type Output;

    fn visit_assign_expr(&mut self, expr: &Assign) -> Self::Output;
    fn visit_binary_expr(&mut self, expr: &Binary) -> Self::Output;
    fn visit_call_expr(&mut self, expr: &Call) -> Self::Output;
    fn visit_get_expr(&mut self, expr: &Get) -> Self::Output;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Self::Output;
    fn visit_literal_expr(&mut self, expr: &Literal) -> Self::Output;
    fn visit_logical_expr(&mut self, expr: &Logical) -> Self::Output;
    fn visit_set_expr(&mut self, expr: &Set) -> Self::Output;
    fn visit_super_expr(&mut self, expr: &Super) -> Self::Output;
    fn visit_this_expr(&mut self, expr: &This) -> Self::Output;
    fn visit_unary_expr(&mut self, expr: &Unary) -> Self::Output;
    fn visit_variable_expr(&mut self, expr: &Variable) -> Self::Output;
}

pub trait ExprElement {
    fn accept<V>(&self, visitor: &mut V) -> <V as ExprVisitor>::Output
    where
        V: ExprVisitor;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    Assign(Assign<'a>),
    Binary(Binary<'a>),
    Call(Call<'a>),
    Get(Get<'a>),
    Grouping(Grouping<'a>),
    Literal(Literal),
    Logical(Logical<'a>),
    Set(Set<'a>),
    Super(Super<'a>),
    This(This<'a>),
    Unary(Unary<'a>),
    Variable(Variable<'a>),
}

macro_rules! impl_expr {
    ($expr_type:ty, $variant:ident, $visitor_method:ident) => {
        #[allow(single_use_lifetimes)]
        impl<'a> From<$expr_type> for Expr<'a> {
            fn from(expr: $expr_type) -> Self {
                Self::$variant(expr)
            }
        }

        #[allow(single_use_lifetimes, unused_lifetimes)]
        impl<'a> ExprElement for $expr_type {
            fn accept<V>(&self, visitor: &mut V) -> <V as ExprVisitor>::Output
            where
                V: ExprVisitor,
            {
                visitor.$visitor_method(self)
            }
        }
    };
}

impl_expr!(Assign<'a>, Assign, visit_assign_expr);
impl_expr!(Binary<'a>, Binary, visit_binary_expr);
impl_expr!(Call<'a>, Call, visit_call_expr);
impl_expr!(Get<'a>, Get, visit_get_expr);
impl_expr!(Grouping<'a>, Grouping, visit_grouping_expr);
impl_expr!(Literal, Literal, visit_literal_expr);
impl_expr!(Logical<'a>, Logical, visit_logical_expr);
impl_expr!(Set<'a>, Set, visit_set_expr);
impl_expr!(Super<'a>, Super, visit_super_expr);
impl_expr!(This<'a>, This, visit_this_expr);
impl_expr!(Unary<'a>, Unary, visit_unary_expr);
impl_expr!(Variable<'a>, Variable, visit_variable_expr);

impl ExprElement for Expr<'_> {
    fn accept<V>(&self, visitor: &mut V) -> <V as ExprVisitor>::Output
    where
        V: ExprVisitor,
    {
        match self {
            Self::Assign(expr) => visitor.visit_assign_expr(expr),
            Self::Binary(expr) => visitor.visit_binary_expr(expr),
            Self::Call(expr) => visitor.visit_call_expr(expr),
            Self::Get(expr) => visitor.visit_get_expr(expr),
            Self::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Self::Literal(expr) => visitor.visit_literal_expr(expr),
            Self::Logical(expr) => visitor.visit_logical_expr(expr),
            Self::Set(expr) => visitor.visit_set_expr(expr),
            Self::Super(expr) => visitor.visit_super_expr(expr),
            Self::This(expr) => visitor.visit_this_expr(expr),
            Self::Unary(expr) => visitor.visit_unary_expr(expr),
            Self::Variable(expr) => visitor.visit_variable_expr(expr),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assign<'a> {
    name: Token<'a>,
    value: Box<Expr<'a>>,
}

impl<'a> Assign<'a> {
    pub fn new(name: Token<'a>, value: impl Into<Expr<'a>>) -> Self {
        Self {
            name,
            value: Box::new(value.into()),
        }
    }

    pub const fn name(&self) -> &Token<'a> {
        &self.name
    }

    pub fn value(&self) -> &Expr<'a> {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binary<'a> {
    left: Box<Expr<'a>>,
    operator: Token<'a>,
    right: Box<Expr<'a>>,
}

impl<'a> Binary<'a> {
    pub fn new(left: impl Into<Expr<'a>>, operator: Token<'a>, right: impl Into<Expr<'a>>) -> Self {
        Self {
            left: Box::new(left.into()),
            operator,
            right: Box::new(right.into()),
        }
    }

    pub const fn left(&self) -> &Expr<'a> {
        &self.left
    }

    pub const fn operator(&self) -> &Token<'a> {
        &self.operator
    }

    pub const fn right(&self) -> &Expr<'a> {
        &self.right
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call<'a> {
    callee: Box<Expr<'a>>,
    paren: Token<'a>,
    arguments: Vec<Expr<'a>>,
}

impl<'a> Call<'a> {
    pub fn new(callee: impl Into<Expr<'a>>, paren: Token<'a>, arguments: Vec<Expr<'a>>) -> Self {
        Self {
            callee: Box::new(callee.into()),
            paren,
            arguments,
        }
    }

    pub const fn callee(&self) -> &Expr<'a> {
        &self.callee
    }

    pub const fn paren(&self) -> &Token<'a> {
        &self.paren
    }

    pub fn arguments(&self) -> &[Expr<'a>] {
        &self.arguments
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Get<'a> {
    object: Box<Expr<'a>>,
    name: Token<'a>,
}

impl<'a> Get<'a> {
    pub fn new(object: impl Into<Expr<'a>>, name: Token<'a>) -> Self {
        Self {
            object: Box::new(object.into()),
            name,
        }
    }

    pub const fn object(&self) -> &Expr<'a> {
        &self.object
    }

    pub const fn name(&self) -> &Token<'a> {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grouping<'a> {
    expression: Box<Expr<'a>>,
}

impl<'a> Grouping<'a> {
    pub fn new(expression: impl Into<Expr<'a>>) -> Self {
        Self {
            expression: Box::new(expression.into()),
        }
    }

    pub const fn expression(&self) -> &Expr<'a> {
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
pub struct Logical<'a> {
    left: Box<Expr<'a>>,
    operator: Token<'a>,
    right: Box<Expr<'a>>,
}

impl<'a> Logical<'a> {
    pub fn new(left: impl Into<Expr<'a>>, operator: Token<'a>, right: impl Into<Expr<'a>>) -> Self {
        Self {
            left: Box::new(left.into()),
            operator,
            right: Box::new(right.into()),
        }
    }

    pub const fn left(&self) -> &Expr<'a> {
        &self.left
    }

    pub const fn operator(&self) -> &Token<'a> {
        &self.operator
    }

    pub const fn right(&self) -> &Expr<'a> {
        &self.right
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Set<'a> {
    object: Box<Expr<'a>>,
    name: Token<'a>,
    value: Box<Expr<'a>>,
}

impl<'a> Set<'a> {
    pub fn new(object: impl Into<Expr<'a>>, name: Token<'a>, value: impl Into<Expr<'a>>) -> Self {
        Self {
            object: Box::new(object.into()),
            name,
            value: Box::new(value.into()),
        }
    }

    pub const fn object(&self) -> &Expr<'a> {
        &self.object
    }

    pub const fn name(&self) -> &Token<'a> {
        &self.name
    }

    pub const fn value(&self) -> &Expr<'a> {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Super<'a> {
    keyword: Token<'a>,
    method: Token<'a>,
}

impl<'a> Super<'a> {
    pub const fn new(keyword: Token<'a>, method: Token<'a>) -> Self {
        Self { keyword, method }
    }

    pub const fn keyword(&self) -> &Token<'a> {
        &self.keyword
    }

    pub const fn method(&self) -> &Token<'a> {
        &self.method
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct This<'a> {
    keyword: Token<'a>,
}

impl<'a> This<'a> {
    pub const fn new(keyword: Token<'a>) -> Self {
        Self { keyword }
    }

    pub const fn keyword(&self) -> &Token<'a> {
        &self.keyword
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unary<'a> {
    operator: Token<'a>,
    right: Box<Expr<'a>>,
}

impl<'a> Unary<'a> {
    pub fn new(operator: Token<'a>, right: impl Into<Expr<'a>>) -> Self {
        Self {
            operator,
            right: Box::new(right.into()),
        }
    }

    pub const fn operator(&self) -> &Token<'a> {
        &self.operator
    }

    pub const fn right(&self) -> &Expr<'a> {
        &self.right
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable<'a> {
    name: Token<'a>,
}

impl<'a> Variable<'a> {
    pub const fn new(name: Token<'a>) -> Self {
        Self { name }
    }

    pub const fn name(&self) -> &Token<'a> {
        &self.name
    }
}
