use crate::data::{Symbol, Value};
use crate::expr::{
    Assign, Binary, Call, Expr, ExprElement, ExprVisitor, Get, Grouping, Literal, Logical, Set,
    Super, This, Unary, Variable,
};
use crate::stmt::{Expression, Print, Stmt, StmtElement, StmtVisitor, Var};
use crate::token::{Token, TokenKind};
use miette::{Diagnostic, SourceSpan};
use std::fmt;
use std::fmt::Display;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeErrorCode {
    /// This error should never occur. However, if it does occur, it is a bug
    /// in the parser. Please report an issue!
    NotABinaryOperator,
    /// This error should never occur. However, if it does occur, it is a bug
    /// in the parser. Please report an issue!
    NotAnUnaryOperator,
    OperandNotANumber,
    OperandNotANumberOrString,
    OperandsOfDifferentType,
    UndefinedVariable(Symbol),
}

impl Display for RuntimeErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotABinaryOperator => write!(
                f,
                "not a binary operator where a binary operator like '=', '+', '-', '*' or '/' is expected"
            ),
            Self::NotAnUnaryOperator => write!(
                f,
                "not an unary operator where an unary operator like '!' or '-'  is expected"
            ),
            Self::OperandNotANumber => write!(
                f,
                "operand is not a number but the operator requires all operands to be numbers"
            ),
            Self::OperandNotANumberOrString => write!(
                f,
                "operand is not a number or a string but the operator requires all operands to be either numbers or strings"
            ),
            Self::OperandsOfDifferentType => write!(
                f,
                "operands are of different type but the operator requires all operands to be of the same type"
            ),
            Self::UndefinedVariable(symbol) => write!(f, "undefined variable '{symbol}'"),
        }
    }
}

#[derive(thiserror::Error, Diagnostic, Debug, Clone, PartialEq, Eq)]
#[error("{code}")]
pub struct RuntimeError {
    code: RuntimeErrorCode,
    operation: TokenKind,
    #[label]
    location: SourceSpan,
}

impl RuntimeError {
    pub const fn new(code: RuntimeErrorCode, token: Token<'_>) -> Self {
        Self {
            code,
            operation: token.kind,
            location: token.location,
        }
    }

    pub const fn code(&self) -> RuntimeErrorCode {
        self.code
    }

    pub const fn operation(&self) -> TokenKind {
        self.operation
    }

    pub const fn location(&self) -> SourceSpan {
        self.location
    }
}

/// A tree-walk interpreter for Lox.
#[derive(Default)]
pub struct Interpreter {}

impl Interpreter {
    pub fn interpret<'a, P>(&mut self, program: P)
    where
        P: AsRef<[Stmt<'a>]>,
    {
        let statements = program.as_ref();
        for stmt in statements {
            if let Err(error) = self.execute(stmt) {
                eprintln!("{error}");
            }
        }
    }

    pub fn execute(&mut self, statement: &Stmt<'_>) -> Result<(), RuntimeError> {
        statement.accept(self)
    }

    pub fn evaluate(&mut self, expression: &Expr<'_>) -> Result<Value, RuntimeError> {
        expression.accept(self)
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
                _ => Err(RuntimeError::new(
                    RuntimeErrorCode::OperandNotANumber,
                    *expr.operator(),
                )),
            },
            TokenKind::Plus => match (left, right) {
                (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left + right)),
                (Value::String(left), Value::String(right)) => {
                    Ok(Value::String(format!("{left}{right}")))
                },
                (Value::String(_), Value::Number(_)) | (Value::Number(_), Value::String(_)) => Err(
                    RuntimeError::new(RuntimeErrorCode::OperandsOfDifferentType, *expr.operator()),
                ),
                _ => Err(RuntimeError::new(
                    RuntimeErrorCode::OperandNotANumberOrString,
                    *expr.operator(),
                )),
            },
            TokenKind::Slash => match (left, right) {
                (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left / right)),
                _ => Err(RuntimeError::new(
                    RuntimeErrorCode::OperandNotANumber,
                    *expr.operator(),
                )),
            },
            TokenKind::Star => match (left, right) {
                (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left * right)),
                _ => Err(RuntimeError::new(
                    RuntimeErrorCode::OperandNotANumber,
                    *expr.operator(),
                )),
            },
            _ => Err(RuntimeError::new(
                RuntimeErrorCode::NotABinaryOperator,
                *expr.operator(),
            )),
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
            Literal::String(value) => Ok(Value::String(value.to_string())),
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
            TokenKind::Bang => Ok(Value::Bool(!right.is_truthy())),
            TokenKind::Minus => {
                if let Value::Number(number) = right {
                    Ok(Value::Number(-number))
                } else {
                    Err(RuntimeError::new(
                        RuntimeErrorCode::OperandNotANumber,
                        *expr.operator(),
                    ))
                }
            },
            _ => Err(RuntimeError::new(
                RuntimeErrorCode::NotAnUnaryOperator,
                *expr.operator(),
            )),
        }
    }

    fn visit_variable_expr(&mut self, _expr: &Variable) -> Self::Output {
        todo!()
    }
}

impl StmtVisitor for Interpreter {
    type Output = Result<(), RuntimeError>;

    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Self::Output {
        self.evaluate(stmt.expression())?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> Self::Output {
        let value = self.evaluate(stmt.expression())?;
        println!("{value}");
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> Self::Output {
        todo!()
    }
}
