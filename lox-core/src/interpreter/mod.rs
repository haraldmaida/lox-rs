use crate::data::{
    Callable, LoxClass, LoxFunction, LoxObject, NativeFunction, Symbol, Value, native_function,
};
use crate::environment::{Environment, EnvironmentError};
use crate::expr::{
    Assign, Binary, Call, Expr, ExprElement, ExprVisitor, Get, Grouping, Literal, Logical, Set,
    Super, This, Unary, Variable,
};
use crate::native::clock;
use crate::program::Program;
use crate::resolver::ResolutionMap;
use crate::runtime::RuntimeContext;
use crate::stmt::{
    Block, Class, Expression, Function, If, Print, Return, Stmt, StmtElement, StmtVisitor, Var,
    While,
};
use crate::token::{Token, TokenKind};
use miette::{Diagnostic, SourceSpan};
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::ControlFlow;
use std::ops::ControlFlow::{Break, Continue};
use std::{fmt, mem};

pub const INIT_METHOD: &str = "init";
pub const SUPER: &str = "super";
pub const THIS: &str = "this";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeErrorCode {
    AccessingPropertyOnNonObject,
    AccessingSuperOutsideOfObject,
    AssignFieldOnNonObject,
    CallExprOnNonCallable,
    CallWithTooFewArguments {
        expected: usize,
        actual: usize,
    },
    CallWithTooManyArguments {
        expected: usize,
        actual: usize,
    },
    ClassDoesNotHaveSuperclass(Symbol),
    /// This error should never occur. However, if it does occur, it is a bug
    /// in the parser. Please report an issue!
    NotABinaryOperator,
    /// This error should never occur. However, if it does occur, it is a bug
    /// in the parser. Please report an issue!
    NotAnUnaryOperator,
    OperandNotANumber,
    OperandNotANumberOrString,
    OperandsOfDifferentType,
    SuperclassIsNotAClass,
    UndefinedClass(Symbol),
    UndefinedFunction(Symbol),
    UndefinedProperty(Symbol),
    UndefinedVariable(Symbol),
}

impl Display for RuntimeErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccessingPropertyOnNonObject => {
                write!(f, "accessing property on a non-object value")
            },
            Self::AccessingSuperOutsideOfObject => {
                write!(f, "accessing 'super' outside of an object")
            },
            Self::AssignFieldOnNonObject => {
                write!(f, "assigning to field on a non-object value")
            },
            Self::CallExprOnNonCallable => write!(f, "can not call a non-callable value"),
            Self::CallWithTooFewArguments { expected, actual } => write!(
                f,
                "call with too few arguments, expected {expected} but got {actual}",
            ),
            Self::CallWithTooManyArguments { expected, actual } => write!(
                f,
                "call with too many arguments, expected {expected} but got {actual}",
            ),
            Self::ClassDoesNotHaveSuperclass(symbol) => {
                write!(f, "the class {symbol} does not have a superclass")
            },
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
            Self::SuperclassIsNotAClass => write!(f, "superclass must be a class"),
            Self::UndefinedClass(symbol) => write!(f, "use of undefined class '{symbol}'"),
            Self::UndefinedFunction(symbol) => write!(f, "call to undefined function '{symbol}'"),
            Self::UndefinedProperty(symbol) => write!(f, "use of undefined property '{symbol}'"),
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
    locals: ResolutionMap,
}

impl Default for Interpreter {
    fn default() -> Self {
        let globals = Environment::new_root();
        globals.define("clock", native_function("clock", [], clock));
        Self {
            environment: globals.clone(),
            globals,
            locals: ResolutionMap::default(),
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
    pub fn interpret(&mut self, rtc: &mut RuntimeContext<'_>, program: &Program) {
        self.locals.clone_from(program.resolution_map());
        let statements = program.statements();
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

    pub fn lookup_variable(&mut self, name: Token) -> Result<Value, EnvironmentError> {
        if let Some(distance) = self.locals.get_distance(name) {
            self.environment.lookup_at(distance, name.lexeme())
        } else {
            self.globals.lookup(name.lexeme())
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
        let assign_result = if let Some(distance) = self.locals.get_distance(expr.name()) {
            self.environment.assign_at(distance, symbol, value.clone())
        } else {
            self.globals.assign(symbol, value.clone())
        };
        match assign_result {
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
        match callee {
            Value::Nil
            | Value::Bool(_)
            | Value::Number(_)
            | Value::String(_)
            | Value::Object(_) => {
                Break(Err(RuntimeError::new(
                    RuntimeErrorCode::CallExprOnNonCallable,
                    expr.paren(), // TODO improve error message for call expr
                )))
            },
            Value::Function(function) => {
                match check_arity_of_call(expr.paren(), &function, arguments.len())
                    .and_then(|()| function.call(self, rtc, &arguments))
                {
                    Ok(value) => Continue(value),
                    Err(error) => Break(Err(error)),
                }
            },
            Value::NativeFunction(native_function) => {
                match check_arity_of_call(expr.paren(), &native_function, arguments.len())
                    .and_then(|()| native_function.call(&mut (), &mut (), &arguments))
                {
                    Ok(value) => Continue(value),
                    Err(error) => Break(Err(error)),
                }
            },
            Value::Class(class) => match check_arity_of_call(expr.paren(), &class, arguments.len())
                .and_then(|()| class.call(self, rtc, &arguments))
            {
                Ok(value) => Continue(value),
                Err(error) => Break(Err(error)),
            },
        }
    }

    fn visit_get_expr(&mut self, rtc: &mut RuntimeContext<'_>, expr: &Get) -> Self::Output {
        let object = self.evaluate_internal(rtc, expr.object())?;
        if let Value::Object(object) = object {
            match object.get(expr.name()) {
                Some(Value::Function(method)) => {
                    let bound_method = method.bind(object);
                    Continue(Value::Function(bound_method))
                },
                Some(value) => Continue(value),
                None => Break(Err(RuntimeError::new(
                    RuntimeErrorCode::UndefinedProperty(expr.name().lexeme),
                    expr.name(),
                ))),
            }
        } else {
            Break(Err(RuntimeError::new(
                RuntimeErrorCode::AccessingPropertyOnNonObject,
                expr.name(),
            )))
        }
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

    fn visit_set_expr(&mut self, rtc: &mut RuntimeContext<'_>, expr: &Set) -> Self::Output {
        let object = self.evaluate_internal(rtc, expr.object())?;
        if let Value::Object(object) = object {
            let value = self.evaluate_internal(rtc, expr.value())?;
            object.set(expr.name(), value.clone());
            Continue(value)
        } else {
            Break(Err(RuntimeError::new(
                RuntimeErrorCode::AssignFieldOnNonObject,
                expr.name(),
            )))
        }
    }

    fn visit_super_expr(&mut self, _rtc: &mut RuntimeContext<'_>, expr: &Super) -> Self::Output {
        let Some(distance) = self.locals.get_distance(expr.keyword()) else {
            unreachable!(
                "'super' should be defined by the resolver by now! please file a bug report."
            )
        };
        let Ok(Value::Object(object)) = self.environment.lookup_at(distance - 1, THIS) else {
            return Break(Err(RuntimeError::new(
                RuntimeErrorCode::AccessingSuperOutsideOfObject,
                expr.keyword(),
            )));
        };
        let Ok(Value::Class(superclass)) = self.environment.lookup_at(distance, SUPER) else {
            return Break(Err(RuntimeError::new(
                RuntimeErrorCode::ClassDoesNotHaveSuperclass(object.class_name()),
                expr.keyword(),
            )));
        };

        let Some(Value::Function(method)) = superclass.find_method(expr.method().lexeme()) else {
            return Break(Err(RuntimeError::new(
                RuntimeErrorCode::UndefinedProperty(expr.method().lexeme),
                expr.method(),
            )));
        };
        Continue(Value::Function(method.clone().bind(object)))
    }

    fn visit_this_expr(&mut self, _rtc: &mut RuntimeContext<'_>, expr: &This) -> Self::Output {
        match self.lookup_variable(expr.keyword()) {
            Ok(value) => Continue(value),
            Err(EnvironmentError::IdentifierNotFound(symbol)) => Break(Err(RuntimeError::new(
                RuntimeErrorCode::UndefinedProperty(symbol),
                expr.keyword(),
            ))),
        }
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
        match self.lookup_variable(expr.name()) {
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

    fn visit_class_stmt(&mut self, rtc: &mut RuntimeContext<'_>, stmt: &Class) -> Self::Output {
        let superclass = if let Some(superclass) = stmt.superclass() {
            match self.visit_variable_expr(rtc, superclass) {
                Continue(Value::Class(superclass)) => Some(superclass),
                Continue(_) | Break(Ok(_)) => {
                    return Break(Err(RuntimeError::new(
                        RuntimeErrorCode::SuperclassIsNotAClass,
                        superclass.name(),
                    )));
                },
                Break(Err(error)) => return Break(Err(error)),
            }
        } else {
            None
        };
        let class_name = stmt.name().lexeme();
        self.environment.define(class_name, Value::Nil);
        if let Some(superclass) = &superclass {
            self.environment = self.environment.new_local();
            self.environment
                .define(Symbol::from(SUPER), Value::Class(superclass.clone()));
        }
        let methods = stmt
            .methods()
            .iter()
            .map(|method| {
                (
                    method.name().lexeme,
                    Value::from(LoxFunction::new(
                        method.clone(),
                        self.environment.clone(),
                        method.name().lexeme == Symbol::from(INIT_METHOD),
                    )),
                )
            })
            .collect();
        if superclass.is_some() {
            self.environment = self.environment.enclosing();
        }
        let class = LoxClass::new(class_name, superclass, methods);
        match self.environment.assign(class_name, class) {
            Ok(()) => {},
            Err(EnvironmentError::IdentifierNotFound(_)) => {
                return Break(Err(RuntimeError::new(
                    RuntimeErrorCode::UndefinedClass(class_name),
                    stmt.name(),
                )));
            },
        }
        Continue(())
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
        let function = LoxFunction::new(stmt.clone(), self.environment.clone(), false);
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

fn check_arity_of_call(
    token: Token,
    callable: &impl Callable,
    num_arguments: usize,
) -> Result<(), RuntimeError> {
    match num_arguments.cmp(&callable.arity()) {
        Ordering::Equal => Ok(()),
        Ordering::Greater => Err(RuntimeError::new(
            RuntimeErrorCode::CallWithTooManyArguments {
                expected: callable.arity(),
                actual: num_arguments,
            },
            token,
        )),
        Ordering::Less => Err(RuntimeError::new(
            RuntimeErrorCode::CallWithTooFewArguments {
                expected: callable.arity(),
                actual: num_arguments,
            },
            token,
        )),
    }
}

impl Callable for LoxFunction {
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
            Continue(()) => {
                if self.is_initializer()
                    && let Ok(this_value) = self.closure().lookup_at(0, THIS)
                {
                    Ok(this_value)
                } else {
                    Ok(Value::Nil)
                }
            },
            Break(Ok(value)) => {
                if self.is_initializer()
                    && let Ok(this_value) = self.closure().lookup_at(0, THIS)
                {
                    Ok(this_value)
                } else {
                    Ok(value)
                }
            },
            Break(Err(error)) => Err(error),
        }
    }
}

impl Callable for NativeFunction {
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

impl Callable for LoxClass {
    type Interpreter = Interpreter;
    type Context<'c> = <Self::Interpreter as StmtVisitor>::Context<'c>;

    fn arity(&self) -> usize {
        if let Some(Value::Function(initializer)) = self.find_method(INIT_METHOD.into()) {
            initializer.arity()
        } else {
            0
        }
    }

    fn call(
        &self,
        interpreter: &mut Self::Interpreter,
        ctx: &mut Self::Context<'_>,
        arguments: &[Value],
    ) -> Result<Value, RuntimeError> {
        let instance = LoxObject::new(self.clone());
        if let Some(Value::Function(initializer)) = self.find_method(INIT_METHOD.into()) {
            initializer
                .clone()
                .bind(instance.clone())
                .call(interpreter, ctx, arguments)?;
        }
        Ok(Value::Object(instance))
    }
}

#[cfg(test)]
mod tests;
