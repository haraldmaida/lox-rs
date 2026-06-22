use crate::data;
use crate::data::{Callable, LoxFunction, NativeFunction, Symbol, Value, native_function};
use crate::environment::{Environment, EnvironmentError};
use crate::expr::{
    Assign, Binary, Call, Expr, ExprElement, ExprVisitor, Get, Grouping, Literal, Logical, Set,
    Super, This, Unary, Variable,
};
use crate::native::clock;
use crate::runtime::RuntimeContext;
use crate::stmt::{
    Block, Class, Expression, Function, If, Print, Return, Stmt, StmtElement, StmtVisitor, Var,
    While,
};
use crate::token::{Token, TokenKind};
use miette::{Diagnostic, SourceSpan};
use std::fmt::Display;
use std::ops::ControlFlow;
use std::ops::ControlFlow::{Break, Continue};
use std::{fmt, mem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeErrorCode {
    CallExprOnNonCallable,
    /// This error should never occur. However, if it does occur, it is a bug
    /// in the parser. Please report an issue!
    NotABinaryOperator,
    /// This error should never occur. However, if it does occur, it is a bug
    /// in the parser. Please report an issue!
    NotAnUnaryOperator,
    OperandNotANumber,
    OperandNotANumberOrString,
    OperandsOfDifferentType,
    UndefinedFunction(Symbol),
    UndefinedVariable(Symbol),
}

impl Display for RuntimeErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CallExprOnNonCallable => write!(f, "cannot call a non-callable value"),
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
            Self::UndefinedFunction(symbol) => write!(f, "call to undefined function '{symbol}'"),
            Self::UndefinedVariable(symbol) => write!(f, "use of undefined variable '{symbol}'"),
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
    pub const fn new(code: RuntimeErrorCode, token: Token) -> Self {
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
pub struct Interpreter {
    environment: Environment,
    globals: Environment,
}

impl Default for Interpreter {
    fn default() -> Self {
        let globals = Environment::new_root();
        globals.define("clock", native_function("clock", [], clock));
        Self {
            environment: globals.clone(),
            globals,
        }
    }
}

impl Interpreter {
    pub const fn environment(&self) -> &Environment {
        &self.environment
    }

    pub const fn globals(&self) -> &Environment {
        &self.globals
    }
}

impl Interpreter {
    pub fn interpret<P>(&mut self, rtc: &mut RuntimeContext<'_>, program: P)
    where
        P: AsRef<[Stmt]>,
    {
        let statements = program.as_ref();
        for stmt in statements {
            if let Break(Err(error)) = self.execute_internal(rtc, stmt) {
                writeln!(rtc.stderr(), "{error}")
                    .unwrap_or_else(|io_err| panic!("failed to write to stderr: {io_err}"));
            }
        }
    }

    pub fn evaluate(
        &mut self,
        rtc: &mut RuntimeContext<'_>,
        expression: &Expr,
    ) -> Result<Value, RuntimeError> {
        match self.evaluate_internal(rtc, expression) {
            Continue(value) | Break(Ok(value)) => Ok(value),
            Break(Err(error)) => Err(error),
        }
    }

    pub fn execute(
        &mut self,
        rtc: &mut RuntimeContext<'_>,
        statement: &Stmt,
    ) -> Result<(), RuntimeError> {
        match self.execute_internal(rtc, statement) {
            Continue(()) | Break(Ok(_)) => Ok(()),
            Break(Err(error)) => Err(error),
        }
    }

    fn evaluate_internal(
        &mut self,
        rtc: &mut RuntimeContext<'_>,
        expression: &Expr,
    ) -> ControlFlow<Result<Value, RuntimeError>, Value> {
        expression.accept(self, rtc)
    }

    fn execute_internal(
        &mut self,
        rtc: &mut RuntimeContext<'_>,
        statement: &Stmt,
    ) -> ControlFlow<Result<Value, RuntimeError>, ()> {
        statement.accept(self, rtc)
    }

    pub fn execute_block(
        &mut self,
        rtc: &mut RuntimeContext<'_>,
        environment: Environment,
        statements: &[Stmt],
    ) -> ControlFlow<Result<Value, RuntimeError>, ()> {
        let previous_environment = mem::replace(&mut self.environment, environment);
        for a_stmt in statements {
            match self.execute_internal(rtc, a_stmt) {
                Continue(()) => {},
                Break(result) => {
                    self.environment = previous_environment;
                    return Break(result);
                },
            }
        }
        self.environment = previous_environment;
        Continue(())
    }
}

impl ExprVisitor for Interpreter {
    type Context<'a> = RuntimeContext<'a>;
    type Output = ControlFlow<Result<Value, RuntimeError>, Value>;

    fn visit_assign_expr(&mut self, rtc: &mut RuntimeContext<'_>, expr: &Assign) -> Self::Output {
        let value = self.evaluate_internal(rtc, expr.value())?;
        let symbol = expr.name().lexeme();
        match self.environment.assign(symbol, value.clone()) {
            Ok(()) => Continue(value),
            Err(EnvironmentError::IdentifierNotFound(symbol)) => Break(Err(RuntimeError::new(
                RuntimeErrorCode::UndefinedVariable(symbol),
                expr.name(),
            ))),
        }
    }

    fn visit_binary_expr(&mut self, rtc: &mut RuntimeContext<'_>, expr: &Binary) -> Self::Output {
        let left = self.evaluate_internal(rtc, expr.left())?;
        let right = self.evaluate_internal(rtc, expr.right())?;

        match expr.operator().kind() {
            TokenKind::BangEqual => Continue(Value::Bool(left != right)),
            TokenKind::EqualEqual => Continue(Value::Bool(left == right)),
            TokenKind::Greater => Continue(Value::Bool(left > right)),
            TokenKind::GreaterEqual => Continue(Value::Bool(left >= right)),
            TokenKind::Less => Continue(Value::Bool(left < right)),
            TokenKind::LessEqual => Continue(Value::Bool(left <= right)),
            TokenKind::Minus => match (left, right) {
                (Value::Number(left), Value::Number(right)) => {
                    Continue(Value::Number(left - right))
                },
                _ => Break(Err(RuntimeError::new(
                    RuntimeErrorCode::OperandNotANumber,
                    expr.operator(),
                ))),
            },
            TokenKind::Plus => match (left, right) {
                (Value::Number(left), Value::Number(right)) => {
                    Continue(Value::Number(left + right))
                },
                (Value::String(left), Value::String(right)) => {
                    Continue(Value::String(format!("{left}{right}")))
                },
                (Value::String(_), Value::Number(_)) | (Value::Number(_), Value::String(_)) => {
                    Break(Err(RuntimeError::new(
                        RuntimeErrorCode::OperandsOfDifferentType,
                        expr.operator(),
                    )))
                },
                _ => Break(Err(RuntimeError::new(
                    RuntimeErrorCode::OperandNotANumberOrString,
                    expr.operator(),
                ))),
            },
            TokenKind::Slash => match (left, right) {
                (Value::Number(left), Value::Number(right)) => {
                    Continue(Value::Number(left / right))
                },
                _ => Break(Err(RuntimeError::new(
                    RuntimeErrorCode::OperandNotANumber,
                    expr.operator(),
                ))),
            },
            TokenKind::Star => match (left, right) {
                (Value::Number(left), Value::Number(right)) => {
                    Continue(Value::Number(left * right))
                },
                _ => Break(Err(RuntimeError::new(
                    RuntimeErrorCode::OperandNotANumber,
                    expr.operator(),
                ))),
            },
            _ => Break(Err(RuntimeError::new(
                RuntimeErrorCode::NotABinaryOperator,
                expr.operator(),
            ))),
        }
    }

    fn visit_call_expr(&mut self, rtc: &mut RuntimeContext<'_>, expr: &Call) -> Self::Output {
        let callee = self.evaluate_internal(rtc, expr.callee())?;
        let mut arguments = Vec::with_capacity(expr.arguments().len());
        for argument in expr.arguments() {
            let arg = self.evaluate_internal(rtc, argument)?;
            arguments.push(arg);
        }
        if let Value::Callable(callable) = callee {
            match data::Call::call(&callable, self, rtc, &arguments) {
                Ok(value) => Continue(value),
                Err(error) => Break(Err(error)),
            }
        } else {
            Break(Err(RuntimeError::new(
                RuntimeErrorCode::CallExprOnNonCallable,
                expr.paren(), // TODO improve error message for call expr
            )))
        }
    }

    fn visit_get_expr(&mut self, _rtc: &mut RuntimeContext<'_>, _expr: &Get) -> Self::Output {
        todo!()
    }

    fn visit_grouping_expr(
        &mut self,
        rtc: &mut RuntimeContext<'_>,
        expr: &Grouping,
    ) -> Self::Output {
        self.evaluate_internal(rtc, expr.expression())
    }

    fn visit_literal_expr(
        &mut self,
        _rtc: &mut RuntimeContext<'_>,
        expr: &Literal,
    ) -> Self::Output {
        match expr {
            Literal::Nil => Continue(Value::Nil),
            Literal::Bool(value) => Continue(Value::Bool(*value)),
            Literal::Number(value) => Continue(Value::Number(*value)),
            Literal::String(value) => Continue(Value::String(value.to_string())),
        }
    }

    fn visit_logical_expr(&mut self, rtc: &mut RuntimeContext<'_>, expr: &Logical) -> Self::Output {
        let left = self.evaluate_internal(rtc, expr.left())?;
        if expr.operator().kind() == TokenKind::Or {
            if left.is_truthy() {
                return Continue(left);
            }
        } else if !left.is_truthy() {
            return Continue(left);
        }
        self.evaluate_internal(rtc, expr.right())
    }

    fn visit_set_expr(&mut self, _rtc: &mut RuntimeContext<'_>, _expr: &Set) -> Self::Output {
        todo!()
    }

    fn visit_super_expr(&mut self, _rtc: &mut RuntimeContext<'_>, _expr: &Super) -> Self::Output {
        todo!()
    }

    fn visit_this_expr(&mut self, _rtc: &mut RuntimeContext<'_>, _expr: &This) -> Self::Output {
        todo!()
    }

    fn visit_unary_expr(&mut self, rtc: &mut RuntimeContext<'_>, expr: &Unary) -> Self::Output {
        let right = self.evaluate_internal(rtc, expr.right())?;
        match expr.operator().kind() {
            TokenKind::Bang => Continue(Value::Bool(!right.is_truthy())),
            TokenKind::Minus => {
                if let Value::Number(number) = right {
                    Continue(Value::Number(-number))
                } else {
                    Break(Err(RuntimeError::new(
                        RuntimeErrorCode::OperandNotANumber,
                        expr.operator(),
                    )))
                }
            },
            _ => Break(Err(RuntimeError::new(
                RuntimeErrorCode::NotAnUnaryOperator,
                expr.operator(),
            ))),
        }
    }

    fn visit_variable_expr(
        &mut self,
        _rtc: &mut RuntimeContext<'_>,
        expr: &Variable,
    ) -> Self::Output {
        let symbol = expr.name().lexeme();
        match self.environment.lookup(symbol) {
            Ok(value) => Continue(value),
            Err(EnvironmentError::IdentifierNotFound(symbol)) => Break(Err(RuntimeError::new(
                RuntimeErrorCode::UndefinedVariable(symbol),
                expr.name(),
            ))),
        }
    }
}

impl StmtVisitor for Interpreter {
    type Context<'c> = RuntimeContext<'c>;
    type Output = ControlFlow<Result<Value, RuntimeError>, ()>;

    fn visit_block_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &Block) -> Self::Output {
        self.execute_block(rtc, self.environment.new_local(), stmt.statements())
    }

    fn visit_class_stmt(&mut self, _rtc: &mut RuntimeContext<'_>, _stmt: &Class) -> Self::Output {
        todo!()
    }

    fn visit_expression_stmt(
        &mut self,
        rtc: &mut RuntimeContext<'_>,
        stmt: &Expression,
    ) -> Self::Output {
        self.evaluate_internal(rtc, stmt.expression())?;
        Continue(())
    }

    fn visit_function_stmt(
        &mut self,
        _rtc: &mut RuntimeContext<'_>,
        stmt: &Function,
    ) -> Self::Output {
        let function = LoxFunction::new(stmt.clone(), self.environment.clone());
        self.environment.define(stmt.name().lexeme(), function);
        Continue(())
    }

    fn visit_if_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &If) -> Self::Output {
        if self.evaluate_internal(rtc, stmt.condition())?.is_truthy() {
            self.execute_internal(rtc, stmt.then_branch())
        } else if let Some(else_branch) = stmt.else_branch() {
            self.execute_internal(rtc, else_branch)
        } else {
            Continue(())
        }
    }

    fn visit_print_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &Print) -> Self::Output {
        let value = self.evaluate_internal(rtc, stmt.expression())?;
        if let Err(err) = writeln!(rtc.stdout(), "{value}") {
            writeln!(rtc.stderr(), "{err}").expect("failed to write to stderr");
        }
        Continue(())
    }

    fn visit_return_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &Return) -> Self::Output {
        let return_value = match stmt.value() {
            None => Value::Nil,
            Some(expression) => match self.evaluate_internal(rtc, expression) {
                Continue(value) | Break(Ok(value)) => value,
                Break(Err(error)) => return Break(Err(error)),
            },
        };
        Break(Ok(return_value))
    }

    fn visit_var_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &Var) -> Self::Output {
        let value = if let Some(initializer) = stmt.initializer() {
            self.evaluate_internal(rtc, initializer)?
        } else {
            Value::Nil
        };
        let symbol = stmt.name().lexeme();
        self.environment.define(symbol, value);
        Continue(())
    }

    fn visit_while_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &While) -> Self::Output {
        while self.evaluate_internal(rtc, stmt.condition())?.is_truthy() {
            self.execute_internal(rtc, stmt.body())?;
        }
        Continue(())
    }
}

impl data::Call for Callable {
    type Interpreter = Interpreter;
    type Context<'c> = <Self::Interpreter as StmtVisitor>::Context<'c>;

    fn arity(&self) -> usize {
        match self {
            Self::LoxFunction(fun) => fun.arity(),
            Self::NativeFunction(fun) => fun.arity(),
        }
    }

    fn call(
        &self,
        interpreter: &mut Self::Interpreter,
        ctx: &mut RuntimeContext<'_>,
        arguments: &[Value],
    ) -> Result<Value, RuntimeError> {
        match self {
            Self::LoxFunction(fun) => fun.call(interpreter, ctx, arguments),
            Self::NativeFunction(fun) => fun.call(&mut (), &mut (), arguments),
        }
    }
}

impl data::Call for LoxFunction {
    type Interpreter = Interpreter;
    type Context<'c> = <Self::Interpreter as StmtVisitor>::Context<'c>;

    fn arity(&self) -> usize {
        self.declaration().parameters().len()
    }

    fn call(
        &self,
        interpreter: &mut Self::Interpreter,
        ctx: &mut Self::Context<'_>,
        arguments: &[Value],
    ) -> Result<Value, RuntimeError> {
        let environment = self.closure().new_local();
        self.declaration()
            .parameters()
            .iter()
            .zip(arguments.iter())
            .for_each(|(param, arg)| environment.define(param.lexeme(), arg.clone()));

        match interpreter.execute_block(ctx, environment, self.declaration().body()) {
            Continue(()) => Ok(Value::Nil),
            Break(Ok(value)) => Ok(value),
            Break(Err(error)) => Err(error),
        }
    }
}

impl data::Call for NativeFunction {
    type Interpreter = ();
    type Context<'c> = ();

    fn arity(&self) -> usize {
        self.parameters().len()
    }

    fn call(
        &self,
        _interpreter: &mut Self::Interpreter,
        _ctx: &mut Self::Context<'_>,
        arguments: &[Value],
    ) -> Result<Value, RuntimeError> {
        self.fun_ptr()(arguments)
    }
}

#[cfg(test)]
mod tests;
