#[cfg(any(test, feature = "dsl"))]
pub use dsl::*;

use crate::environment::Environment;
use crate::interpreter::RuntimeError;
use crate::stmt::Function;
use crate::token::Token;
use lasso::{Spur, ThreadedRodeo};
use rustc_hash::FxBuildHasher;
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Display};
use std::rc::Rc;
use std::sync::LazyLock;

pub type HashMap<K, V> = std::collections::HashMap<K, V, FxBuildHasher>;

static SYMBOL_TABLE: LazyLock<ThreadedRodeo> = LazyLock::new(ThreadedRodeo::new);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol(Spur);

impl Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Symbol({:?}:{})", self.0, self.as_str())
    }
}

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
    Function(LoxFunction),
    NativeFunction(NativeFunction),
    Class(LoxClass),
    Object(LoxObject),
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Bool(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "{value}"),
            Self::Function(value) => write!(f, "{value}"),
            Self::NativeFunction(value) => write!(f, "{value}"),
            Self::Class(value) => write!(f, "{value}"),
            Self::Object(value) => write!(f, "{value}"),
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

impl From<LoxFunction> for Value {
    fn from(value: LoxFunction) -> Self {
        Self::Function(value)
    }
}

impl From<NativeFunction> for Value {
    fn from(value: NativeFunction) -> Self {
        Self::NativeFunction(value)
    }
}

impl From<LoxClass> for Value {
    fn from(value: LoxClass) -> Self {
        Self::Class(value)
    }
}

impl From<LoxObject> for Value {
    fn from(value: LoxObject) -> Self {
        Self::Object(value)
    }
}

pub trait Callable {
    type Interpreter;
    type Context<'c>;

    fn arity(&self) -> usize;

    fn call(
        &self,
        interpreter: &mut Self::Interpreter,
        ctx: &mut Self::Context<'_>,
        arguments: &[Value],
    ) -> Result<Value, RuntimeError>;
}

pub type FunPtr = fn(&[Value]) -> Result<Value, RuntimeError>;

#[derive(Debug, Clone)]
pub struct LoxFunction {
    declaration: Function,
    closure: Environment,
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self.declaration.name())
    }
}

impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        self.declaration == other.declaration
    }
}

impl PartialOrd for LoxFunction {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl LoxFunction {
    pub const fn new(declaration: Function, closure: Environment) -> Self {
        Self {
            declaration,
            closure,
        }
    }

    pub const fn declaration(&self) -> &Function {
        &self.declaration
    }

    pub const fn closure(&self) -> &Environment {
        &self.closure
    }

    #[must_use]
    pub fn bind(self, object: LoxObject) -> Self {
        let environment = self.closure.new_local();
        environment.define("this", Value::Object(object));
        Self {
            declaration: self.declaration,
            closure: environment,
        }
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

impl PartialOrd for NativeFunction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.as_str().partial_cmp(other.name.as_str())
    }
}

pub fn native_function(
    name: impl Into<Symbol>,
    parameters: impl IntoIterator<Item = Symbol>,
    fun_ptr: FunPtr,
) -> NativeFunction {
    NativeFunction::new(name.into(), parameters.into_iter().collect(), fun_ptr)
}

#[derive(Debug, Clone)]
pub struct LoxClass(Rc<LoxClassData>);

#[derive(Debug, Clone)]
struct LoxClassData {
    name: Symbol,
    methods: HashMap<Symbol, Value>,
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.name.as_str())
    }
}

impl PartialEq for LoxClass {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl PartialOrd for LoxClass {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.name.as_str().partial_cmp(other.0.name.as_str())
    }
}

impl LoxClass {
    pub fn new(name: Symbol, methods: HashMap<Symbol, Value>) -> Self {
        Self(Rc::new(LoxClassData { name, methods }))
    }

    pub fn name(&self) -> Symbol {
        self.0.name
    }

    pub fn methods(&self) -> &HashMap<Symbol, Value> {
        &self.0.methods
    }

    pub fn find_method(&self, name: Symbol) -> Option<&Value> {
        self.0.methods.get(&name)
    }
}

#[derive(Debug, Clone)]
pub struct LoxObject(Rc<RefCell<LoxObjectData>>);

#[derive(Debug)]
struct LoxObjectData {
    class: LoxClass,
    fields: HashMap<Symbol, Value>,
}

impl Display for LoxObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} instance", self.0.borrow().class.name().as_str())
    }
}

impl PartialEq for LoxObject {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl PartialOrd for LoxObject {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().class.partial_cmp(&other.0.borrow().class)
    }
}

impl LoxObject {
    pub fn new(class: LoxClass) -> Self {
        Self(Rc::new(RefCell::new(LoxObjectData {
            class,
            fields: HashMap::with_hasher(FxBuildHasher),
        })))
    }

    pub fn set(&self, name: Token, value: Value) {
        self.0.borrow_mut().fields.insert(name.lexeme(), value);
    }

    pub fn get(&self, name: Token) -> Option<Value> {
        self.0
            .borrow()
            .fields
            .get(&name.lexeme())
            .cloned()
            .or_else(|| self.0.borrow().class.find_method(name.lexeme()).cloned())
    }
}

#[cfg(any(test, feature = "dsl"))]
mod dsl;

#[cfg(any(test, feature = "proptest_support"))]
mod proptest_support;

#[cfg(test)]
mod tests;
