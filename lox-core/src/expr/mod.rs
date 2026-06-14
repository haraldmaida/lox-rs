use crate::token::Token;

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
    ($expr_type:ty, $variant:ident) => {
        impl From<$expr_type> for Expr {
            fn from(expr: $expr_type) -> Self {
                Self::$variant(expr)
            }
        }
    };
}

impl_expr!(Assign, Assign);
impl_expr!(Binary, Binary);
impl_expr!(Call, Call);
impl_expr!(Get, Get);
impl_expr!(Grouping, Grouping);
impl_expr!(Literal, Literal);
impl_expr!(Logical, Logical);
impl_expr!(Set, Set);
impl_expr!(Super, Super);
impl_expr!(This, This);
impl_expr!(Unary, Unary);
impl_expr!(Variable, Variable);

#[derive(Debug, Clone, PartialEq)]
pub struct Assign {
    name: Token,
    value: Box<Expr>,
}

impl Assign {
    pub fn new(name: Token, value: impl Into<Expr>) -> Self {
        Self {
            name,
            value: Box::new(value.into()),
        }
    }

    pub const fn name(&self) -> &Token {
        &self.name
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
    pub fn new(left: impl Into<Expr>, operator: Token, right: impl Into<Expr>) -> Self {
        Self {
            left: Box::new(left.into()),
            operator,
            right: Box::new(right.into()),
        }
    }

    pub const fn left(&self) -> &Expr {
        &self.left
    }

    pub const fn operator(&self) -> &Token {
        &self.operator
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
    pub fn new(callee: impl Into<Expr>, paren: Token, arguments: Vec<Expr>) -> Self {
        Self {
            callee: Box::new(callee.into()),
            paren,
            arguments,
        }
    }

    pub const fn callee(&self) -> &Expr {
        &self.callee
    }

    pub const fn paren(&self) -> &Token {
        &self.paren
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
    pub fn new(object: impl Into<Expr>, name: Token) -> Self {
        Self {
            object: Box::new(object.into()),
            name,
        }
    }

    pub const fn object(&self) -> &Expr {
        &self.object
    }

    pub const fn name(&self) -> &Token {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grouping {
    expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: impl Into<Expr>) -> Self {
        Self {
            expression: Box::new(expression.into()),
        }
    }

    pub const fn expression(&self) -> &Expr {
        &self.expression
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
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
        Self::String(value)
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
    pub fn new(left: impl Into<Expr>, operator: Token, right: impl Into<Expr>) -> Self {
        Self {
            left: Box::new(left.into()),
            operator,
            right: Box::new(right.into()),
        }
    }

    pub const fn left(&self) -> &Expr {
        &self.left
    }

    pub const fn operator(&self) -> &Token {
        &self.operator
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
    pub fn new(object: impl Into<Expr>, name: Token, value: impl Into<Expr>) -> Self {
        Self {
            object: Box::new(object.into()),
            name,
            value: Box::new(value.into()),
        }
    }

    pub const fn object(&self) -> &Expr {
        &self.object
    }

    pub const fn name(&self) -> &Token {
        &self.name
    }

    pub const fn value(&self) -> &Expr {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Super {
    keyword: Token,
    method: Token,
}

impl Super {
    pub const fn new(keyword: Token, method: Token) -> Self {
        Self { keyword, method }
    }

    pub const fn keyword(&self) -> &Token {
        &self.keyword
    }

    pub const fn method(&self) -> &Token {
        &self.method
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct This {
    keyword: Token,
}

impl This {
    pub const fn new(keyword: Token) -> Self {
        Self { keyword }
    }

    pub const fn keyword(&self) -> &Token {
        &self.keyword
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unary {
    operator: Token,
    right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: impl Into<Expr>) -> Self {
        Self {
            operator,
            right: Box::new(right.into()),
        }
    }

    pub const fn operator(&self) -> &Token {
        &self.operator
    }

    pub const fn right(&self) -> &Expr {
        &self.right
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    name: Token,
}

impl Variable {
    pub const fn new(name: Token) -> Self {
        Self { name }
    }

    pub const fn name(&self) -> &Token {
        &self.name
    }
}
