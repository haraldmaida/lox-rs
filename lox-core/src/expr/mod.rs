use crate::token::{Literal, Token};

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
    pub fn new(object: Expr, name: Token) -> Self {
        Self {
            object: Box::new(object),
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
    pub fn new(expression: Expr) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }

    pub const fn expression(&self) -> &Expr {
        &self.expression
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
    pub fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
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
