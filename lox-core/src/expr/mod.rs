use crate::token::{Literal, Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Self>,
    },
    Binary {
        left: Box<Self>,
        operator: Token,
        right: Box<Self>,
    },
    Call {
        callee: Box<Self>,
        paren: Token,
        arguments: Vec<Self>,
    },
    Get {
        object: Box<Self>,
        name: Token,
    },
    Grouping {
        expression: Box<Self>,
    },
    Literal {
        value: Literal,
    },
    Logical {
        left: Box<Self>,
        operator: Token,
        right: Box<Self>,
    },
    Set {
        object: Box<Self>,
        name: Token,
        value: Box<Self>,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<Self>,
    },
    Variable {
        name: Token,
    },
}
