use crate::expr::{
    Assign, Binary, Call, Expr, ExprElement, ExprVisitor, Get, Grouping, Literal, Logical, Set,
    Super, This, Unary, Variable,
};
use crate::token::{Token, TokenKind};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

impl Value {
    /// Returns whether this value is true for all types.
    ///
    /// In dynamically typed languages, logical operations can be applied to any
    /// type of value. Each language defines what values are "truthy" and what
    /// are not.
    ///
    /// For Lox, we follow the simple rules of Lua and Ruby:
    ///
    /// * `false` and `nil` are falsy
    /// * everything else is truthy
    pub const fn is_truthy(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Bool(value) => *value,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeErrorCode {
    NumberOperandExpected,
    StringOperandExpected,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeError {
    code: RuntimeErrorCode,
    token: Token,
}

impl RuntimeError {
    pub const fn new(code: RuntimeErrorCode, token: Token) -> Self {
        Self { code, token }
    }

    pub const fn code(&self) -> RuntimeErrorCode {
        self.code
    }

    pub const fn token(&self) -> &Token {
        &self.token
    }
}

/// A tree-walk interpreter for Lox.
#[derive(Default)]
pub struct Interpreter {}

impl Interpreter {
    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        expr.accept(self)
    }
}

impl ExprVisitor for Interpreter {
    type Output = Result<Value, RuntimeError>;

    fn visit_assign_expr(&mut self, _expr: &Assign) -> Self::Output {
        todo!()
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> Self::Output {
        let left = self.evaluate(expr.left())?;
        let right = self.evaluate(expr.right())?;

        match expr.operator().kind() {
            TokenKind::BangEqual => Ok(Value::Bool(left != right)),
            TokenKind::EqualEqual => Ok(Value::Bool(left == right)),
            TokenKind::Greater => Ok(Value::Bool(left > right)),
            TokenKind::GreaterEqual => Ok(Value::Bool(left >= right)),
            TokenKind::Less => Ok(Value::Bool(left < right)),
            TokenKind::LessEqual => Ok(Value::Bool(left <= right)),
            TokenKind::Minus => match (left, right) {
                (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left - right)),
                _ => todo!("error handling for binary operation"),
            },
            TokenKind::Plus => match (left, right) {
                (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left + right)),
                (Value::String(left), Value::String(right)) => {
                    Ok(Value::String(format!("{left}{right}")))
                },
                _ => todo!("error handling for binary operation"),
            },
            TokenKind::Slash => match (left, right) {
                (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left / right)),
                _ => todo!("error handling for binary operation"),
            },
            TokenKind::Star => match (left, right) {
                (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left * right)),
                _ => todo!("error handling for binary operation"),
            },
            _ => todo!("error handling for binary operation"),
        }
    }

    fn visit_call_expr(&mut self, _expr: &Call) -> Self::Output {
        todo!()
    }

    fn visit_get_expr(&mut self, _expr: &Get) -> Self::Output {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Self::Output {
        self.evaluate(expr.expression())
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> Self::Output {
        match expr {
            Literal::Nil => Ok(Value::Nil),
            Literal::Bool(value) => Ok(Value::Bool(*value)),
            Literal::Number(value) => Ok(Value::Number(*value)),
            Literal::String(value) => Ok(Value::String(value.clone())),
        }
    }

    fn visit_logical_expr(&mut self, _expr: &Logical) -> Self::Output {
        todo!()
    }

    fn visit_set_expr(&mut self, _expr: &Set) -> Self::Output {
        todo!()
    }

    fn visit_super_expr(&mut self, _expr: &Super) -> Self::Output {
        todo!()
    }

    fn visit_this_expr(&mut self, _expr: &This) -> Self::Output {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Self::Output {
        let right = self.evaluate(expr.right())?;
        match expr.operator().kind() {
            TokenKind::Minus => {
                if let Value::Number(number) = right {
                    Ok(Value::Number(-number))
                } else {
                    todo!("error handling for unary operation")
                }
            },
            TokenKind::Bang => Ok(Value::Bool(!right.is_truthy())),
            _ => todo!("error handling for unary operation"),
        }
    }

    fn visit_variable_expr(&mut self, _expr: &Variable) -> Self::Output {
        todo!()
    }
}
