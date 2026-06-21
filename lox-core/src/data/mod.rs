#[cfg(any(test, feature = "dsl"))]
pub use dsl::*;

use crate::interpreter::RuntimeError;
use crate::runtime::RuntimeContext;
use crate::stmt::Function;
use lasso::{Spur, ThreadedRodeo};
use std::fmt;
use std::fmt::Display;
use std::sync::LazyLock;

static SYMBOL_TABLE: LazyLock<ThreadedRodeo> = LazyLock::new(ThreadedRodeo::new);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol(Spur);

impl Symbol {
    pub fn intern(identifier: &str) -> Self {
        Self(SYMBOL_TABLE.get_or_intern(identifier))
    }

    pub fn as_str(&self) -> &'static str {
        SYMBOL_TABLE.resolve(&self.0)
    }
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        Self::intern(value)
    }
}

impl From<String> for Symbol {
    fn from(value: String) -> Self {
        Self::intern(&value)
    }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        SYMBOL_TABLE.resolve(&self.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    Callable(Callable),
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Bool(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "{value}"),
            Self::Callable(value) => write!(f, "{value}"),
        }
    }
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

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Number(f64::from(value))
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<Callable> for Value {
    fn from(value: Callable) -> Self {
        Self::Callable(value)
    }
}

impl From<LoxFunction> for Value {
    fn from(value: LoxFunction) -> Self {
        Self::Callable(Callable::LoxFunction(value))
    }
}

impl From<NativeFunction> for Value {
    fn from(value: NativeFunction) -> Self {
        Self::Callable(Callable::NativeFunction(value))
    }
}

pub trait Call {
    type Interpreter;

    fn arity(&self) -> usize;

    fn call(
        &self,
        interpreter: &mut Self::Interpreter,
        rtc: &mut RuntimeContext<'_>,
        arguments: &[Value],
    ) -> Result<Value, RuntimeError>;
}

pub type FunPtr = fn(&[Value]) -> Result<Value, RuntimeError>;

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    LoxFunction(LoxFunction),
    NativeFunction(NativeFunction),
}

impl Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoxFunction(fun) => write!(f, "{fun}"),
            Self::NativeFunction(fun) => write!(f, "{fun}"),
        }
    }
}

impl PartialOrd for Callable {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl From<LoxFunction> for Callable {
    fn from(value: LoxFunction) -> Self {
        Self::LoxFunction(value)
    }
}

impl From<NativeFunction> for Callable {
    fn from(value: NativeFunction) -> Self {
        Self::NativeFunction(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoxFunction {
    declaration: Function,
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self.declaration.name())
    }
}

impl LoxFunction {
    pub const fn new(declaration: Function) -> Self {
        Self { declaration }
    }

    pub const fn declaration(&self) -> &Function {
        &self.declaration
    }
}

#[derive(Debug, Clone)]
pub struct NativeFunction {
    name: Symbol,
    parameters: Vec<Symbol>,
    fun_ptr: FunPtr,
}

impl Display for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}

impl NativeFunction {
    pub const fn new(
        name: Symbol,
        parameters: Vec<Symbol>,
        fun_ptr: fn(&[Value]) -> Result<Value, RuntimeError>,
    ) -> Self {
        Self {
            name,
            parameters,
            fun_ptr,
        }
    }

    pub const fn name(&self) -> Symbol {
        self.name
    }

    pub fn parameters(&self) -> &[Symbol] {
        &self.parameters
    }

    pub fn fun_ptr(&self) -> fn(&[Value]) -> Result<Value, RuntimeError> {
        self.fun_ptr
    }
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        // function pointers may not be unique, so we exclude them from the comparison
        self.name == other.name && self.parameters == other.parameters
    }
}

pub fn native_function(
    name: impl Into<Symbol>,
    parameters: impl IntoIterator<Item = Symbol>,
    fun_ptr: FunPtr,
) -> NativeFunction {
    NativeFunction::new(name.into(), parameters.into_iter().collect(), fun_ptr)
}

#[cfg(any(test, feature = "dsl"))]
mod dsl;

#[cfg(any(test, feature = "proptest_support"))]
mod proptest_support;

#[cfg(test)]
mod tests;
